package e2e

import (
	"testing"
)

func TestViralInviteLoop(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)

	// 1. Visit User Management page
	_, err := page.Goto("/user_management")
	if err != nil {
		t.Fatalf("could not go to user_management: %v", err)
	}

	// 2. Click "Share OHC (1 Month Free Pro)" button
	btn := page.Locator("text=Share OHC (1 Month Free Pro)")
	if err := btn.WaitFor(); err != nil {
		t.Fatalf("Invite button not found: %v", err)
	}
	if err := btn.Click(); err != nil {
		t.Fatalf("could not click invite button: %v", err)
	}

	// Wait for snackbar
	snackbar := page.Locator("text=Cloud-Bridge invite link copied")
	if err := snackbar.WaitFor(); err != nil {
		t.Fatalf("Snackbar not shown: %v", err)
	}

	// 3. Visit Viral Loop Dashboard
	_, err = page.Goto("/referrals")
	if err != nil {
		t.Fatalf("could not go to referrals: %v", err)
	}

	// Check title
	title := page.Locator("text=Viral Loop Dashboard")
	if err := title.WaitFor(); err != nil {
		t.Fatalf("Dashboard title not found: %v", err)
	}

	// Since we mocked API or use in-memory, we can check if there's any referral code shown
	refCode := page.Locator("text=Ref: xYz8vQ_local_sovereign")
	if err := refCode.WaitFor(); err != nil {
		t.Fatalf("Referral code not found in dashboard: %v", err)
	}
}
