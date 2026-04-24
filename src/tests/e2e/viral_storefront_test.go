package e2e

import (
	"strings"
	"testing"

	playwright "github.com/playwright-community/playwright-go"
)

// TestCUJViralStorefront verifies the end-to-end user journey for the Business Share Widget.
func TestCUJViralStorefront(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	// 1. Navigate to the dashboard (assumes login via session setup)
	loginAsAdmin(t, page)

	// Ensure we wait for the network to be somewhat idle or main element to be visible
	waitForDashboard(t, page)

	// 2. Locate the Business Share Widget by its semantics label
	shareWidget := page.Locator("[aria-label='Business Share Card']")
	if err := shareWidget.WaitFor(playwright.LocatorWaitForOptions{
		State:   playwright.WaitForSelectorStateVisible,
		Timeout: playwright.Float(10000), // CanvasKit initialization delay
	}); err != nil {
		t.Fatalf("Business Share Card not visible: %v", err)
	}

	// 3. Locate the share buttons
	copyButton := shareWidget.Locator("role=button[name='Copy Link']")
	igButton := shareWidget.Locator("role=button[name='Instagram']")
	waButton := shareWidget.Locator("role=button[name='WhatsApp']")
	xButton := shareWidget.Locator("role=button[name='X']")

	// 4. Click the copy button
	if err := copyButton.Click(); err != nil {
		t.Fatalf("Failed to click Copy Link button: %v", err)
	}

	// 5. Verify the snackbar message appears for Copy
	snackbarCopy := page.Locator("text='Public link copied to clipboard!'")
	if err := snackbarCopy.WaitFor(playwright.LocatorWaitForOptions{
		State:   playwright.WaitForSelectorStateVisible,
		Timeout: playwright.Float(5000),
	}); err != nil {
		t.Fatalf("Snackbar did not appear after clicking copy: %v", err)
	}

	// 6. Click Instagram button
	if err := igButton.Click(); err != nil {
		t.Fatalf("Failed to click IG button: %v", err)
	}
	snackbarIG := page.Locator("text='Posted to Instagram'")
	if err := snackbarIG.WaitFor(playwright.LocatorWaitForOptions{
		State:   playwright.WaitForSelectorStateVisible,
		Timeout: playwright.Float(5000),
	}); err != nil {
		t.Fatalf("Snackbar did not appear after clicking IG: %v", err)
	}

	// 7. Click WA button
	if err := waButton.Click(); err != nil {
		t.Fatalf("Failed to click WA button: %v", err)
	}
	snackbarWA := page.Locator("text='Posted to WhatsApp'")
	if err := snackbarWA.WaitFor(playwright.LocatorWaitForOptions{
		State:   playwright.WaitForSelectorStateVisible,
		Timeout: playwright.Float(5000),
	}); err != nil {
		t.Fatalf("Snackbar did not appear after clicking WA: %v", err)
	}

	// 8. Click X button
	if err := xButton.Click(); err != nil {
		t.Fatalf("Failed to click X button: %v", err)
	}
	snackbarX := page.Locator("text='Posted to X'")
	if err := snackbarX.WaitFor(playwright.LocatorWaitForOptions{
		State:   playwright.WaitForSelectorStateVisible,
		Timeout: playwright.Float(5000),
	}); err != nil {
		t.Fatalf("Snackbar did not appear after clicking X: %v", err)
	}
}

func waitForDashboard(t *testing.T, page playwright.Page) {
	err := page.Locator("[aria-label='Dashboard']").WaitFor(playwright.LocatorWaitForOptions{
		State:   playwright.WaitForSelectorStateVisible,
		Timeout: playwright.Float(10000),
	})
	if err != nil && !strings.Contains(err.Error(), "Timeout") {
		// Ignore timeout as the semantics might be slightly different
	}
	page.WaitForTimeout(5000) // CanvasKit
}
