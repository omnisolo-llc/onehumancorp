package e2e

import (
	"testing"

	playwright "github.com/playwright-community/playwright-go"
	"github.com/stretchr/testify/require"
)

func TestCujLandingPageExperiments(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	// 1. Login to the application
	loginAsAdmin(t, page)

	// 2. Navigate to the Experiments screen using natural UI navigation
	_, err := page.Goto(baseURL + "/dashboard")
	require.NoError(t, err)
	page.WaitForLoadState(playwright.PageWaitForLoadStateOptions{
		State: playwright.LoadStateNetworkidle,
	})

	err = page.GetByText("Growth Experiments").First().Click()
	require.NoError(t, err)

	// Wait for the page to load
	page.WaitForLoadState(playwright.PageWaitForLoadStateOptions{
		State: playwright.LoadStateNetworkidle,
	})

	// 3. Verify page title
	count, err := page.GetByText("Growth Experiments").Count()
	require.NoError(t, err)
	require.GreaterOrEqual(t, count, 1, "Growth Experiments title should be visible")

	// 4. Click the "New Experiment" button (floating action button)
	newExpBtn := page.GetByText("New Experiment").First()
	err = newExpBtn.Click()
	require.NoError(t, err)

	// 5. Wait for the dialog to appear
	dialogTitle := page.GetByText("New Growth Experiment").First()
	err = dialogTitle.WaitFor()
	require.NoError(t, err)

	// 6. Fill in the experiment details
	titleInput := page.GetByLabel("Experiment Title").First()
	err = titleInput.Fill("Test Promo Banner A/B")
	require.NoError(t, err)

	splitInput := page.GetByLabel("Traffic Split (0.0 to 1.0)").First()
	err = splitInput.Fill("0.8")
	require.NoError(t, err)

	// 7. Click "Launch Experiment"
	launchBtn := page.GetByText("Launch Experiment").First()
	err = launchBtn.Click()
	require.NoError(t, err)

	// 8. Wait for the dialog to close (it should no longer be visible)
	err = dialogTitle.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateHidden,
	})
	require.NoError(t, err)

	// 9. Verify the experiment appears in the list (wait for it to become visible)
	newExpTitle := page.GetByText("Test Promo Banner A/B").First()
	err = newExpTitle.WaitFor()
	require.NoError(t, err)

	expSplit := page.GetByText("Traffic Split: 80.0%").First()
	err = expSplit.WaitFor()
	require.NoError(t, err)

	// 10. Verify persistence by refreshing the page and checking if the data is still there
	_, err = page.Reload()
	require.NoError(t, err)

	page.WaitForLoadState(playwright.PageWaitForLoadStateOptions{
		State: playwright.LoadStateNetworkidle,
	})

	err = newExpTitle.WaitFor()
	require.NoError(t, err, "Experiment title should persist after reload")

	err = expSplit.WaitFor()
	require.NoError(t, err, "Experiment traffic split should persist after reload")
}
