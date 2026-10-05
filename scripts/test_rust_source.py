"""Real parser/consumer parity; run after cargo build -p ohc-rust-source-extract."""
import os
from pathlib import Path
import subprocess
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parent))
import rust_source

ROOT = Path(__file__).resolve().parents[1]
# Explicit test input only; production preparation always builds through Cargo.
BINARY = Path(os.environ["OHC_RUST_EXTRACT_TEST_BINARY"])


class ExactRustAdapter(unittest.TestCase):
    def test_comments_and_character_braces_preserve_original_source(self):
        for body in ("// } comment\n    assert!(true);", "let brace = '}';\n    assert!(true);"):
            item = "fn sample() {\n    " + body + "\n}"
            self.assertEqual(rust_source._checked_extract(BINARY, item, "function", "sample"), item)

    def test_outer_attributes_and_crlf_are_exact(self):
        item = '#[cfg(test)]\r\n/// Café\r\nfn sample() {}'
        self.assertEqual(rust_source._checked_extract(BINARY, item, "function", "sample"), item)

    def test_inherent_method_identity_and_attributes_are_preserved(self):
        method = "#[inline] fn sample(&self) { let brace = '}'; }"
        source = f"impl Item {{ {method} }} impl Other {{ fn sample(&self) {{}} }}"
        self.assertEqual(rust_source._checked_extract(BINARY, source, "function", "sample", impl_type="Item"), method)

    def test_enclosing_impl_attributes_cannot_be_silently_discarded(self):
        with self.assertRaisesRegex(RuntimeError, "enclosing impl changed"):
            rust_source._checked_extract(BINARY, '#[cfg(test)] impl Item { fn sample() {} }', "function", "sample", impl_type="Item")

    def test_syntax_errors_and_ambiguity_do_not_fall_back_to_scanning(self):
        for source in ("fn sample() {} fn sample() {}", "fn sample() { // }\n"):
            with self.assertRaises(subprocess.CalledProcessError):
                rust_source._checked_extract(BINARY, source, "function", "sample")

    def test_parser_inputs_include_locked_dependencies_and_all_implementation(self):
        names = {str(path.relative_to(ROOT)) for path in rust_source.input_paths()}
        self.assertTrue({"Cargo.toml", "Cargo.lock", "scripts/rust_source.py", "scripts/rust-source-extract/Cargo.toml", "scripts/rust-source-extract/src/lib.rs", "scripts/rust-source-extract/src/main.rs"} <= names)


if __name__ == "__main__":
    unittest.main()
