package harness

import (
	"reflect"
	"testing"
)

func TestScrubEnv(t *testing.T) {
	env := []string{
		"PATH=/usr/bin",
		"OHC_API_KEY_1=secret",
		"OTEL_EXPORTER_OTLP_HEADERS=bearer",
		"USER=jules",
	}
	expected := []string{
		"PATH=/usr/bin",
		"USER=jules",
	}
	got := ScrubEnv(env)
	if !reflect.DeepEqual(got, expected) {
		t.Errorf("ScrubEnv() = %v, want %v", got, expected)
	}
}
