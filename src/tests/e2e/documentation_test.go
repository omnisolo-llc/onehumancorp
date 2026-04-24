package e2e

import (
	"testing"
	"time"
)

func TestDocumentationFeatures(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	// 1. Log in using the UI.
	loginAsAdmin(t, page)

	// Wait for home screen.
	err := page.Locator(`[aria-label="Dashboard"]`).WaitFor()
	if err != nil {
		t.Fatalf("Failed to wait for Dashboard: %v", err)
	}

	// 2. Help Chat FAB
	err = page.Locator(`text=Ask anything`).Click()
	if err != nil {
		t.Fatalf("Failed to click Ask anything: %v", err)
	}
	time.Sleep(1 * time.Second)

	visible, _ := page.Locator(`text=Hi! I am your OHC Help Agent. What do you need help with today?`).IsVisible()
	if !visible {
		t.Fatalf("Help Agent chat bubble not visible")
	}

	// 3. Navigation to Help Center
	// We need to click outside to close the bottom sheet
	page.Mouse().Click(10, 10)
	time.Sleep(500 * time.Millisecond)

	err = page.Locator(`text=Help Center`).Click()
	if err != nil {
		t.Fatalf("Failed to click Help Center: %v", err)
	}
	time.Sleep(1 * time.Second)

	visible, _ = page.Locator(`text=How can we help you?`).IsVisible()
	if !visible {
		t.Fatalf("Help Center title not visible")
	}
	visible, _ = page.Locator(`text=Video Tutorials`).IsVisible()
	if !visible {
		t.Fatalf("Video Tutorials not visible")
	}

	// 4. Navigation to Release Notes
	err = page.Locator(`text=Release Notes`).Click()
	if err != nil {
		t.Fatalf("Failed to click Release Notes: %v", err)
	}
	time.Sleep(1 * time.Second)

	visible, _ = page.Locator(`text=What's New in OHC`).IsVisible()
	if !visible {
		t.Fatalf("Release Notes title not visible")
	}
}
