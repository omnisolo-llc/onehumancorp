package e2e

import (
	"testing"

	playwright "github.com/playwright-community/playwright-go"
)

// TestCujReferralsDashboardNavigatesFromHome verifies that a user can login, navigate to the
// Viral Referrals dashboard using the sidebar, and see the expected initial UI state.
func TestCujReferralsDashboardNavigatesFromHome(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	// 1. Start from the home page UI login
	loginAsAdmin(t, page)

	// 2. Click the "Viral Referrals" link in the sidebar
	linkLocator := page.Locator("nav").GetByText("Viral Referrals", playwright.LocatorGetByTextOptions{Exact: playwright.Bool(true)})
	if count, _ := linkLocator.Count(); count == 0 {
		linkLocator = page.GetByText("Viral Referrals", playwright.PageGetByTextOptions{Exact: playwright.Bool(true)})
	}

	if err := linkLocator.First().Click(); err != nil {
		t.Fatalf("Failed to click Viral Referrals sidebar link: %v", err)
	}

	// 3. Wait for the Referrals Dashboard to load
	page.WaitForLoadState(playwright.PageWaitForLoadStateOptions{State: playwright.LoadStateNetworkidle})

	// 4. Assert the final UI state: Dashboard title
	titleLocator := page.GetByText("Viral Loop Dashboard", playwright.PageGetByTextOptions{Exact: playwright.Bool(true)})
	if err := titleLocator.First().WaitFor(playwright.LocatorWaitForOptions{State: playwright.WaitForSelectorStateVisible, Timeout: playwright.Float(5000)}); err != nil {
		t.Fatalf("Failed to find 'Viral Loop Dashboard' title: %v", err)
	}

	// 5. Explicitly wait for the list content to render using WaitFor
	emptyStateLocator := page.GetByText("No referrals tracked yet.", playwright.PageGetByTextOptions{Exact: playwright.Bool(true)})
	cardLocator := page.GetByText("Ref:", playwright.PageGetByTextOptions{Exact: playwright.Bool(false)})

	// We wait up to 5 seconds for either the empty state or the card to be visible
	errEmpty := emptyStateLocator.First().WaitFor(playwright.LocatorWaitForOptions{State: playwright.WaitForSelectorStateVisible, Timeout: playwright.Float(5000)})
	errCard := cardLocator.First().WaitFor(playwright.LocatorWaitForOptions{State: playwright.WaitForSelectorStateVisible, Timeout: playwright.Float(5000)})

	if errEmpty != nil && errCard != nil {
		t.Fatalf("Neither empty state nor referral cards appeared on the Referrals dashboard within timeout. errEmpty: %v, errCard: %v", errEmpty, errCard)
	}

	t.Log("Successfully navigated to Viral Referrals Dashboard and verified UI state")
}
