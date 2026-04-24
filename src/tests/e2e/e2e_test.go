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
			filepath.Join(srcdir, "_main", "src", "server", "ohc_", "ohc"),
			filepath.Join(srcdir, "_main", "src", "server", "ohc"),
			filepath.Join(srcdir, "mono", "src", "server", "ohc_", "ohc"),
			filepath.Join(srcdir, "mono", "src", "server", "ohc"),
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

func findWebArtifacts(srcdir string) string {
	candidates := []string{
		filepath.Join(srcdir, "_main", "src", "app", "app_web.web_build_artifacts"),
		filepath.Join(srcdir, "_main", "src", "app", "app_web_build_artifacts"),
		filepath.Join(srcdir, "mono", "src", "app", "app_web.web_build_artifacts"),
		filepath.Join(srcdir, "mono", "src", "app", "app_web_build_artifacts"),
	}
	for _, p := range candidates {
		if _, err := os.Stat(p); err == nil {
			return p
		}
	}
	return ""
}

// freePort returns an available TCP port on localhost.
func freePort() int {
	// Use a random port to avoid race conditions when multiple test binaries
	// call freePort() simultaneously.
	for i := 0; i < 10; i++ {
		// Pick a random port between 20000 and 60000
		port := 20000 + (time.Now().Nanosecond() % 40000)
		l, err := net.Listen("tcp", fmt.Sprintf(":%d", port))
		if err == nil {
			l.Close()
			return port
		}
		time.Sleep(10 * time.Millisecond)
	}
	return 18080
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

		webDir := findWebArtifacts(os.Getenv("TEST_SRCDIR"))
		serverCmd = exec.Command(ohcBin)
		serverCmd.Env = append(os.Environ(),
			"OHC_STANDALONE=true",
			"OHC_HEADLESS=true",
			"OHC_SERVE_UI=true",
			fmt.Sprintf("FRONTEND_STATIC_DIR=%s", webDir),
			fmt.Sprintf("PORT=%d", port),
			fmt.Sprintf("STATE_DIR=%s", stateDir),
			fmt.Sprintf("OHC_RUNTIME_DIR=%s", stateDir),
			"REDIS_URL=",
			fmt.Sprintf("DATABASE_URL=sqlite://%s/ohc_state.db", stateDir),
			// Point the OHC server at the in-process fake LLM so tests are
			// deterministic and do not require a live AI API key.
			fmt.Sprintf("OHC_LOCAL_LLM_ENDPOINT=%s/api/chat", llmURL),
			fmt.Sprintf("OHC_LOCAL_LLM_EMBED_ENDPOINT=%s/api/embeddings", llmURL),
			"OHC_LLM_PROVIDER=ollama",
			fmt.Sprintf("GRPC_PORT=%d", freePort()),
		)
		serverCmd.Stdout = os.Stdout
		serverCmd.Stderr = os.Stderr
		if err := serverCmd.Start(); err != nil {
			fmt.Fprintf(os.Stderr, "e2e: start ohc: %v\n", err)
			os.Exit(1)
		}

		// Wait up to 120s for the server to be ready.
		// 120s (instead of 60s) is necessary because up to 4 test binaries
		// run in parallel (--local_test_jobs=4), each launching an OHC
		// process; on resource-constrained CI hosts startup can be slow.
		deadline := time.Now().Add(120 * time.Second)
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
	// Skip host-requirement validation so tests are hermetic and pass even
	// when the host is missing optional system libraries (e.g. in CI containers).
	// Failure is non-fatal: browser-based tests will be skipped automatically
	// via newPage() which calls t.Skip() when bCtx is nil.
	os.Setenv("PLAYWRIGHT_SKIP_VALIDATE_HOST_REQUIREMENTS", "1")
	playwrightReady := true
	if os.Getenv("PLAYWRIGHT_SKIP_INSTALL") == "" {
		if installErr := playwright.Install(); installErr != nil {
			fmt.Fprintf(os.Stderr, "playwright install: %v (browser tests will be skipped)\n", installErr)
			playwrightReady = false
		}
	} else {
		fmt.Fprintln(os.Stdout, "playwright install skipped via PLAYWRIGHT_SKIP_INSTALL")
	}

	var err error
	if playwrightReady {
		pw, err = playwright.Run()
		if err != nil {
			fmt.Fprintf(os.Stderr, "playwright run: %v (browser tests will be skipped)\n", err)
			playwrightReady = false
			err = nil // reset so browser launch path does not inherit this error
		}
	}

	if !playwrightReady {
		// Browser unavailable: run tests in API-only mode.
		// All tests that call newPage() will be skipped automatically.
		browser = nil
		bCtx = nil
		code := m.Run()
		if serverCmd != nil {
			serverCmd.Process.Kill()
		}
		os.Exit(code)
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
		fmt.Fprintf(os.Stderr, "browser context: %v (browser tests will be skipped)\n", err)
		browser = nil
		bCtx = nil
		code := m.Run()
		if serverCmd != nil {
			serverCmd.Process.Kill()
		}
		os.Exit(code)
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
