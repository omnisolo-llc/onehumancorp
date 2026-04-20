package e2e

import (
	"fmt"
	"log"
	"os"
	"path/filepath"
	"time"

	playwright "github.com/playwright-community/playwright-go"
)

const (
	baseURL = "http://localhost:8083/?renderer=html" // Keeping HTML renderer for better locators
	outputDir = "artifacts/screenshots"
)

func main() {
	if err := os.MkdirAll(outputDir, 0755); err != nil {
		log.Fatalf("could not create output dir: %v", err)
	}

	pw, err := playwright.Run()
	if err != nil {
		log.Fatalf("could not start playwright: %v", err)
	}
	defer pw.Stop()

	browser, err := pw.Chromium.Launch(playwright.BrowserTypeLaunchOptions{
		Headless: playwright.Bool(false), // Run with head if possible (xvfb handles it)
	})
	if err != nil {
		log.Fatalf("could not launch browser: %v", err)
	}
	defer browser.Close()

	page, err := browser.NewPage()
	if err != nil {
		log.Fatalf("could not create page: %v", err)
	}

	fmt.Println("Starting CUJ Audit (Refactored)...")

	// 1. Authentication CUJ
	fmt.Println("Step 1: Authentication...")
	if _, err := page.Goto(baseURL); err != nil {
		log.Fatalf("could not goto baseURL: %v", err)
	}
	time.Sleep(5 * time.Second) // Wait for Flutter

	// Enable accessibility
	enableAccessibility(page)
	takeScreenshot(page, "01_landing_page_updated.png")

	// Navigate to login
    fmt.Println("Navigating to login...")
    // Using a more robust selector after UI cleanup
	if err := page.Click("text=Or continue to Cloud Dashboard"); err != nil {
		fmt.Printf("Warning: Login button click failed: %v\n", err)
	}
	time.Sleep(2 * time.Second)
	takeScreenshot(page, "02_login_form_updated.png")

	// Fill login with NEW CREDENTIALS
    fmt.Println("Entering credentials (admin@example.com / admin)...")
	page.Fill("input[type=\"email\"]", "admin@example.com")
	page.Fill("input[type=\"password\"]", "admin")
	page.Click("text=Sign In")
	time.Sleep(3 * time.Second)
	takeScreenshot(page, "03_dashboard_updated.png")

	// Check for UID in profile (manual verification via screenshot)
    fmt.Println("Capturing profile for UID verification...")
    // Click on profile/settings if possible
    page.Click("text=Settings")
    time.Sleep(2 * time.Second)
	takeScreenshot(page, "04_settings_uid.png")

	// 2. Agent Management CUJ
	fmt.Println("Step 2: Agent Management...")
    page.Goto(baseURL + "/#/agents")
	time.Sleep(2 * time.Second)
	takeScreenshot(page, "05_agents_list.png")

	fmt.Println("Audit complete. Screenshots saved to", outputDir)
}

func enableAccessibility(page playwright.Page) {
	fmt.Println("Enabling Accessibility...")
	placeholder := page.Locator("flt-semantics-placeholder[aria-label=\"Enable accessibility\"]")
	if visible, _ := placeholder.IsVisible(); visible {
		placeholder.Click()
	}
}

func takeScreenshot(page playwright.Page, name string) {
	path := filepath.Join(outputDir, name)
	if _, err := page.Screenshot(playwright.PageScreenshotOptions{
		Path: playwright.String(path),
	}); err != nil {
		fmt.Printf("Failed to take screenshot %s: %v\n", name, err)
	} else {
		fmt.Printf("Screenshot saved: %s\n", path)
	}
}
