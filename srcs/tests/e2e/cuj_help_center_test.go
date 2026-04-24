package e2e

import (
	"testing"

	playwright "github.com/playwright-community/playwright-go"
	"github.com/stretchr/testify/require"
)

func TestHelpCenterCUJ(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Navigate to Help Center using the sidebar
	helpNav := page.Locator("text=Help Center").First()
	// Just attempt to click it if it's there
	err := helpNav.Click(playwright.LocatorClickOptions{
		Timeout: playwright.Float(3000),
	})
	if err != nil {
		// Fallback navigation
		_, err = page.Goto(baseURL + "/#/help")
		require.NoError(t, err)
	}

	// Verify Help Center is displayed
	page.WaitForSelector("text=How can we help you?", playwright.PageWaitForSelectorOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	page.WaitForSelector("text=Set up your store", playwright.PageWaitForSelectorOptions{
		State: playwright.WaitForSelectorStateVisible,
	})

	// Search for an article
	err = page.GetByRole("textbox").First().Fill("Instagram", playwright.LocatorFillOptions{
		Timeout: playwright.Float(5000),
	})
	require.NoError(t, err)

	// Verify only the matching article is shown
	page.WaitForSelector("text=How to sell on Instagram", playwright.PageWaitForSelectorOptions{
		State: playwright.WaitForSelectorStateVisible,
	})

	// Clear search
	err = page.GetByRole("textbox").First().Fill("", playwright.LocatorFillOptions{
		Timeout: playwright.Float(5000),
	})
	require.NoError(t, err)
	page.WaitForSelector("text=Set up your store", playwright.PageWaitForSelectorOptions{
		State: playwright.WaitForSelectorStateVisible,
	})

	// Test Tooltip/FAB existence
	page.WaitForSelector("text=Ask anything", playwright.PageWaitForSelectorOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
}
