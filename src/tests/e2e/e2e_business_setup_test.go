package e2e

import (
	"testing"
	"time"
)

func TestBusinessSetupWizard(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	// Wait for home screen. (Before login)
	if _, err := page.Goto(baseURL + "/"); err != nil {
		t.Fatalf("Failed to navigate to root: %v", err)
	}

	loginAsAdmin(t, page)

	// Go to dashboard
	expectElement(t, page, "Dashboard")

	// Navigate to the Setup Business wizard
	clickElement(t, page, "Start Business Setup")
	time.Sleep(1 * time.Second) // wait for UI to settle

	// Step 0 -> Step 1
	clickElement(t, page, "Get Started")
	expectElement(t, page, "What kind of business are you building?")

	// Step 1 -> Step 2
	clickElement(t, page, "Online Store")
	expectElement(t, page, "Tell us about your business")

	// Step 2 -> Step 3
	fillInput(t, page, "Business Name", "Acme Corp")
	fillInput(t, page, "Short Description", "A great store")
	clickElement(t, page, "Continue")
	expectElement(t, page, "What do you sell?")

	// Step 3 -> Step 4
	clickElement(t, page, "Physical products")
	clickElement(t, page, "Continue")
	expectElement(t, page, "How do you want to receive payments?")

	// Step 4 -> Step 5
	clickElement(t, page, "Online only")
	clickElement(t, page, "Continue")
	expectElement(t, page, "Administrator account")

	// Step 5 -> Step 6
	fillInput(t, page, "Name", "Admin")
	fillInput(t, page, "Email", "admin@acmecorp.com")
	fillInput(t, page, "Password", "supersecret")
	clickElement(t, page, "Continue")
	expectElement(t, page, "Review & Launch")

	// Check summary output
	expectElement(t, page, "Acme Corp")
	expectElement(t, page, "Online Store")
	expectElement(t, page, "Physical products")
	expectElement(t, page, "Online only")
	expectElement(t, page, "admin@acmecorp.com")

	// Finish
	clickElement(t, page, "Launch My Business →")

	// Wait for Dashboard
	expectElement(t, page, "Dashboard")
}
