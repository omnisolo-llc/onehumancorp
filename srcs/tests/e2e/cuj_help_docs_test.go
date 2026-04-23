package e2e

import (
	"testing"

	"github.com/playwright-community/playwright-go"
	"github.com/stretchr/testify/require"
)

func TestHelpDocsE2E(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Wait for dashboard to load
	require.NoError(t, page.Locator("text=Dashboard").WaitFor(playwright.LocatorWaitForOptions{
		State:   playwright.WaitForSelectorStateVisible,
		Timeout: playwright.Float(10000),
	}))

	// Click on the Help Center link in the navigation
	require.NoError(t, page.Locator("text=Help Center").Click())

	// Wait for Help Center to load
	require.NoError(t, page.Locator("text=Getting Started").First().WaitFor(playwright.LocatorWaitForOptions{
		State:   playwright.WaitForSelectorStateVisible,
		Timeout: playwright.Float(5000),
	}))

	// Click on Getting Started article
	require.NoError(t, page.Locator("text=Getting Started").First().Click())

	// Wait for article content
	require.NoError(t, page.Locator("text=Welcome to OneHumanCorp!").WaitFor(playwright.LocatorWaitForOptions{
		State:   playwright.WaitForSelectorStateVisible,
		Timeout: playwright.Float(5000),
	}))

	// Check for video tutorial section
	require.NoError(t, page.Locator("text=Video Tutorial").WaitFor(playwright.LocatorWaitForOptions{
		State:   playwright.WaitForSelectorStateVisible,
		Timeout: playwright.Float(5000),
	}))

	// Click Ask our AI Support Agent button
	require.NoError(t, page.Locator("text=Ask our AI Support Agent").Click())

	// Check if chat bottom sheet is open
	require.NoError(t, page.Locator("text=AI Support Agent").WaitFor(playwright.LocatorWaitForOptions{
		State:   playwright.WaitForSelectorStateVisible,
		Timeout: playwright.Float(5000),
	}))

	// Give time to stabilize
}
