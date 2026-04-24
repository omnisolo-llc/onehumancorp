package e2e

import (
    "testing"
    playwright "github.com/playwright-community/playwright-go"
    "github.com/stretchr/testify/require"
)

func TestCUJDashboardUX(t *testing.T) {
    if bCtx == nil {
        t.Skip("Browser context is not available")
    }

    // Setup - Create page and perform login
    page, err := bCtx.NewPage()
    require.NoError(t, err)
    defer page.Close()

    // Always use the established helper method that works robustly in the codebase
    loginAsAdmin(t, page)

    err = page.Locator("body").WaitFor(playwright.LocatorWaitForOptions{
        Timeout: playwright.Float(10000),
    })
    require.NoError(t, err)

    // Wait until URL changes to dashboard or we see some dashboard content
    page.WaitForURL("**/dashboard**", playwright.PageWaitForURLOptions{Timeout: playwright.Float(10000)})

    // Verify the UI changes
    err = page.GetByText("Today's Sales", playwright.PageGetByTextOptions{Exact: playwright.Bool(true)}).WaitFor(playwright.LocatorWaitForOptions{State: playwright.WaitForSelectorStateVisible})
    require.NoError(t, err)

    err = page.GetByText("Running Smoothly", playwright.PageGetByTextOptions{Exact: playwright.Bool(true)}).WaitFor(playwright.LocatorWaitForOptions{State: playwright.WaitForSelectorStateVisible})
    require.NoError(t, err)

    err = page.GetByText("AI Helpers", playwright.PageGetByTextOptions{Exact: playwright.Bool(true)}).WaitFor(playwright.LocatorWaitForOptions{State: playwright.WaitForSelectorStateVisible})
    require.NoError(t, err)
}
