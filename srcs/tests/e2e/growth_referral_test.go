package e2e

import (
	"testing"

	"github.com/playwright-community/playwright-go"
	"github.com/stretchr/testify/require"
)

func TestGrowthReferralWidget(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Ensure the widget text is visible.
	err := page.Locator("text=Grow Your Swarm. Maintain Sovereignty.").WaitFor(playwright.LocatorWaitForOptions{
		Timeout: playwright.Float(10000),
	})
	require.NoError(t, err, "GrowthReferralWidget should be visible on Dashboard")

	// Ensure the quota is visible.
	err = page.Locator("text=missions used").WaitFor()
	require.NoError(t, err, "Quota should be visible")

	// Click "Invite Team to Expand Quota"
	err = page.Locator("text=Invite Team to Expand Quota").Click()
	require.NoError(t, err, "Should click Invite Team button")

	// Verify SnackBar appears
	err = page.Locator("text=Cloud-Bridge invite link copied").WaitFor(playwright.LocatorWaitForOptions{
		Timeout: playwright.Float(5000),
	})
	require.NoError(t, err, "SnackBar with invite link should appear")
}
