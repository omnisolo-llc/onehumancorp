#!/usr/bin/env python3
"""Package/reuse the actual production images within one source revision.

CI builds the production Dockerfile once and both deployment suites consume
its verified archive. A local invocation without a prebuilt archive still builds
normally. Checksums detect corruption; CI artifact provenance remains required.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tarfile

ROOT = Path(__file__).resolve().parents[1]
IMAGES = ("omnisolo/server:latest", "omnisolo/agent:latest")
HEX = re.compile(r"[0-9a-f]{64}\Z")


def file_digest(path: Path) -> str:
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def source_digest() -> str:
    names = subprocess.check_output(
        ["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z"], cwd=ROOT
    ).split(b"\0")
    digest = hashlib.sha256()
    for raw in sorted(set(name for name in names if name)):
        name = os.fsdecode(raw)
        path = ROOT / name
        digest.update(raw + b"\0")
        if path.is_symlink():
            # Never dereference a source symlink outside the repository.
            digest.update(b"link\0" + os.fsencode(os.readlink(path)))
        elif path.is_file():
            digest.update(b"file\0" + file_digest(path).encode())
            digest.update(str(path.stat().st_mode & 0o111).encode())
        elif not path.exists():
            digest.update(b"deleted")
        else:
            raise ValueError(f"Unsupported source entry: {name}")
        digest.update(b"\0")
    return digest.hexdigest()


def image_info(reference: str) -> dict:
    value = json.loads(subprocess.check_output(["docker", "image", "inspect", reference], text=True))[0]
    if (value.get("Os") != "linux" or value.get("Architecture") not in ("amd64", "arm64")
            or not re.fullmatch(r"sha256:[0-9a-f]{64}", value.get("Id", ""))):
        raise ValueError("Native deployment image has an unsupported identity or platform")
    return {"reference": reference, "id": value["Id"], "os": value["Os"], "architecture": value["Architecture"]}


def validate_bundle(directory: Path, current_source: str) -> dict:
    manifest = json.loads((directory / "manifest.json").read_text())
    if (manifest.get("schema_version") != 1 or manifest.get("source_sha256") != current_source
            or not HEX.fullmatch(current_source)):
        raise ValueError("Prebuilt images do not match the current source")
    rows = manifest.get("images")
    if not isinstance(rows, list) or len(rows) != len(IMAGES):
        raise ValueError("Incomplete native image set")
    if [row.get("reference") for row in rows] != list(IMAGES):
        raise ValueError("Unexpected native image references")
    architectures = set()
    for row in rows:
        if (row.get("os") != "linux" or row.get("architecture") not in ("amd64", "arm64")
                or not re.fullmatch(r"sha256:[0-9a-f]{64}", row.get("id", ""))):
            raise ValueError("Invalid native image identity")
        architectures.add(row["architecture"])
    if len(architectures) != 1:
        raise ValueError("Native image architectures must agree")
    if manifest.get("archive_sha256") != file_digest(directory / "images.tar"):
        raise ValueError("Native image archive checksum mismatch")
    # Inspect metadata without extracting paths. Do not let a valid outer
    # checksum hide extra image tags that would replace unrelated host images.
    with tarfile.open(directory / "images.tar", "r:") as archive:
        member = archive.getmember("manifest.json")
        if not member.isfile() or member.size > 1024 * 1024:
            raise ValueError("Invalid Docker archive manifest")
        stream = archive.extractfile(member)
        if stream is None:
            raise ValueError("Docker archive manifest is missing")
        entries = json.load(stream)
        if not isinstance(entries, list) or any(not isinstance(entry, dict) for entry in entries):
            raise ValueError("Invalid Docker archive entries")
        tags = [tag for entry in entries for tag in entry.get("RepoTags", [])]
        if sorted(tags) != sorted(IMAGES):
            raise ValueError("Docker archive contains unexpected or duplicate image tags")
    return manifest


def package(directory: Path, expected_source: str) -> None:
    if not HEX.fullmatch(expected_source) or source_digest() != expected_source:
        raise ValueError("Source changed while the production images were being built")
    directory.mkdir(parents=True, exist_ok=True)
    images = [image_info(reference) for reference in IMAGES]
    subprocess.run(["docker", "save", "--output", str(directory / "images.tar"), *IMAGES], check=True)
    manifest = {"schema_version": 1, "source_sha256": expected_source,
                "archive_sha256": file_digest(directory / "images.tar"), "images": images}
    if source_digest() != expected_source:
        raise ValueError("Source changed while packaging production images")
    (directory / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    validate_bundle(directory, expected_source)


def load(directory: Path) -> None:
    manifest = validate_bundle(directory, source_digest())
    subprocess.run(["docker", "load", "--input", str(directory / "images.tar")], check=True)
    for expected in manifest["images"]:
        if image_info(expected["reference"]) != expected:
            raise ValueError("Loaded image identity differs from the source-bound manifest")
    subprocess.run(["docker", "tag", IMAGES[0], "omnisolo/mono-core:latest"], check=True)
    print("Loaded source-bound production images; no rebuild or registry publication occurred.")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=("fingerprint", "package", "load"))
    parser.add_argument("--directory", type=Path)
    parser.add_argument("--expected-source")
    args = parser.parse_args()
    try:
        if args.action == "fingerprint":
            print(source_digest())
        else:
            if args.directory is None:
                raise ValueError("An explicit artifact directory is required")
            if args.action == "package":
                if args.expected_source is None:
                    raise ValueError("Record the source fingerprint before building")
                package(args.directory, args.expected_source)
            else:
                load(args.directory)
    except (OSError, ValueError, KeyError, TypeError, tarfile.TarError, subprocess.CalledProcessError) as error:
        print(f"Native image artifact rejected: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
