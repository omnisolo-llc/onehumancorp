package e2e

import (
	"testing"
	"time"
	"github.com/playwright-community/playwright-go"
)

func TestBusinessSetupWizard(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	// 1. Log in using the UI.
	loginAsAdmin(t, page)

	// Setup Wizard might be triggered by a specific button or URL
	// For testing, let's navigate to /business_setup directly or click on the Setup Business button
	if _, err := page.Goto(baseURL + "/#/business_setup_wizard"); err != nil {
		t.Fatalf("Failed to navigate to business setup: %v", err)
	}

	// Wait for the UI to settle
	time.Sleep(1 * time.Second)

	// Helper function for click
	clickText := func(text string) {
		loc := page.Locator("text=" + text)
		if err := loc.First().Click(); err != nil {
			t.Fatalf("Failed to click %s: %v", text, err)
		}
		time.Sleep(1 * time.Second) // wait for animation
	}

	// Helper for checking text visibility
	expectText := func(text string) {
		loc := page.Locator("text=" + text)
		if err := loc.First().WaitFor(playwright.LocatorWaitForOptions{
			State: playwright.WaitForSelectorStateVisible,
			Timeout: playwright.Float(5000),
		}); err != nil {
			t.Fatalf("Expected text %s to be visible: %v", text, err)
		}
	}

	// Step 0 -> Step 1
	clickText("Get Started")
	expectText("What kind of business are you building?")

	// Step 1 -> Step 2
	clickText("Online Store")
	expectText("Tell us about your business")

	// Step 2 -> Step 3
	// Just fill any input available for step 2
	inputs := page.Locator("input")
	inputs.Nth(0).Fill("Acme Corp")
	inputs.Nth(1).Fill("A great store")
	clickText("Continue")
	expectText("What do you sell?")

	// Step 3 -> Step 4
	clickText("Physical products")
	clickText("Continue")
	expectText("How do you want to receive payments?")

	// Step 4 -> Step 5
	clickText("Online only")
	clickText("Continue")
	expectText("Administrator account")

	// Step 5 -> Step 6
	inputs = page.Locator("input")
	inputs.Nth(0).Fill("Admin")
	inputs.Nth(1).Fill("admin@acmecorp.com")
	inputs.Nth(2).Fill("supersecret")
	clickText("Continue")
	expectText("Review & Launch")

	// Check summary output
	expectText("Acme Corp")
	expectText("Online Store")
	expectText("Physical products")
	expectText("Online only")
	expectText("admin@acmecorp.com")

	// Finish
	clickText("Launch My Business →")

	// We expect the user to be redirected to the dashboard eventually.
	// We can just sleep and check if the dashboard element is present, but checking URL is better.
	time.Sleep(2 * time.Second)

	url := page.URL()
	if url != baseURL+"/#/" && url != baseURL+"/#/dashboard" && url != baseURL+"/" {
		t.Fatalf("Expected dashboard URL after launch, got: %s", url)
	}
}
