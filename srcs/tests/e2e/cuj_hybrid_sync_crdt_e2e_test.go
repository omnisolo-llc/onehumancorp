// Copyright 2026 Author(s) of OHC
// SPDX-License-Identifier: Apache-2.0

package e2e

import (
	"strings"
	"testing"

	playwright "github.com/playwright-community/playwright-go"
)

func TestHybridSyncCRDTDeltasEndToEnd(t *testing.T) {
	page := newPage(t)
	if page == nil {
		t.Skip("Browser not available")
	}
	defer page.Close()

	// E2E Standard: Every E2E test MUST start from the home page after user login via the UI
	loginAsAdmin(t, page)

	// Wait for home page to fully load
	expectTitleContainsLocal(t, page, "Dashboard")

	// Navigate the full feature flow by clicking links and buttons on the UI
	navigateTo(t, page, "Settings")

	// Wait for the settings page to render
	expectTitleContainsLocal(t, page, "Settings")

	// Trigger the hybrid sync action using the Semantics label
	syncBtn := page.Locator("[aria-label='Trigger Hybrid Sync'], text=Trigger Hybrid Sync").First()
	err := syncBtn.WaitFor(playwright.LocatorWaitForOptions{
		Timeout: playwright.Float(5000),
	})
	if err != nil {
		t.Fatalf("failed to find Trigger Hybrid Sync button: %v", err)
	}

	err = syncBtn.Click(playwright.LocatorClickOptions{
		Force: playwright.Bool(true),
	})
	if err != nil {
		t.Fatalf("failed to click Trigger Hybrid Sync button: %v", err)
	}

	// Verify the final UI state shows the sync was successful (Snackbar text)
	successIndicator := page.Locator("text=Sync successful, [aria-label='Sync successful']").First()
	err = successIndicator.WaitFor(playwright.LocatorWaitForOptions{
		Timeout: playwright.Float(10000), // Wait up to 10s for the sync to complete and UI to show toast
	})
	if err != nil {
		t.Fatalf("failed to find Sync successful text at end: %v", err)
	}
}

func expectTitleContainsLocal(t *testing.T, page playwright.Page, title string) {
	t.Helper()
	err := page.WaitForLoadState()
	if err != nil {
		t.Fatalf("WaitForLoadState failed: %v", err)
	}
	actualTitle, err := page.Title()
	if err != nil {
		t.Fatalf("failed to get page title: %v", err)
	}
	if !strings.Contains(actualTitle, title) {
		t.Fatalf("expected title to contain %q, got %q", title, actualTitle)
	}
}
