"""Archive/provenance unit tests; Docker calls are explicit boundary doubles."""
import copy
import importlib.util
import io
import json
from pathlib import Path
import re
import shlex
import tarfile
import tempfile
import tomllib
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location("native_images", Path(__file__).with_name("native-images.py"))
images = importlib.util.module_from_spec(spec)
spec.loader.exec_module(images)


class RuntimeBuildContractTests(unittest.TestCase):
    ROOT = Path(__file__).resolve().parents[1]
    RUNTIME_BINS = {'server', 'omnisolo-builtin-agent', 'omnisolo-harness-worker'}
    PACKAGES = {'omnisolo', 'omnisolo_builtin_agent', 'omnisolo_harness_worker'}

    def dockerfile(self):
        return (self.ROOT / 'deploy/docker/server/Dockerfile').read_text()

    def assert_runtime_targets(self, source):
        commands = [shlex.split(line) for line in source.replace('\\\n', ' ').splitlines()
                    if line.startswith('RUN cargo chef cook ') or line.startswith('RUN cargo build ')]
        self.assertEqual(len(commands), 2, 'retain the dependency cook and actual source build')
        for command, prefix in zip(commands, (['RUN', 'cargo', 'chef', 'cook'], ['RUN', 'cargo', 'build'])):
            self.assertEqual(command[:len(prefix)], prefix)
            flags = command[len(prefix):]
            self.assertEqual(flags.count('--release'), 1)
            self.assertEqual(flags.count('--locked'), 1)
            binaries, packages = [], []
            while flags:
                flag = flags.pop(0)
                if flag in ('--release', '--locked'):
                    continue
                self.assertIn(flag, ('--bin', '--package', '-p', '--recipe-path'),
                              'do not widen targets or alter production features/profile')
                self.assertTrue(flags, f'missing value for {flag}')
                value = flags.pop(0)
                if flag == '--bin':
                    binaries.append(value)
                elif flag in ('--package', '-p'):
                    packages.append(value)
                else:
                    self.assertIn('chef', prefix)
                    self.assertEqual(value, 'recipe.json')
            self.assertEqual(set(binaries), self.RUNTIME_BINS)
            self.assertEqual(len(binaries), len(self.RUNTIME_BINS), 'no duplicate binary selection')
            self.assertEqual(set(packages), self.PACKAGES)
            self.assertEqual(len(packages), len(self.PACKAGES), 'no duplicate package selection')
        copied = re.findall(r'^COPY --from=builder /src/target/release/(\S+) /usr/local/bin/(\S+)$', source, re.M)
        self.assertEqual({src for src, _ in copied}, self.RUNTIME_BINS)
        self.assertTrue(all(src == dst for src, dst in copied))
        for role in ('server', 'agent', 'worker'):
            self.assertIn(f'FROM runtime AS {role}\n', source)

    def test_cook_and_build_select_only_the_same_complete_runtime_binaries(self):
        self.assert_runtime_targets(self.dockerfile())

    def test_runtime_target_guard_rejects_widening_omissions_and_profile_changes(self):
        source = self.dockerfile()
        before_builder, builder = source.split('FROM chef AS builder', 1)
        for old, new in (
            ('--bin server', '--bins'),
            ('--bin omnisolo-builtin-agent', '--bin benchmark'),
            ('--bin omnisolo-harness-worker', '--bin grpc-mtls-probe'),
            ('--bin server', '--bin server --bin server'),
            ('--release', ''),
            ('--locked', ''),
            ('--bin server', '--no-default-features --bin server'),
        ):
            for occurrence in (0, 1):
                with self.subTest(old=old, new=new, occurrence=occurrence):
                    parts = builder.split(old)
                    self.assertEqual(len(parts), 3, 'both cook and build must declare the same selection')
                    replacement = (new if occurrence == 0 else old).join(parts[:2])
                    replacement += (new if occurrence == 1 else old) + parts[2]
                    with self.assertRaises(AssertionError):
                        self.assert_runtime_targets(before_builder + 'FROM chef AS builder' + replacement)

    def test_runtime_targets_match_manifests_and_auxiliary_binaries_remain_tested(self):
        paths = ('Cargo.toml', 'src/agents/builtin/Cargo.toml', 'src/server/harness_worker/Cargo.toml')
        manifests = [tomllib.loads((self.ROOT / path).read_text()) for path in paths]
        self.assertEqual({value['package']['name'] for value in manifests}, self.PACKAGES)
        binaries = {target['name'] for value in manifests for target in value.get('bin', [])}
        self.assertEqual(binaries, self.RUNTIME_BINS | {'benchmark', 'grpc-mtls-probe'})
        workflow = (self.ROOT / '.github/workflows/ci.yml').read_text()
        native = workflow.split('  native-build:', 1)[1].split('  native-test:', 1)[0]
        self.assertIn('cargo build --locked -p omnisolo -p omnisolo_builtin_agent -p omnisolo_harness_worker --bins --timings', native)
        self.assertIn('target/debug/grpc-mtls-probe', native)
        self.assertEqual(workflow.count('OMNISOLO_GRPC_PROBE: target/debug/grpc-mtls-probe'), 2)
        makefile = (self.ROOT / 'Makefile').read_text()
        self.assertIn('$(CARGO) test --locked --workspace --exclude app $(RUST_TEST_ARGS)', makefile)


class ImageArtifactTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.directory = Path(self.temp.name)
        self.source = "a" * 64
        self.rows = [{"reference": ref, "id": "sha256:" + str(i + 1) * 64,
                      "os": "linux", "architecture": "amd64"} for i, ref in enumerate(images.IMAGES)]
        self.archive()

    def archive(self, tags=None):
        entries = [{"Config": "fixture.json", "Layers": [], "RepoTags": [tag]}
                   for tag in (tags if tags is not None else images.IMAGES)]
        body = json.dumps(entries).encode()
        with tarfile.open(self.directory / "images.tar", "w") as archive:
            member = tarfile.TarInfo("manifest.json")
            member.size = len(body)
            archive.addfile(member, io.BytesIO(body))
        self.manifest = {"schema_version": 1, "source_sha256": self.source,
                         "archive_sha256": images.file_digest(self.directory / "images.tar"),
                         "images": copy.deepcopy(self.rows)}
        self.write_manifest()

    def write_manifest(self):
        (self.directory / "manifest.json").write_text(json.dumps(self.manifest))

    def test_matching_source_and_archive_are_accepted(self):
        self.assertEqual(images.validate_bundle(self.directory, self.source)["images"], self.rows)

    def test_foreign_source_is_not_reused(self):
        with self.assertRaises(ValueError):
            images.validate_bundle(self.directory, "b" * 64)

    def test_corrupted_archive_fails_before_docker(self):
        (self.directory / "images.tar").write_bytes(b"corrupt")
        with patch.object(images, "source_digest", return_value=self.source), patch.object(images.subprocess, "run") as docker:
            with self.assertRaises(ValueError):
                images.load(self.directory)
            docker.assert_not_called()

    def test_missing_agent_is_rejected(self):
        self.manifest["images"].pop()
        self.write_manifest()
        with self.assertRaises(ValueError):
            images.validate_bundle(self.directory, self.source)

    def test_mixed_architectures_are_rejected(self):
        self.manifest["images"][1]["architecture"] = "arm64"
        self.write_manifest()
        with self.assertRaises(ValueError):
            images.validate_bundle(self.directory, self.source)

    def test_unrelated_tags_are_rejected_even_with_a_matching_checksum(self):
        self.archive([*images.IMAGES, "unrelated/database:latest"])
        with self.assertRaises(ValueError):
            images.validate_bundle(self.directory, self.source)

    def test_duplicate_tags_are_rejected(self):
        self.archive([images.IMAGES[0], images.IMAGES[0]])
        with self.assertRaises(ValueError):
            images.validate_bundle(self.directory, self.source)

    def test_loaded_ids_must_match_before_compatibility_tag_is_created(self):
        wrong = {**self.rows[0], "id": "sha256:" + "f" * 64}
        with patch.object(images, "source_digest", return_value=self.source), patch.object(images, "image_info", return_value=wrong), patch.object(images.subprocess, "run") as docker:
            with self.assertRaises(ValueError):
                images.load(self.directory)
            self.assertEqual(docker.call_count, 1)
            self.assertEqual(docker.call_args.args[0][:2], ["docker", "load"])


if __name__ == "__main__":
    unittest.main()
