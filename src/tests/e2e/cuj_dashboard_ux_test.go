// Copyright 2026 Author(s) of OHC
// SPDX-License-Identifier: Apache-2.0

package e2e

import (
	"testing"
	"github.com/stretchr/testify/assert"
	playwright "github.com/playwright-community/playwright-go"
)

func TestDashboardDisplaysPlainLanguageMetrics(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Wait for dashboard to load
	err := page.WaitForURL("**/dashboard")
	assert.NoError(t, err)

	// Verify "AI Staff" is visible
	aiStaff := page.Locator("text=AI Staff")
	err = aiStaff.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
		Timeout: playwright.Float(10000),
	})
	assert.NoError(t, err)

	// Verify "Active Orders" is visible
	activeOrders := page.Locator("text=Active Orders")
	err = activeOrders.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
		Timeout: playwright.Float(10000),
	})
	assert.NoError(t, err)

	// Verify "Upcoming Bookings" is visible
	upcomingBookings := page.Locator("text=Upcoming Bookings")
	err = upcomingBookings.WaitFor(playwright.LocatorWaitForOptions{
		State: playwright.WaitForSelectorStateVisible,
		Timeout: playwright.Float(10000),
	})
	assert.NoError(t, err)
}
