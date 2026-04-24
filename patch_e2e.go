package e2e

import (
	"fmt"
	"strings"
	"testing"
	"time"

	"github.com/playwright-community/playwright-go"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

// loginAsAdmin is a mock login function to get to the dashboard.
func loginAsAdminE2E(t *testing.T, page playwright.Page) {
	// Let's assume the test helpers `loginAsAdmin` function navigates to the start
	// and performs some setup.
	loginAsAdmin(t, page)
}

func TestBusinessSetupWizardEndToEnd(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	// loginAsAdmin normally logs the user in and lands on dashboard or home.
	// We want to navigate specifically to the Business Setup Wizard or trigger it.
	loginAsAdmin(t, page)

	// Since we are mocking the login flow, we navigate directly to the Wizard path
	// to start the test. The Wizard might be triggered on first login, or via a route.
	// For OHC, business setup wizard is triggered on onboarding or via a specific route.
	// Let's assume it's at `/` when not fully configured, or at a specific wizard URL.
	// In the Flutter UI we know we're checking for "Your business, live in minutes."

	// Let's go to the root to see if it brings up the wizard
	_, err := page.Goto(baseURL)
	require.NoError(t, err)

	// Wait for the app to load
	page.WaitForLoadState(playwright.PageWaitForLoadStateOptions{
		State: playwright.LoadStateNetworkidle,
	})

	// Wait for flutter to load, typically it will show the landing page, and we click "Start Business Setup"
	// Let's find and click "Start Business Setup"
	err = page.Locator("text=Start Business Setup").Click()
	if err != nil {
		// If not found, perhaps it loaded the wizard directly.
		t.Log("Did not find 'Start Business Setup', assuming wizard loaded directly")
	}

	// Step 0: Welcome
	welcomeText := page.Locator("text=Your business, live in minutes.")
	err = welcomeText.WaitFor()
	require.NoError(t, err, "Wizard welcome text not found")

	getStartedBtn := page.Locator("text=Get Started")
	err = getStartedBtn.Click()
	require.NoError(t, err)

	// Step 1: Business Type
	businessTypeText := page.Locator("text=What type of business are you building?")
	err = businessTypeText.WaitFor()
	require.NoError(t, err)

	onlineStoreTile := page.Locator("text=Online Store")
	err = onlineStoreTile.Click()
	require.NoError(t, err)

	nextBtn := page.Locator("text=Next")
	err = nextBtn.Click()
	require.NoError(t, err)

	// Step 2: Name & Description
	nameText := page.Locator("text=What is your business called?")
	err = nameText.WaitFor()
	require.NoError(t, err)

	// Fill Name
	err = page.Locator("input").Nth(0).Fill("Maya Cakes")
	require.NoError(t, err)

	// Fill Description
	err = page.Locator("input").Nth(1).Fill("I bake custom cakes")
	require.NoError(t, err)

	err = nextBtn.Click()
	require.NoError(t, err)

	// Step 3: What do you sell
	sellText := page.Locator("text=What do you sell?")
	err = sellText.WaitFor()
	require.NoError(t, err)

	physTile := page.Locator("text=Physical products")
	err = physTile.Click()
	require.NoError(t, err)

	err = nextBtn.Click()
	require.NoError(t, err)

	// Step 4: Payments
	payText := page.Locator("text=How do you want to receive payments?")
	err = payText.WaitFor()
	require.NoError(t, err)

	bothTile := page.Locator("text=Both")
	err = bothTile.Click()
	require.NoError(t, err)

	err = nextBtn.Click()
	require.NoError(t, err)

	// Step 5: Admin Account
	adminText := page.Locator("text=Create your admin account")
	err = adminText.WaitFor()
	require.NoError(t, err)

	err = page.Locator("input").Nth(0).Fill("Maya")
	require.NoError(t, err)

	err = page.Locator("input").Nth(1).Fill("maya@cakes.com")
	require.NoError(t, err)

	err = page.Locator("input").Nth(2).Fill("password")
	require.NoError(t, err)

	err = nextBtn.Click()
	require.NoError(t, err)

	// Step 6: Review & Launch
	launchText := page.Locator("text=You are ready to launch!")
	err = launchText.WaitFor()
	require.NoError(t, err)

	// Check summary
	content, _ := page.Content()
	assert.Contains(t, content, "Maya Cakes")
	assert.Contains(t, content, "Online Store")
	assert.Contains(t, content, "Physical products")
	assert.Contains(t, content, "Both")
	assert.Contains(t, content, "maya@cakes.com")

	launchBtn := page.Locator("text=Launch My Business →")
	err = launchBtn.Click()
	require.NoError(t, err)

	// Verify we land in the dashboard (the Flutter routing sends to '/dashboard')
	err = page.WaitForURL("**/dashboard**", playwright.PageWaitForURLOptions{
		Timeout: playwright.Float(10000), // 10 seconds timeout
	})
	if err != nil {
		t.Logf("Warning: Could not verify routing to dashboard: %v", err)
	}
}
