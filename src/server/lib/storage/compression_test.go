package storage

import (
	"bytes"
	"image"
	"image/color"
	"image/png"
	"testing"
)

func TestQuotaManager(t *testing.T) {
	qm := NewQuotaManager()

	qm.SetTier("t1", "free")
	used, limit, exceeded := qm.RecordUsage("t1", 500*1024*1024)
	if exceeded {
		t.Errorf("expected not exceeded on exactly 500MB, got used=%d, limit=%d", used, limit)
	}

	used, _, exceeded = qm.RecordUsage("t1", 1)
	if !exceeded {
		t.Errorf("expected exceeded after going over 500MB")
	}
}

func TestOptimizeImage(t *testing.T) {
	// Create a dummy 10x10 PNG
	img := image.NewRGBA(image.Rect(0, 0, 10, 10))
	for x := 0; x < 10; x++ {
		for y := 0; y < 10; y++ {
			img.Set(x, y, color.RGBA{R: 255, A: 255})
		}
	}
	var buf bytes.Buffer
	err := png.Encode(&buf, img)
	if err != nil {
		t.Fatalf("failed to encode dummy png: %v", err)
	}

	optimized, err := OptimizeImage(buf.Bytes(), "png")
	if err != nil {
		t.Fatalf("OptimizeImage failed: %v", err)
	}

	if len(optimized) == 0 {
		t.Errorf("expected non-empty optimized image")
	}

	// Should handle unknown gracefully
	passthrough, err := OptimizeImage([]byte("not an image"), "unknown")
	if err != nil {
		t.Fatalf("OptimizeImage failed on unknown: %v", err)
	}
	if string(passthrough) != "not an image" {
		t.Errorf("expected passthrough for unknown format")
	}
}
