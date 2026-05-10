package storage

import (
	"bytes"
	"testing"
)

func TestConvertToWebP(t *testing.T) {
	input := []byte("fake_image_data")
	output := ConvertToWebP(input)

	if !bytes.HasSuffix(output, []byte("_webp_converted")) {
		t.Errorf("Expected output to have WebP suffix, got: %s", string(output))
	}
}
