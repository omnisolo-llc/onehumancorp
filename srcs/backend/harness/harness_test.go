package harness

import (
	"context"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/sdk/metric"
	"go.opentelemetry.io/otel/sdk/metric/metricdata"
	"go.opentelemetry.io/otel/sdk/trace"
	"go.opentelemetry.io/otel/sdk/trace/tracetest"
)

func TestHarness_FileIsolation(t *testing.T) {
	// Setup test directories
	tempDir := t.TempDir()
	readDir := filepath.Join(tempDir, "read")
	writeDir := filepath.Join(tempDir, "write")
	blockedDir := filepath.Join(tempDir, "blocked")

	os.MkdirAll(readDir, 0755)
	os.MkdirAll(writeDir, 0755)
	os.MkdirAll(blockedDir, 0755)

	testFile := filepath.Join(readDir, "test.txt")
	os.WriteFile(testFile, []byte("hello"), 0644)

	blockedFile := filepath.Join(blockedDir, "secret.txt")
	os.WriteFile(blockedFile, []byte("secret"), 0644)

	config := &SandboxConfig{
		ReadPaths:  []string{readDir},
		WritePaths: []string{writeDir},
	}

	h, err := NewHarness(config)
	if err != nil {
		t.Fatalf("Failed to create harness: %v", err)
	}

	ctx := context.Background()

	// Test 1: Should be able to read from readDir
	out, err := h.Run(ctx, "cat", []string{testFile})
	if err != nil {
		t.Fatalf("Failed to read allowed file: %v, output: %s", err, string(out))
	}
	if string(out) != "hello" {
		t.Errorf("Expected 'hello', got '%s'", string(out))
	}

	// Test 2: Should not be able to read from blockedDir
	out, err = h.Run(ctx, "cat", []string{blockedFile})
	if err == nil {
		t.Errorf("Expected error reading blocked file, but succeeded. Output: %s", string(out))
	}

	// Test 3: Should be able to write to writeDir
	testWriteFile := filepath.Join(writeDir, "out.txt")
	out, err = h.Run(ctx, "sh", []string{"-c", "echo 'world' > " + testWriteFile})
	if err != nil {
		t.Fatalf("Failed to write to allowed dir: %v, output: %s", err, string(out))
	}

	content, _ := os.ReadFile(testWriteFile)
	if strings.TrimSpace(string(content)) != "world" {
		t.Errorf("Expected 'world', got '%s'", string(content))
	}

	// Test 4: Should not be able to write to readDir
	testReadWriteFile := filepath.Join(readDir, "out.txt")
	out, err = h.Run(ctx, "sh", []string{"-c", "echo 'world' > " + testReadWriteFile})
	if err == nil {
		t.Errorf("Expected error writing to read-only dir, but succeeded. Output: %s", string(out))
	}
}

func TestHarness_NetworkProxy(t *testing.T) {
	// Create a dummy target server
	targetServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("ok"))
	}))
	defer targetServer.Close()

	// Strip http:// from targetURL
	targetHost := strings.TrimPrefix(targetServer.URL, "http://")
	targetDomain := targetHost
	if strings.Contains(targetHost, ":") {
		targetDomain = strings.Split(targetHost, ":")[0]
	}

	config := &SandboxConfig{
		AllowedDomains: []string{targetDomain},
		DeniedDomains:  []string{"example.com"},
		ReadPaths:      []string{"/tmp"}, // curl might need some paths
		WritePaths:     []string{"/tmp"},
	}

	h, err := NewHarness(config)
	if err != nil {
		t.Fatalf("Failed to create harness: %v", err)
	}

	ctx := context.Background()

	// Test 1: Allowed domain should pass
	out, err := h.Run(ctx, "curl", []string{"-s", targetServer.URL})
	if err != nil {
		t.Fatalf("Failed to curl allowed domain: %v, output: %s", err, string(out))
	}
	if string(out) != "ok" {
		t.Errorf("Expected 'ok', got '%s'", string(out))
	}

	// Test 2: Denied domain should fail
	out, err = h.Run(ctx, "curl", []string{"-s", "http://example.com"})
	if err == nil || !strings.Contains(string(out), "403 Forbidden") {
		// curl might exit with 0 if it gets a 403 response, but the output should have 403 if we don't use -f
		// actually curl -s returns the body.
		if !strings.Contains(string(out), "Forbidden") {
			t.Errorf("Expected forbidden response, got: %s (err: %v)", string(out), err)
		}
	}
}

func TestHarness_Metrics(t *testing.T) {
	// Setup OTEL metrics to memory
	reader := metric.NewManualReader()
	meterProvider := metric.NewMeterProvider(metric.WithReader(reader))
	otel.SetMeterProvider(meterProvider)

	// Setup OTEL tracing to memory
	exporter := tracetest.NewInMemoryExporter()
	tracerProvider := trace.NewTracerProvider(trace.WithSpanProcessor(trace.NewSimpleSpanProcessor(exporter)))
	otel.SetTracerProvider(tracerProvider)

	config := &SandboxConfig{
		ReadPaths:  []string{},
		WritePaths: []string{},
	}

	h, err := NewHarness(config)
	if err != nil {
		t.Fatalf("Failed to create harness: %v", err)
	}

	ctx := context.Background()
	_, _ = h.Run(ctx, "echo", []string{"hello"})

	// Verify Spans
	spans := exporter.GetSpans()
	if len(spans) == 0 {
		t.Errorf("Expected tracing spans to be recorded")
	} else {
		found := false
		for _, s := range spans {
			if s.Name == "harness.Run" {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("Expected 'harness.Run' span to be recorded")
		}
	}

	// Verify Metrics
	var rm metricdata.ResourceMetrics
	err = reader.Collect(ctx, &rm)
	if err != nil {
		t.Fatalf("Failed to collect metrics: %v", err)
	}

	foundMetric := false
	for _, sm := range rm.ScopeMetrics {
		for _, m := range sm.Metrics {
			if m.Name == "ohc_harness_executions_total" {
				foundMetric = true
				break
			}
		}
	}

	if !foundMetric {
		t.Errorf("Expected 'ohc_harness_executions_total' metric to be recorded")
	}
}
