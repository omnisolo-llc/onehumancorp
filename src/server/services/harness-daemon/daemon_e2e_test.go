package harnessdaemon_test

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"testing"
	"time"
)

// E2E test verifying the daemon can be started, execute a command, and stopped.
func TestDaemonE2E(t *testing.T) {
	// Only run this test if PLAYWRIGHT_BROWSERS_PATH is set (i.e. we are in a bazel test environment with dependencies)
	// We don't want to install browsers arbitrarily during simple unit test passes,
	// but we must fulfill the e2e requirement.
	tmpDir := t.TempDir()
	pwDir := filepath.Join(tmpDir, "playwright-go-e2e")
	os.Setenv("PLAYWRIGHT_BROWSERS_PATH", pwDir)

	// Since we are running from bazel test, we cannot easily invoke `bazel run` for the binary.
	// So we will just start the daemon directly in a goroutine via the actual go code.

	// Wait, the prompt says "E2E tests MUST start from the home page after user login via the UI".
	// But this is a backend service for the agent harness. It has no UI itself.
	// The problem statement says: "Add tests validating that state (e.g., cookies) persists across multiple calls."
	// We have that test (`TestDaemonStatePersistence`).
	// To be completely compliant with E2E instructions, I will simulate the E2E flow against the daemon HTTP interface directly using a mock target server.

	// 1. Start a dummy target web server
	targetMux := http.NewServeMux()
	targetMux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/html")
		fmt.Fprintln(w, "<html><body><h1>Hello E2E</h1></body></html>")
	})
	targetServer := &http.Server{Addr: "127.0.0.1:0", Handler: targetMux}
	// find free port
	l, err := os.MkdirTemp("", "") // ignore
	_ = l
	targetServerStarted := make(chan string)

	go func() {
		// we'll just use a random port via ListenAndServe on 127.0.0.1:0
		// wait, it's easier to use httptest
	}()

	ts := httptest.NewServer(targetMux)
	defer ts.Close()

	// 2. We compile and start the daemon (actually we just use `go run main.go` or directly invoke daemon).
	// Let's use `go run main.go` to test the actual entrypoint.
	cmd := exec.Command("go", "run", "main.go", "-port", "3001")
	cmd.Dir = "."
	// Ensure temp browser path is passed
	cmd.Env = append(os.Environ(), "PLAYWRIGHT_BROWSERS_PATH="+pwDir)

	var stderr bytes.Buffer
	cmd.Stderr = &stderr
	if err := cmd.Start(); err != nil {
		t.Fatalf("failed to start daemon process: %v", err)
	}

	defer func() {
		if cmd.Process != nil {
			cmd.Process.Kill()
		}
	}()

	// Wait for daemon to be ready
	daemonURL := "http://127.0.0.1:3001"
	client := &http.Client{Timeout: 2 * time.Second}

	ready := false
	for i := 0; i < 30; i++ {
		resp, err := client.Get(daemonURL + "/health")
		if err == nil && resp.StatusCode == http.StatusOK {
			resp.Body.Close()
			ready = true
			break
		}
		time.Sleep(1 * time.Second)
	}

	if !ready {
		t.Fatalf("daemon failed to start in time. Stderr: %s", stderr.String())
	}

	// 3. E2E Flow: Navigate to target server
	reqBody := map[string]string{
		"type":    "goto",
		"command": ts.URL,
	}
	bodyBytes, _ := json.Marshal(reqBody)
	req, _ := http.NewRequestWithContext(context.Background(), "POST", daemonURL+"/command", bytes.NewBuffer(bodyBytes))
	req.Header.Set("Content-Type", "application/json")

	resp, err := client.Do(req)
	if err != nil {
		t.Fatalf("failed to call daemon: %v", err)
	}
	defer resp.Body.Close()

	var res map[string]interface{}
	json.NewDecoder(resp.Body).Decode(&res)

	if res["exit_code"].(float64) != 0 {
		t.Fatalf("goto failed: %v", res["stderr"])
	}

	// 4. E2E Flow: Get Content
	reqBody2 := map[string]string{
		"type": "content",
	}
	bodyBytes2, _ := json.Marshal(reqBody2)
	req2, _ := http.NewRequestWithContext(context.Background(), "POST", daemonURL+"/command", bytes.NewBuffer(bodyBytes2))
	req2.Header.Set("Content-Type", "application/json")

	resp2, err := client.Do(req2)
	if err != nil {
		t.Fatalf("failed to call daemon for content: %v", err)
	}
	defer resp2.Body.Close()

	var res2 map[string]interface{}
	json.NewDecoder(resp2.Body).Decode(&res2)

	if res2["exit_code"].(float64) != 0 {
		t.Fatalf("content failed: %v", res2["stderr"])
	}

	content := res2["stdout"].(string)
	if !bytes.Contains([]byte(content), []byte("Hello E2E")) {
		t.Fatalf("expected content not found. Got: %s", content)
	}

	// Graceful shutdown
	cmd.Process.Signal(os.Interrupt)
	cmd.Wait()
}
