package e2e

import (
	"fmt"
	"net/http"
	"os"
	"os/exec"
	"testing"
	"time"

	playwright "github.com/playwright-community/playwright-go"
)

var (
	pw      *playwright.Playwright
	browser playwright.Browser
	bCtx    playwright.BrowserContext
	baseURL = getEnvOr("OHC_E2E_BASE_URL", "http://localhost:8080")
)

func TestMain(m *testing.M) {
	// Start docker-compose stack
	compose := exec.Command("docker", "compose", "-f", "../../../../deploy/docker-compose.yml", "up", "-d")
	compose.Stdout = os.Stdout
	compose.Stderr = os.Stderr
	_ = compose.Run()

	// Wait up to 60s for stack to be ready
	deadline := time.Now().Add(60 * time.Second)
	for time.Now().Before(deadline) {
		resp, err := http.Get(baseURL + "/health")
		if err == nil && resp.StatusCode < 500 {
			resp.Body.Close()
			break
		}
		if resp != nil {
			resp.Body.Close()
		}
		time.Sleep(2 * time.Second)
	}

	// Install playwright browsers
	if err := playwright.Install(); err != nil {
		fmt.Fprintf(os.Stderr, "playwright install: %v\n", err)
		os.Exit(1)
	}

	var err error
	pw, err = playwright.Run()
	if err != nil {
		fmt.Fprintf(os.Stderr, "playwright run: %v\n", err)
		os.Exit(1)
	}

	browser, err = pw.Chromium.Launch()
	if err != nil {
		fmt.Fprintf(os.Stderr, "browser launch: %v\n", err)
		os.Exit(1)
	}

	bCtx, err = browser.NewContext(playwright.BrowserNewContextOptions{
		BaseURL: playwright.String(baseURL),
	})
	if err != nil {
		fmt.Fprintf(os.Stderr, "browser context: %v\n", err)
		os.Exit(1)
	}

	code := m.Run()

	_ = bCtx.Close()
	_ = browser.Close()
	_ = pw.Stop()

	// Tear down docker-compose
	down := exec.Command("docker", "compose", "-f", "../../../../deploy/docker-compose.yml", "down")
	down.Stdout = os.Stdout
	down.Stderr = os.Stderr
	_ = down.Run()

	os.Exit(code)
}
