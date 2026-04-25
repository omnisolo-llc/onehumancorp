// Copyright 2026 Author(s) of OHC
// SPDX-License-Identifier: Apache-2.0

package e2e

import (
	"testing"
	"github.com/stretchr/testify/assert"
	playwright "github.com/playwright-community/playwright-go"
)

func TestWelcomeChecklistRendersOnDashboard(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Wait for dashboard to load
	err := page.WaitForURL("**/dashboard")
	assert.NoError(t, err)

	// Wait for the welcome checklist widget
	locator := page.Locator("text=Welcome Checklist")
	err = locator.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	assert.NoError(t, err)

	// Check if the business live checklist item is there
	liveItem := page.Locator("text=Business live")
	err = liveItem.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	assert.NoError(t, err)

	// Verify "Add 3 more products" exists
	addProductItem := page.Locator("text=Add 3 more products")
	err = addProductItem.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	assert.NoError(t, err)
}
