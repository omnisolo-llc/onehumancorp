package e2e

import (
	"testing"
	"github.com/stretchr/testify/require"
	playwright "github.com/playwright-community/playwright-go"
)

func TestViralReferralLoopEndToEnd(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	// 1. Authenticate user
	loginAsAdmin(t, page)

	// 2. Navigate to User Management
	require.NoError(t, page.Locator("text=Users").Click())

	// 3. Open Invite User Dialog
	require.NoError(t, page.Locator("text=Invite User").Click())

	// 4. Input Username
	require.NoError(t, page.Locator("text=Username").Fill("TestReferralUser"))

	// 5. Generate Secure Invite (creates referral via API)
	require.NoError(t, page.Locator("text=Generate Secure Invite").Click())

	// 6. Assert snackbar pops up indicating link copied
	snackLocator := page.Locator("text=Cloud-Bridge invite link copied")
	err := snackLocator.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)

	// 7. Check Viral Loop Dashboard for new referral
	require.NoError(t, page.Locator("text=Viral Referrals").Click())

	// Wait for dashboard to load
	err = page.Locator("text=Viral Loop Dashboard").WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)

	// We expect the newly created referral to be visible
	// Wait for 'xYz8vQ_local_sovereign' to show up
	err = page.Locator("text=xYz8vQ_local_sovereign").WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)

	err = page.Locator("text=User: TestReferralUser").WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	require.NoError(t, err)
}
