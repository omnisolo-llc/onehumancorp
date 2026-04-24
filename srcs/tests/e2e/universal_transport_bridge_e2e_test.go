package e2e

import (
	"fmt"
	"testing"

	playwright "github.com/playwright-community/playwright-go"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestCUJUniversalTransportBridge(t *testing.T) {
	if bCtx == nil {
		t.Skip("Browser context is not available")
	}

	page := newPage(t)
	defer page.Close()

	// Ensure we don't mock any network requests
	// We want to test the full stack, including the HTTP endpoint /api/diagnostics/transport

	// 1. Setup - Create page and perform login
	loginAsAdmin(t, page)

	// Wait for dashboard or any indication of being logged in before continuing
	err := page.Locator("body").WaitFor(playwright.LocatorWaitForOptions{
		Timeout: playwright.Float(10000),
	})
	require.NoError(t, err)

	// 2. Direct Navigation to Diagnostics Screen
	// If there's no navigation link to diagnostics, we go there directly.
	_, err = page.Goto(baseURL + "/diagnostics")
	require.NoError(t, err)

	// Wait for Diagnostics page to load
	err = page.Locator("text=Diagnostics Dashboard").First().WaitFor(playwright.LocatorWaitForOptions{
		Timeout: playwright.Float(10000),
	})
	require.NoError(t, err)

	// 3. Click the "Run Diagnostics" button to execute transport test
	// The button has a key assigned: 'run-diagnostics-btn'
	btn := page.Locator("button", playwright.PageLocatorOptions{
		HasText: "Run Diagnostics",
	}).First()

	// Ensure the button is visible
	err = btn.WaitFor(playwright.LocatorWaitForOptions{
		State:   playwright.WaitForSelectorStateVisible,
		Timeout: playwright.Float(5000),
	})
	require.NoError(t, err)

	err = btn.Click()
	require.NoError(t, err)

	// 4. Verify result from the API endpoint
	// Our dart code sets a Key('diagnostics-result') when displaying the text
	// Wait for the result message indicating the bridge successfully round-tripped.
	resultLocator := page.Locator("text=bridge active").First()
	err = resultLocator.WaitFor(playwright.LocatorWaitForOptions{
		State:   playwright.WaitForSelectorStateVisible,
		Timeout: playwright.Float(5000),
	})

	if err != nil {
		// Fallback check
		content, err := page.Content()
		require.NoError(t, err)
		assert.Contains(t, content, "bridge active")
	} else {
		assert.NoError(t, err)
	}

	fmt.Println("Universal Transport Bridge E2E test completed successfully")
}
