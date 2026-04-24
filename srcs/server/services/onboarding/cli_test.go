package onboarding

import (
	"bytes"
	"context"
	"io"
	"os"
	"strings"
	"testing"
)

func TestRunCLI_Cloud(t *testing.T) {
	oldStdout := os.Stdout
	r, w, _ := os.Pipe()
	os.Stdout = w

	err := RunCLI(context.Background(), true)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	w.Close()
	os.Stdout = oldStdout

	var buf bytes.Buffer
	io.Copy(&buf, r)
	output := buf.String()

	if !strings.Contains(output, "OHC Interactive Setup (Cloud-native)") {
		t.Errorf("expected output to contain Cloud-native setup title, got %s", output)
	}
	if !strings.Contains(output, "mode: cloud") {
		t.Errorf("expected output to contain mode: cloud, got %s", output)
	}
}

func TestRunCLI_Standalone(t *testing.T) {
	oldStdout := os.Stdout
	r, w, _ := os.Pipe()
	os.Stdout = w

	err := RunCLI(context.Background(), false)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	w.Close()
	os.Stdout = oldStdout

	var buf bytes.Buffer
	io.Copy(&buf, r)
	output := buf.String()

	if !strings.Contains(output, "OHC Interactive Setup (Standalone)") {
		t.Errorf("expected output to contain Standalone setup title, got %s", output)
	}
	if !strings.Contains(output, "mode: standalone") {
		t.Errorf("expected output to contain mode: standalone, got %s", output)
	}
}
