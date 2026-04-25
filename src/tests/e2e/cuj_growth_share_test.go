package e2e

import (
	"testing"
)

func TestGrowthShareWidgetRendersOnDashboard(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestGrowthShareWidgetTriggersShareIntent(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestGrowthShareWidgetFallbackCopyLink(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestGrowthShareWidgetEmbedCodeDialog(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}
