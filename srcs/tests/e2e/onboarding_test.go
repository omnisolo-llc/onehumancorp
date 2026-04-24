package e2e

import (
	"testing"
	"time"

	playwright "github.com/playwright-community/playwright-go"
	"github.com/stretchr/testify/require"
)

func TestOnboardingFlow(t *testing.T) {
	page := newPage(t)
	openApp(t, page)

	err := page.WaitForLoadState(playwright.PageWaitForLoadStateOptions{State: playwright.LoadStateNetworkidle})
	require.NoError(t, err)

	_, err = page.Goto(baseURL + "/#/login")
	require.NoError(t, err)
	err = page.WaitForLoadState(playwright.PageWaitForLoadStateOptions{State: playwright.LoadStateNetworkidle})
	require.NoError(t, err)

	require.NoError(t, page.Locator("text=One Human Corp").First().WaitFor())
	require.NoError(t, page.Locator("text=Don't have an account? Sign Up").Click())
	require.NoError(t, page.Locator("button:has-text('Sign Up')").WaitFor())

	uniqueUser := "testuser"
	require.NoError(t, page.Locator("input").Nth(0).Fill(uniqueUser))
	require.NoError(t, page.Locator("input").Nth(1).Fill(uniqueUser+"@example.com"))
	require.NoError(t, page.Locator("input").Nth(2).Fill("password123"))
	require.NoError(t, page.Locator("input").Nth(3).Fill("password123"))

	require.NoError(t, page.Locator("button:has-text('Sign Up')").Click())

	_ = page.WaitForURL("**/*business_setup*", playwright.PageWaitForURLOptions{Timeout: playwright.Float(10000)})

	require.NoError(t, page.Locator("text=Welcome!").First().WaitFor())
	require.NoError(t, page.Locator("button:has-text('Next')").Click())

	require.NoError(t, page.Locator("text=Business Name").First().WaitFor())
	require.NoError(t, page.Locator("button:has-text('Next')").Click())

	require.NoError(t, page.Locator("text=Payment Preferences").First().WaitFor())
	require.NoError(t, page.Locator("button:has-text('Next')").Click())

	require.NoError(t, page.Locator("text=Template Selection").First().WaitFor())
	require.NoError(t, page.Locator("button:has-text('Next')").Click())

	require.NoError(t, page.Locator("text=First Product / Service Add").First().WaitFor())
	require.NoError(t, page.Locator("button:has-text('Next')").Click())

	require.NoError(t, page.Locator("text=Domain & Go-Live").First().WaitFor())
	require.NoError(t, page.Locator("button:has-text('Publish')").Click())

	require.NoError(t, page.Locator("text=Welcome Checklist").First().WaitFor())
	require.NoError(t, page.Locator("text=Business live").First().WaitFor())
	require.NoError(t, page.Locator("button:has-text('Go to Dashboard')").Click())

	_ = page.WaitForURL("**/*dashboard*", playwright.PageWaitForURLOptions{Timeout: playwright.Float(10000)})
	require.NoError(t, page.Locator("text=Dashboard").First().WaitFor())

	time.Sleep(1 * time.Second)
}
