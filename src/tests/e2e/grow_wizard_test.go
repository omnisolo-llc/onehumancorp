package e2e

import (
	"testing"
	"time"

	"github.com/playwright-community/playwright-go"
)

func TestGrowMyBusinessWizard(t *testing.T) {
	t.Parallel()

	// Playwright is installed via bazel dependencies
	pw, err := playwright.Run()
	if err != nil {
		t.Skipf("skipping test: could not start playwright: %v", err)
	}
	defer pw.Stop()

	browser, err := pw.Chromium.Launch(playwright.BrowserTypeLaunchOptions{
		Headless: playwright.Bool(true),
	})
	if err != nil {
		t.Fatalf("could not launch browser: %v", err)
	}
	defer browser.Close()

	page, err := browser.NewPage()
	if err != nil {
		t.Fatalf("could not create page: %v", err)
	}
	defer page.Close()

	// Use helper for common setup and login flow
	loginAsAdmin(t, page)

	// Wait for navigation to dashboard
	err = page.WaitForURL("**/dashboard", playwright.PageWaitForURLOptions{Timeout: playwright.Float(10000)})
	if err != nil {
		t.Logf("did not land directly on dashboard, going to dashboard manually")
		if _, err := page.Goto(baseURL + "/#/dashboard"); err != nil {
			t.Fatalf("could not goto dashboard: %v", err)
		}
	}

	// Wait for dashboard to load
	time.Sleep(2 * time.Second)

	// Force semantic tree if needed
	page.Evaluate(`() => { if (window._flutter_semantics_enable) window._flutter_semantics_enable(); }`)
	time.Sleep(1 * time.Second)

	// Trigger "Grow my business"
	err = page.Locator("[aria-label=\"Grow my business\"]").WaitFor(playwright.LocatorWaitForOptions{State: playwright.WaitForSelectorStateVisible})
	if err == nil {
		err = page.Locator("[aria-label=\"Grow my business\"]").Click()
	}
	if err != nil {
		t.Fatalf("failed to click Grow my business button: %v", err)
	}

	// Verify wizard loaded and click "See Suggestions"
	err = page.Locator("[aria-label=\"See Suggestions\"]").WaitFor(playwright.LocatorWaitForOptions{State: playwright.WaitForSelectorStateVisible})
	if err == nil {
		err = page.Locator("[aria-label=\"See Suggestions\"]").Click()
	}
	if err != nil {
		t.Fatalf("failed to click See Suggestions: %v", err)
	}

	// Click an action, e.g. "Do it" for Add 5 more products
	err = page.Locator("[aria-label=\"Do it\"]").WaitFor(playwright.LocatorWaitForOptions{State: playwright.WaitForSelectorStateVisible})
	if err == nil {
		err = page.Locator("[aria-label=\"Do it\"]").Click()
	}
	if err != nil {
		t.Fatalf("failed to click Do it: %v", err)
	}

	// Wait for processing to finish and Verify "Back to Dashboard" appears
	err = page.Locator("[aria-label=\"Back to Dashboard\"]").WaitFor(playwright.LocatorWaitForOptions{State: playwright.WaitForSelectorStateVisible, Timeout: playwright.Float(5000)})
	if err == nil {
		err = page.Locator("[aria-label=\"Back to Dashboard\"]").Click()
	}
	if err != nil {
		t.Fatalf("failed to click Back to Dashboard: %v", err)
	}

	// Verify we are back to dashboard
	err = page.WaitForURL("**/dashboard", playwright.PageWaitForURLOptions{Timeout: playwright.Float(5000)})
	if err != nil {
		t.Fatalf("failed to navigate back to dashboard: %v", err)
	}
}
