package harnessdaemon

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"path/filepath"
	"sync"
	"time"

	"github.com/playwright-community/playwright-go"
)

// Daemon manages a persistent Chromium instance.
type Daemon struct {
	mu           sync.Mutex
	pw           *playwright.Playwright
	browser      playwright.Browser
	context      playwright.BrowserContext
	page         playwright.Page
	port         int
	server       *http.Server
	ready        bool
}

// CommandRequest represents an incoming command to the daemon.
type CommandRequest struct {
	Command string `json:"command"` // URL to navigate to or JS to evaluate
	Type    string `json:"type"`    // "goto", "eval", "content", "cookies", "set_cookie"
}

// CommandResponse represents the result of a command.
type CommandResponse struct {
	Stdout   string `json:"stdout"`
	Stderr   string `json:"stderr"`
	ExitCode int    `json:"exit_code"`
}

// NewDaemon creates a new Harness Daemon instance.
func NewDaemon(port int) *Daemon {
	return &Daemon{
		port: port,
	}
}

// Start initializes Playwright and starts the HTTP server.
func (d *Daemon) Start() error {
	d.mu.Lock()
	defer d.mu.Unlock()

	// Ensure playwright driver can be installed in a writable directory.
	// This is especially needed for bazel test environments.
	tmpDir := os.TempDir()
	pwDir := filepath.Join(tmpDir, "playwright-go")
	os.Setenv("PLAYWRIGHT_BROWSERS_PATH", pwDir)

	err := playwright.Install(&playwright.RunOptions{
		DriverDirectory: pwDir,
	})
	if err != nil {
		return fmt.Errorf("failed to install playwright: %v", err)
	}

	pw, err := playwright.Run(&playwright.RunOptions{
		DriverDirectory: pwDir,
	})
	if err != nil {
		return fmt.Errorf("failed to start playwright: %v", err)
	}
	d.pw = pw

	browser, err := pw.Chromium.Launch(playwright.BrowserTypeLaunchOptions{
		Headless: playwright.Bool(true),
	})
	if err != nil {
		return fmt.Errorf("failed to launch chromium: %v", err)
	}
	d.browser = browser

	ctx, err := browser.NewContext()
	if err != nil {
		return fmt.Errorf("failed to create context: %v", err)
	}
	d.context = ctx

	page, err := ctx.NewPage()
	if err != nil {
		return fmt.Errorf("failed to create page: %v", err)
	}
	d.page = page

	mux := http.NewServeMux()
	mux.HandleFunc("/command", d.handleCommand)
	mux.HandleFunc("/health", d.handleHealth)

	d.server = &http.Server{
		Addr:    fmt.Sprintf(":%d", d.port),
		Handler: mux,
	}

	d.ready = true
	go func() {
		log.Printf("Harness Daemon listening on :%d", d.port)
		if err := d.server.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("Server failed: %v", err)
		}
	}()

	return nil
}

// Stop shuts down the server and browser.
func (d *Daemon) Stop() error {
	d.mu.Lock()
	defer d.mu.Unlock()

	d.ready = false
	if d.server != nil {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		if err := d.server.Shutdown(ctx); err != nil {
			log.Printf("Failed to shutdown HTTP server: %v", err)
		}
	}

	if d.browser != nil {
		if err := d.browser.Close(); err != nil {
			log.Printf("Failed to close browser: %v", err)
		}
	}
	if d.pw != nil {
		if err := d.pw.Stop(); err != nil {
			log.Printf("Failed to stop playwright: %v", err)
		}
	}

	return nil
}

func (d *Daemon) handleHealth(w http.ResponseWriter, r *http.Request) {
	if d.ready {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("OK"))
	} else {
		w.WriteHeader(http.StatusServiceUnavailable)
	}
}

func (d *Daemon) handleCommand(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req CommandRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	d.mu.Lock()
	defer d.mu.Unlock()

	var res CommandResponse

	switch req.Type {
	case "goto":
		if _, err := d.page.Goto(req.Command); err != nil {
			res.Stderr = err.Error()
			res.ExitCode = 1
		} else {
			res.Stdout = "Navigated to " + req.Command
			res.ExitCode = 0
		}
	case "eval":
		val, err := d.page.Evaluate(req.Command)
		if err != nil {
			res.Stderr = err.Error()
			res.ExitCode = 1
		} else {
			res.Stdout = fmt.Sprintf("%v", val)
			res.ExitCode = 0
		}
	case "content":
		content, err := d.page.Content()
		if err != nil {
			res.Stderr = err.Error()
			res.ExitCode = 1
		} else {
			res.Stdout = content
			res.ExitCode = 0
		}
	case "cookies":
		cookies, err := d.context.Cookies()
		if err != nil {
			res.Stderr = err.Error()
			res.ExitCode = 1
		} else {
			cookiesJSON, _ := json.Marshal(cookies)
			res.Stdout = string(cookiesJSON)
			res.ExitCode = 0
		}
	case "set_cookie":
		var cookie playwright.OptionalCookie
		if err := json.Unmarshal([]byte(req.Command), &cookie); err != nil {
			res.Stderr = "invalid cookie format"
			res.ExitCode = 1
		} else {
			if err := d.context.AddCookies([]playwright.OptionalCookie{cookie}); err != nil {
				res.Stderr = err.Error()
				res.ExitCode = 1
			} else {
				res.Stdout = "Cookie set"
				res.ExitCode = 0
			}
		}
	default:
		res.Stderr = "Unknown command type: " + req.Type
		res.ExitCode = 1
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(res)
}
