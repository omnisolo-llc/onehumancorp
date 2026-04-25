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
	locator := page.Locator("text='Grow Your Swarm. Maintain Sovereignty.'")
	err = locator.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	assert.NoError(t, err)

	// Wait for the invite link button to be there
	copyBtn := page.Locator("text='Invite Team to Expand Quota'")
	err = copyBtn.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
	})
	assert.NoError(t, err)
}
