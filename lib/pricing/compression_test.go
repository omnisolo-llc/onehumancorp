package pricing

import (
	"strings"
	"testing"
)

func TestCompressionLossless(t *testing.T) {
	original := strings.Repeat("This is a test string to compress. ", 10)
	compressed, err := CompressLossless(original)
	if err != nil {
		t.Fatalf("Failed to compress: %v", err)
	}

	if compressed == original {
		t.Fatalf("Compressed string is same as original")
	}

	decompressed, err := DecompressLossless(compressed)
	if err != nil {
		t.Fatalf("Failed to decompress: %v", err)
	}

	if decompressed != original {
		t.Fatalf("Decompressed string doesn't match original. Got %q, expected %q", decompressed, original)
	}
}

func TestDecompressLossless_NotCompressed(t *testing.T) {
	original := "Not compressed"
	decompressed, err := DecompressLossless(original)
	if err != nil {
		t.Fatalf("Failed to decompress: %v", err)
	}

	if decompressed != original {
		t.Fatalf("Decompressed string doesn't match original. Got %q, expected %q", decompressed, original)
	}
}
