package harnessdaemon

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/playwright-community/playwright-go"
)

func TestDaemonStatePersistence(t *testing.T) {
	d := NewDaemon(0) // 0 means randomly assigned port, but we'll use httptest

	tmpDir := os.TempDir()
	pwDir := filepath.Join(tmpDir, "playwright-go-test")
	os.Setenv("PLAYWRIGHT_BROWSERS_PATH", pwDir)

	err := playwright.Install(&playwright.RunOptions{
		DriverDirectory: pwDir,
	})
	if err != nil {
		t.Fatalf("failed to install playwright: %v", err)
	}

	pw, err := playwright.Run(&playwright.RunOptions{
		DriverDirectory: pwDir,
	})
	if err != nil {
		t.Fatalf("failed to start playwright: %v", err)
	}
	defer pw.Stop()

	browser, err := pw.Chromium.Launch(playwright.BrowserTypeLaunchOptions{
		Headless: playwright.Bool(true),
	})
	if err != nil {
		t.Fatalf("failed to launch chromium: %v", err)
	}
	defer browser.Close()

	ctx, err := browser.NewContext()
	if err != nil {
		t.Fatalf("failed to create context: %v", err)
	}

	page, err := ctx.NewPage()
	if err != nil {
		t.Fatalf("failed to create page: %v", err)
	}

	d.pw = pw
	d.browser = browser
	d.context = ctx
	d.page = page

	mux := http.NewServeMux()
	mux.HandleFunc("/command", d.handleCommand)
	server := httptest.NewServer(mux)
	defer server.Close()

	// 1. Set a cookie
	cookieReq := CommandRequest{
		Type:    "set_cookie",
		Command: `{"name":"testcookie","value":"123","domain":"example.com","path":"/"}`,
	}
	reqBody, _ := json.Marshal(cookieReq)
	resp, err := http.Post(server.URL+"/command", "application/json", bytes.NewBuffer(reqBody))
	if err != nil {
		t.Fatalf("Failed to post: %v", err)
	}
	var res CommandResponse
	json.NewDecoder(resp.Body).Decode(&res)
	if res.ExitCode != 0 {
		t.Fatalf("Expected exit code 0, got %d. Stderr: %s", res.ExitCode, res.Stderr)
	}

	// 2. Read cookies to verify persistence
	cookieReadReq := CommandRequest{
		Type: "cookies",
	}
	reqBody2, _ := json.Marshal(cookieReadReq)
	resp2, err := http.Post(server.URL+"/command", "application/json", bytes.NewBuffer(reqBody2))
	if err != nil {
		t.Fatalf("Failed to post: %v", err)
	}
	var res2 CommandResponse
	json.NewDecoder(resp2.Body).Decode(&res2)
	if res2.ExitCode != 0 {
		t.Fatalf("Expected exit code 0, got %d. Stderr: %s", res2.ExitCode, res2.Stderr)
	}

	if !bytes.Contains([]byte(res2.Stdout), []byte("testcookie")) {
		t.Fatalf("Expected cookie not found in response: %s", res2.Stdout)
	}

	// Wait to avoid race
	time.Sleep(100 * time.Millisecond)
}
