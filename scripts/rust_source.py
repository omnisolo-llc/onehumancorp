"""Build the locked Rust parser and copy only its verified original byte ranges."""
from functools import lru_cache
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def input_paths(root=ROOT):
    crate = root / "scripts/rust-source-extract"
    return tuple(sorted({
        root / "Cargo.toml", root / "Cargo.lock", root / "scripts/rust_source.py",
        crate / "Cargo.toml", *crate.rglob("*.rs"),
    }))


def _fingerprints(paths):
    return {path: hashlib.sha256(path.read_bytes()).hexdigest() for path in paths}


@lru_cache(maxsize=1)
def _build():
    inputs = _fingerprints(input_paths())
    result = subprocess.run([
        "cargo", "build", "--locked", "--offline", "--manifest-path", str(ROOT / "Cargo.toml"),
        "-p", "ohc-rust-source-extract", "--message-format=json",
    ], stdout=subprocess.PIPE, text=True, check=True)
    artifacts = []
    for line in result.stdout.splitlines():
        message = json.loads(line)
        if (message.get("reason") == "compiler-artifact"
                and message.get("target", {}).get("name") == "ohc-rust-source-extract"
                and message.get("target", {}).get("kind") == ["bin"]
                and Path(message.get("target", {}).get("src_path", "")).resolve() == ROOT / "scripts/rust-source-extract/src/main.rs"
                and message.get("executable")):
            artifacts.append(Path(message["executable"]))
    if len(artifacts) != 1 or _fingerprints(input_paths()) != inputs:
        raise RuntimeError("Rust parser build did not produce one unchanged source-bound executable")
    return artifacts[0], inputs


def _checked_extract(executable, source, kind, name, *, impl_type=None, impl_trait=None):
    original = source.encode("utf-8")
    source_sha256 = hashlib.sha256(original).hexdigest()
    with tempfile.TemporaryDirectory(prefix="ohc-rust-source-") as directory:
        path = Path(directory) / "source.rs"
        path.write_bytes(original)
        command = [str(executable), str(path), kind, name, "--expect-source-sha256", source_sha256]
        if impl_type is not None:
            command += ["--impl-type", impl_type]
        if impl_trait is not None:
            command += ["--impl-trait", impl_trait]
        result = subprocess.run(command, capture_output=True, check=True, text=True)
    proof = json.loads(result.stdout)
    start, end = proof["start"], proof["end"]
    if type(start) is not int or type(end) is not int or not 0 <= start < end <= len(original):
        raise RuntimeError("Rust parser returned invalid byte offsets")
    fragment = original[start:end]
    if (proof["source_sha256"] != source_sha256
            or proof["selection_sha256"] != hashlib.sha256(fragment).hexdigest()
            or proof["text"].encode("utf-8") != fragment):
        raise RuntimeError("Rust parser fragment is not bound to the original source bytes")
    enclosing = proof["enclosing"]
    if impl_type is None:
        if enclosing:
            raise RuntimeError("top-level selection unexpectedly has enclosing scope")
    else:
        # These three consumers reconstruct a plain inherent impl around selected
        # methods. New impl attrs/generics/scope require an explicit caller review.
        if impl_trait is not None or len(enclosing) != 1:
            raise RuntimeError("method extraction requires one plain inherent impl")
        scope = enclosing[0]
        if (type(scope["start"]) is not int or type(scope["end"]) is not int
                or not 0 <= scope["start"] <= start < end <= scope["end"] <= len(original)):
            raise RuntimeError("Rust parser returned invalid enclosing byte offsets")
        context = original[scope["start"]:scope["end"]]
        if (hashlib.sha256(context).hexdigest() != scope["sha256"]
                or not context.startswith(f"impl {impl_type} {{".encode())):
            raise RuntimeError("enclosing impl changed; preserve its attributes and identity explicitly")
    return fragment.decode("utf-8")


def extract_item(source, kind, name, *, impl_type=None, impl_trait=None):
    executable, inputs = _build()
    if _fingerprints(input_paths()) != inputs:
        raise RuntimeError("Rust parser inputs changed during source preparation")
    fragment = _checked_extract(executable, source, kind, name,
                                impl_type=impl_type, impl_trait=impl_trait)
    if _fingerprints(input_paths()) != inputs:
        raise RuntimeError("Rust parser inputs changed during source preparation")
    return fragment
