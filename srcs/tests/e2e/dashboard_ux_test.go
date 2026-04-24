package e2e

import (
	"testing"

	playwright "github.com/playwright-community/playwright-go"
	"github.com/stretchr/testify/require"
)

func TestEchoDashboardSimplificationUX(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Ensure the page loads properly
	_, err := page.Goto(baseURL + "/dashboard")
	require.NoError(t, err)
	page.WaitForLoadState(playwright.PageWaitForLoadStateOptions{State: playwright.LoadStateNetworkidle})

	// Assert that "Today's Sales" exists
	count, err := page.GetByText("Today's Sales").Count()
	require.NoError(t, err)
	require.Greater(t, count, 0, "Dashboard missing 'Today's Sales' plain-language label")

	// Assert that "Dashboard Updates" exists
	count, err = page.GetByText("Dashboard Updates").Count()
	require.NoError(t, err)
	require.Greater(t, count, 0, "Dashboard missing 'Dashboard Updates' label")
}
