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
	fillInput(t, page, "Business Name", "Acme Corp")
	fillInput(t, page, "Short Description", "A great store")
	clickElement(t, page, "text=Continue")
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
	fillInput(t, page, "Email", "admin@acmecorp.com")
	fillInput(t, page, "Password", "supersecret")
	clickElement(t, page, "text=Continue")
	expectElement(t, page, "text=Review & Launch")

	// Check summary output
	expectElement(t, page, "text=Acme Corp")
	expectElement(t, page, "text=Online Store")
	expectElement(t, page, "text=Physical products")
	expectElement(t, page, "text=Online only")
	expectElement(t, page, "text=admin@acmecorp.com")

	// Finish
	clickElement(t, page, "text=Launch My Business →")

	expectElement(t, page, "text=Dashboard")
}
