package e2e

import (
	"fmt"
	"testing"
	"github.com/playwright-community/playwright-go"
)

func TestWebsiteBuilderWizard(t *testing.T) {
	page := newPage(t)
	// Skip if page is nil (e.g., Playwright not installed)
	if page == nil {
		t.Skip("Playwright page is nil, skipping TestWebsiteBuilderWizard")
	}
    defer page.Close()

	// Login and start
    loginAsAdmin(t, page)

	err := page.Locator("body").WaitFor(playwright.LocatorWaitForOptions{
		Timeout: playwright.Float(10000),
	})
    if err != nil {
		t.Fatalf("could not wait for body: %v", err)
	}

	// Navigate to Wizard
	if _, err := page.Goto(baseURL + "/wizards/website-builder"); err != nil {
		t.Fatalf("could not goto wizard: %v", err)
	}
	page.WaitForLoadState()

	// Step 1: Choose a Template
	page.Locator("text=Minimalist").Click()
	page.Locator("text=Use this template →").Click()

	// Step 2: Brand Colors & Logo
	page.Locator("text=Ocean Blue").Click()
	// Test the mock AI logo generation
	page.Locator("text=Generate a logo for me").Click()
	page.Locator("text=Logo generated").WaitFor()
	page.Locator("text=Next Step →").Click()

	// Step 3: Add product
	page.Locator("text=Name").Locator("..").Locator("input").Fill("Test Cake")

	// Click the AI description generate button using tooltip text
	page.Locator("[aria-label='AI generate description']").Click()
	// Wait for the description field to have the generated content
	page.Locator("input[value='A beautiful, custom-made Test Cake perfect for any occasion.']").WaitFor()

	page.Locator("text=Next Step →").Click()

	// Step 4: Domain (One-tap selection advances automatically)
	page.Locator("text=Use a free OHC subdomain (mybusiness.ohc.app)").Click()

	// Step 5: Go Live
	page.Locator("text=Publish").Click()

	// Wait for success and redirect
	page.Locator("text=Site published! Link copied to clipboard.").WaitFor()
	page.WaitForURL("**/dashboard")

	fmt.Println("Wizard test finished successfully")
}
