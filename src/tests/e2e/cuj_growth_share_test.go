// Copyright 2026 Author(s) of OHC
// SPDX-License-Identifier: Apache-2.0

package e2e

import (
	"testing"
	"github.com/stretchr/testify/assert"
	playwright "github.com/playwright-community/playwright-go"
)

func TestGrowthShareWidgetRendersOnDashboard(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Wait for dashboard to load
	err := page.WaitForURL("**/dashboard")
	assert.NoError(t, err)

	// Wait for the share widget
	locator := page.GetByText("Share my business").First()
	err = locator.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	assert.NoError(t, err)

	// Check if the "Copy Link" button is there
	copyBtn := page.GetByText("Copy Link").First()
	err = copyBtn.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	assert.NoError(t, err)

	// Check if the generic built with OHC tagline is there
	tagline := page.GetByText("built with OHC").First()
	err = tagline.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	assert.NoError(t, err)
}
