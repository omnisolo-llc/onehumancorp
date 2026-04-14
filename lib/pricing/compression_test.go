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

func TestReduceTokens(t *testing.T) {
	original := "The quick brown fox and a lazy dog are jumping over the fence"
	reduced := ReduceTokens(original)
	expected := "quick brown fox lazy dog jumping over fence"
	if reduced != expected {
		t.Fatalf("Reduced string doesn't match expected. Got %q, expected %q", reduced, expected)
	}
}

func TestReduceTokens_Empty(t *testing.T) {
	original := "a and the or"
	reduced := ReduceTokens(original)
	expected := ""
	if reduced != expected {
		t.Fatalf("Reduced string doesn't match expected. Got %q, expected %q", reduced, expected)
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
