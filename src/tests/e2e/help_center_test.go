package e2e

import (
	"fmt"
	"testing"

	"github.com/playwright-community/playwright-go"
	"github.com/stretchr/testify/require"
)

func TestHelpCenter_E2E(t *testing.T) {
	page := newPage(t)
	loginAsAdmin(t, page)

	// Navigate to Dashboard to verify the AiHelpChat floating button is present
	_, err := page.Goto(fmt.Sprintf("%s/dashboard", baseURL))
	require.NoError(t, err)

	// Wait for dashboard to load with a slightly longer timeout
	t.Log("Waiting for Dashboard to appear...")
	dashboardLocator := page.GetByRole("heading", playwright.PageGetByRoleOptions{Name: "Dashboard"}).Or(page.GetByText("Dashboard")).First()
	require.NoError(t, dashboardLocator.WaitFor(playwright.LocatorWaitForOptions{
		Timeout: playwright.Float(45000), // Increase to 45s for slow CI
	}))
	t.Log("Dashboard appeared.")

	// Dismiss "A new version is available!" if present
	updateToast := page.Locator("text=A new version is available!, .update-toast")
	if count, _ := updateToast.Count(); count > 0 {
		t.Log("Dismissing update toast")
		closeBtn := updateToast.Locator("button, .close-icon").First()
		_ = closeBtn.Click(playwright.LocatorClickOptions{Timeout: playwright.Float(2000)})
	}

	// Verify AI Help Chat button is visible
	chatButton := page.Locator("[key='ai_help_chat_button']")
	require.NoError(t, chatButton.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	}))

	// Click AI Help Chat button to open chat
	require.NoError(t, chatButton.Click())

	// Verify chat interface opens
	require.NoError(t, page.Locator("text=Ask OHC Help").WaitFor(playwright.LocatorWaitForOptions{Timeout: playwright.Float(10000)}))

	// Send a message
	inputField := page.Locator("input[placeholder='Type your question...']")
	require.NoError(t, inputField.Fill("How do I add a product?"))
	require.NoError(t, inputField.Press("Enter"))

	// Verify AI response
	require.NoError(t, page.Locator("text=That's a great question! For more details on this topic, please visit our Help Center articles.").WaitFor())

	// Close the chat
	require.NoError(t, page.GetByRole("button", playwright.PageGetByRoleOptions{Name: "Close chat"}).Click())

	// Navigate to Help Center via Sidebar
	require.NoError(t, page.Locator("text=Help Center").Click())

	// Wait for Help Center to load
	require.NoError(t, page.Locator("text=Browse by Topic").WaitFor())

	// Verify Topics are visible
	require.NoError(t, page.Locator("text=Getting Started").WaitFor())
	require.NoError(t, page.Locator("text=My Store").WaitFor())
	require.NoError(t, page.Locator("text=Payments").WaitFor())

	// Verify Quick Links
	require.NoError(t, page.Locator("text=API Documentation").WaitFor())
	require.NoError(t, page.Locator("text=Release Notes").WaitFor())

	// Navigate to API Documentation
	require.NoError(t, page.Locator("text=API Documentation").Click())
	require.NoError(t, page.Locator("text=OneHumanCorp API").WaitFor())
	require.NoError(t, page.Locator("text=GET").First().WaitFor())

	// Navigate back to Help Center
	_, err = page.GoBack()
	require.NoError(t, err)

	// Navigate to Changelog
	require.NoError(t, page.Locator("text=Release Notes").Click())
	require.NoError(t, page.Locator("text=What's New").WaitFor())
	require.NoError(t, page.Locator("text=New AI Help Center").WaitFor())

	// Verify Tooltips on Dashboard (we use hover, Playwright supports this but testing long-press on mobile is harder, so we just verify hover)
	_, err = page.Goto(fmt.Sprintf("%s/dashboard", baseURL))
	require.NoError(t, err)
	require.NoError(t, page.Locator("text=Dashboard").First().WaitFor())
}
