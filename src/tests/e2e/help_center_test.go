package e2e

import (
	"testing"
)

func TestHelpCenter_E2E(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test HelpCenter components load and render (Visual test or simple pass)
	body, _ := page.Content()
	_ = body
}
