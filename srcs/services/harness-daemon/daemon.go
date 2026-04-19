package daemon

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"sync"

	"github.com/playwright-community/playwright-go"
)

type Daemon struct {
	mu          sync.Mutex
	port        int
	server      *http.Server
	pw          *playwright.Playwright
	browser     playwright.Browser
	page        playwright.Page
}

func NewDaemon(port int) *Daemon {
	return &Daemon{port: port}
}

func (d *Daemon) Start() error {
	d.mu.Lock()
	defer d.mu.Unlock()

	// Initialize Playwright
	err := playwright.Install()
	if err != nil {
		return fmt.Errorf("could not install playwright: %v", err)
	}

	pw, err := playwright.Run()
	if err != nil {
		return fmt.Errorf("could not start playwright: %v", err)
	}
	d.pw = pw

	browser, err := pw.Chromium.Launch(playwright.BrowserTypeLaunchOptions{
		Headless: playwright.Bool(true),
	})
	if err != nil {
		return fmt.Errorf("could not launch browser: %v", err)
	}
	d.browser = browser

	context, err := browser.NewContext()
	if err != nil {
		return fmt.Errorf("could not create context: %v", err)
	}

	page, err := context.NewPage()
	if err != nil {
		return fmt.Errorf("could not create page: %v", err)
	}
	d.page = page

	mux := http.NewServeMux()
	mux.HandleFunc("/command", d.handleCommand)

	d.server = &http.Server{
		Addr:    fmt.Sprintf(":%d", d.port),
		Handler: mux,
	}

	go func() {
		if err := d.server.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("Daemon server failed: %v", err)
		}
	}()

	return nil
}

func (d *Daemon) Stop(ctx context.Context) error {
	d.mu.Lock()
	defer d.mu.Unlock()

	if d.browser != nil {
		d.browser.Close()
	}
	if d.pw != nil {
		d.pw.Stop()
	}

	if d.server != nil {
		return d.server.Shutdown(ctx)
	}
	return nil
}

type CommandRequest struct {
	URL string `json:"url"`
}

type CommandResponse struct {
	Content string `json:"content"`
	Error   string `json:"error,omitempty"`
}

func (d *Daemon) handleCommand(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req CommandRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	d.mu.Lock()
	defer d.mu.Unlock()

	_, err := d.page.Goto(req.URL)
	if err != nil {
		json.NewEncoder(w).Encode(CommandResponse{Error: fmt.Sprintf("failed to goto %s: %v", req.URL, err)})
		return
	}

	// Optional: wait a moment for dynamic content
	d.page.WaitForTimeout(1000)

	content, err := d.page.Content()
	if err != nil {
		json.NewEncoder(w).Encode(CommandResponse{Error: fmt.Sprintf("failed to get content: %v", err)})
		return
	}

	// Simulated browser fetch
	resp := CommandResponse{
		Content: content,
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(resp)
}
