"""Archive/provenance unit tests; Docker calls are explicit boundary doubles."""
import copy
import importlib.util
import io
import json
from pathlib import Path
import tarfile
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location("native_images", Path(__file__).with_name("native-images.py"))
images = importlib.util.module_from_spec(spec)
spec.loader.exec_module(images)


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
