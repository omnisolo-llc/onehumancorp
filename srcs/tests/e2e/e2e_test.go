package e2e

import (
	"fmt"
	"net"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"testing"
	"time"

	playwright "github.com/playwright-community/playwright-go"
)

var (
	pw      *playwright.Playwright
	browser playwright.Browser
	bCtx    playwright.BrowserContext
	baseURL = getEnvOr("OHC_E2E_BASE_URL", "")
)

// findOhcBinary locates the ohc server binary from Bazel runfiles or PATH.
func findOhcBinary() string {
	if srcdir := os.Getenv("TEST_SRCDIR"); srcdir != "" {
		candidates := []string{
			filepath.Join(srcdir, "_main", "srcs", "server", "ohc_", "ohc"),
			filepath.Join(srcdir, "_main", "srcs", "server", "ohc"),
			filepath.Join(srcdir, "mono", "srcs", "server", "ohc_", "ohc"),
			filepath.Join(srcdir, "mono", "srcs", "server", "ohc"),
		}
		for _, p := range candidates {
			if _, err := os.Stat(p); err == nil {
				return p
			}
		}
	}
	if p, err := exec.LookPath("ohc"); err == nil {
		return p
	}
	return ""
}

// freePort returns an available TCP port on localhost.
func freePort() int {
	l, err := net.Listen("tcp", ":0")
	if err != nil {
		return 18080
	}
	defer l.Close()
	return l.Addr().(*net.TCPAddr).Port
}

func TestMain(m *testing.M) {
	var serverCmd *exec.Cmd

	// If no external server provided, start the OHC binary in standalone mode.
	if baseURL == "" {
		ohcBin := findOhcBinary()
		if ohcBin == "" {
			fmt.Fprintln(os.Stderr, "e2e: ohc binary not found in runfiles; set OHC_E2E_BASE_URL to use an existing server")
			os.Exit(1)
		}

		port := freePort()
		baseURL = fmt.Sprintf("http://localhost:%d", port)

		stateDir, err := os.MkdirTemp("", "ohc-e2e-state-*")
		if err != nil {
			fmt.Fprintf(os.Stderr, "e2e: mkdirtemp: %v\n", err)
			os.Exit(1)
		}
		defer os.RemoveAll(stateDir)

		// Start a fake LLM server so E2E tests do not depend on external AI
		// API availability. The OHC binary is configured to call this server
		// instead of a real LLM provider.
		llmURL := startFakeLLM()

		serverCmd = exec.Command(ohcBin)
		serverCmd.Env = append(os.Environ(),
			"OHC_STANDALONE=true",
			"OHC_HEADLESS=true",
			"OHC_SERVE_UI=false",
			fmt.Sprintf("PORT=%d", port),
			fmt.Sprintf("STATE_DIR=%s", stateDir),
			"REDIS_URL=",
			"DATABASE_URL=",
			// Point the OHC server at the in-process fake LLM so tests are
			// deterministic and do not require a live AI API key.
			fmt.Sprintf("OHC_LOCAL_LLM_ENDPOINT=%s/api/chat", llmURL),
			fmt.Sprintf("OHC_LOCAL_LLM_EMBED_ENDPOINT=%s/api/embeddings", llmURL),
			"OHC_LLM_PROVIDER=ollama",
		)
		serverCmd.Stdout = os.Stdout
		serverCmd.Stderr = os.Stderr
		if err := serverCmd.Start(); err != nil {
			fmt.Fprintf(os.Stderr, "e2e: start ohc: %v\n", err)
			os.Exit(1)
		}

		// Wait up to 60s for the server to be ready.
		deadline := time.Now().Add(60 * time.Second)
		ready := false
		for time.Now().Before(deadline) {
			resp, err := http.Get(baseURL + "/healthz")
			if err == nil && resp.StatusCode < 500 {
				resp.Body.Close()
				ready = true
				break
			}
			if resp != nil {
				resp.Body.Close()
			}
			time.Sleep(500 * time.Millisecond)
		}
		if !ready {
			fmt.Fprintln(os.Stderr, "e2e: server did not become ready in time")
			serverCmd.Process.Kill()
			os.Exit(1)
		}
	}

	// Install playwright browsers (no-op if already installed).
	// This downloads browsers to ~/.cache/ms-playwright by default.
	if err := playwright.Install(); err != nil {
		fmt.Fprintf(os.Stderr, "playwright install: %v\n", err)
		if serverCmd != nil {
			serverCmd.Process.Kill()
		}
		os.Exit(1)
	}

	var err error
	pw, err = playwright.Run()
	if err != nil {
		fmt.Fprintf(os.Stderr, "playwright run: %v\n", err)
		if serverCmd != nil {
			serverCmd.Process.Kill()
		}
		os.Exit(1)
	}

	// Use Firefox instead of Chromium for better cross-platform compatibility
	// Firefox has fewer system library dependencies than Chromium/GTK.
	// The hermetic playwright.Install() downloads Firefox with bundled dependencies.
	browser, err = pw.Firefox.Launch(playwright.BrowserTypeLaunchOptions{
		Headless: playwright.Bool(true),
	})
	if err != nil {
		// If Firefox fails, try Chromium with sandbox disabled for CI environments
		fmt.Fprintf(os.Stderr, "firefox launch: %v, trying chromium with no-sandbox\n", err)
		browser, err = pw.Chromium.Launch(playwright.BrowserTypeLaunchOptions{
			Headless: playwright.Bool(true),
			Args: []string{
				"--no-sandbox",
				"--disable-setuid-sandbox",
				"--disable-dev-shm-usage",
				"--disable-gpu",
			},
		})
		if err != nil {
			// If browser launch fails entirely, continue without browser
			// Tests that need browser will be skipped
			fmt.Fprintf(os.Stderr, "browser launch failed (tests requiring browser will be skipped): %v\n", err)
			browser = nil
			bCtx = nil
			code := m.Run()
			if serverCmd != nil {
				serverCmd.Process.Kill()
			}
			os.Exit(code)
		}
	}

	bCtx, err = browser.NewContext(playwright.BrowserNewContextOptions{
		BaseURL: playwright.String(baseURL),
	})
	if err != nil {
		fmt.Fprintf(os.Stderr, "browser context: %v\n", err)
		if serverCmd != nil {
			serverCmd.Process.Kill()
		}
		os.Exit(1)
	}

	code := m.Run()

	_ = bCtx.Close()
	_ = browser.Close()
	_ = pw.Stop()

	if serverCmd != nil {
		serverCmd.Process.Kill()
	}

	os.Exit(code)
}
