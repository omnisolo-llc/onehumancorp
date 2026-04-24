package e2e

import (
	"testing"
	"time"
)

func TestBusinessSetupWizard(t *testing.T) {
	page, cleanup := setupE2ETest(t)
	defer cleanup()

	// 1. Log in using the UI.
	loginE2E(t, page, "admin@onehumancorp.com", "admin123")

	// Wait for home screen.
	expectElement(t, page, `[aria-label="Dashboard"]`)

	// Go to business setup wizard
	if err := page.Goto(baseURL + "/#/business_setup"); err != nil {
		t.Fatalf("Failed to navigate to business setup: %v", err)
	}

	time.Sleep(1 * time.Second)

	// Step 0 -> Step 1
	clickElement(t, page, "text=Get Started")
	expectElement(t, page, "text=What kind of business are you building?")

	// Step 1 -> Step 2
	clickElement(t, page, "text=Online Store")
	expectElement(t, page, "text=Tell us about your business")

	// Step 2 -> Step 3
	fillInput(t, page, "Business Name", "Acme AI Corp")

	// Wait for debounce and AI auto-suggest (500ms debounce + API time)
	time.Sleep(1500 * time.Millisecond)

	// In test mode or when no provider is strictly bound, generate_description returns
	// "A premium, handcrafted Acme AI Corp tailored for exceptional quality and performance."
	// Just assert that the input got filled.
	val, err := page.InputValue("text=Short Description")
	if err != nil || val == "" {
		t.Fatalf("AI auto suggest failed to populate description, got: '%v'", val)
	}

	clickElement(t, page, "text=Continue")
	expectElement(t, page, "text=What do you sell?")

	// Refresh the page to test State Persist/Restore!
	if err := page.Reload(); err != nil {
		t.Fatalf("Failed to reload: %v", err)
	}
	time.Sleep(1 * time.Second)
	// After reload, we should be right back at Step 3
	expectElement(t, page, "text=What do you sell?")

	// Step 3 -> Step 4
	clickElement(t, page, "text=Physical products")
	clickElement(t, page, "text=Continue")
	expectElement(t, page, "text=How do you want to receive payments?")

	// Step 4 -> Step 5
	clickElement(t, page, "text=Online only")
	clickElement(t, page, "text=Continue")
	expectElement(t, page, "text=Administrator account")

	// Step 5 -> Step 6
	fillInput(t, page, "Name", "Admin")
	fillInput(t, page, "Email", "admin@acmeai.com")
	fillInput(t, page, "Password", "supersecret123")
	clickElement(t, page, "text=Continue")
	expectElement(t, page, "text=Review & Launch")

	// Check summary output
	expectElement(t, page, "text=Acme AI Corp")
	expectElement(t, page, "text=Online Store")
	expectElement(t, page, "text=Physical products")
	expectElement(t, page, "text=Online only")
	expectElement(t, page, "text=admin@acmeai.com")

	// Finish
	clickElement(t, page, "text=Launch My Business →")

	// We expect the user to be redirected to the dashboard eventually.
	expectElement(t, page, "text=Dashboard")
}
