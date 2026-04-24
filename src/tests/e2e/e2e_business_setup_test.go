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

	// For testing, let's navigate to /business_setup directly or click on the Setup Business button
	if err := page.Goto(baseURL + "/#/business_setup"); err != nil {
		t.Fatalf("could not go to setup page: %v", err)
	}
	time.Sleep(1 * time.Second)

	// Step 0: Welcome
	expectText(t, page, "Your business, live in minutes")
	clickButton(t, page, "Get Started")

	// Step 1: Business Type
	expectText(t, page, "What kind of business are you building?")
	clickElementWithText(t, page, "Online Store")

	// Step 2: Details
	expectText(t, page, "Tell us about your business")
	fillInputByLabel(t, page, "Business Name", "E2E EShop")
	fillInputByLabel(t, page, "Short Description", "E2E items")
	clickButton(t, page, "Continue")

	// Step 3: What do you sell?
	expectText(t, page, "What do you sell?")
	clickElementWithText(t, page, "Physical products")
	clickButton(t, page, "Continue")

	// Step 4: Payments
	expectText(t, page, "How do you want to receive payments?")
	clickElementWithText(t, page, "Online only")
	clickButton(t, page, "Continue")

	// Step 5: Admin
	expectText(t, page, "Administrator account")
	fillInputByLabel(t, page, "Name", "Admin")
	fillInputByLabel(t, page, "Email", "admin@e2e.com")
	fillInputByLabel(t, page, "Password", "secr3tpwd")
	clickButton(t, page, "Continue")

	// Step 6: Launch
	expectText(t, page, "Review & Launch")
	clickButton(t, page, "Launch My Business →")

	// Should land in dashboard
	time.Sleep(1 * time.Second)
	expectText(t, page, "Dashboard")
}
