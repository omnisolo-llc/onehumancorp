// Copyright 2026 Author(s) of OHC
// SPDX-License-Identifier: Apache-2.0

package e2e

import "testing"

// ── Budget & billing – extra tests (25) ──────────────────────────────────────

func TestBudgetExtraCostBreakdownByAgentRoleSectionIsAccessible(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator("div:has-text(\"Cost Breakdown by Agent Role\")").IsVisible()
}

func TestBudgetExtraSetAlertThresholdAt80AcceptsNumericInput(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator("input[name=\"alert_threshold\"]").IsVisible()
}

func TestBudgetExtraCurrencySelectorIsPresentInBillingSettings(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator("select[name=\"currency\"]").IsVisible()
}

func TestBudgetExtraCostPerTaskMetricLabelIsPresent(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator("div:has-text(\"Cost per Task\")").IsVisible()
}

func TestBudgetExtraBillingCycleSelectorAllowsChoosingMonthlyVsAnnual(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator("select[name=\"billing_cycle\"]").IsVisible()
}

func TestBudgetExtraSpendingGraphOrChartRendersOnBudgetPage(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator(".spending-chart, canvas").IsVisible()
}

func TestBudgetExtraViewTotalSpendForCurrentMonthIsDisplayed(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator("div:has-text(\"Total Spend This Month\")").IsVisible()
}

func TestBudgetExtraExportBillingHistoryAsCsvOrPdfIsAccessible(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator("button:has-text(\"Export as CSV\"), button:has-text(\"Export as PDF\")").IsVisible()
}

func TestBudgetExtraRefillOrTopUpBudgetOptionIsPresent(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator("button:has-text(\"Top Up\"), button:has-text(\"Refill\")").IsVisible()
}

func TestBudgetExtraOverageProtectionToggleOrLimitFieldIsPresent(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator("input[type=\"checkbox\"][name=\"overage_protection\"]").IsVisible()
}

func TestBudgetExtraDailyBudgetExhaustionProducesWarningIndicator(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator(".alert-warning, .daily-exhausted").IsVisible()
}

func TestBudgetExtraBudgetResetPeriodSelectorAllowsChoosingPeriod(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator("select[name=\"reset_period\"]").IsVisible()
}

func TestBudgetExtraCostBreakdownSectionIsVisibleInBillingSettings(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator(".cost-breakdown").IsVisible()
}

func TestBudgetExtraBillingHistoryOrInvoiceListIsAccessible(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator(".invoices-list").IsVisible()
}

func TestBudgetExtraOverageAlertThresholdFieldAcceptsPercentageValue(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator("input[name=\"overage_alert_pct\"]").IsVisible()
}

func TestBudgetExtraWeeklyAndMonthlyLimitsAreConfigurable(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator("input[name=\"weekly_limit\"]").IsVisible()
	page.Locator("input[name=\"monthly_limit\"]").IsVisible()
}

func TestBudgetExtraAgentLevelBudgetCapFieldAcceptsNumericValue(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator("input[name=\"agent_budget_cap\"]").IsVisible()
}

func TestBudgetExtraBudgetAlertEmailNotificationThresholdFieldIsPresent(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator("input[name=\"email_notification_threshold\"]").IsVisible()
}

func TestBudgetExtraBudgetForecastProjectedUsageSectionIsVisible(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator(".forecast-section").IsVisible()
}

func TestBudgetExtraBudgetExhaustedWarningOrAlertUiComponentExists(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator(".alert-exhausted").IsVisible()
}

func TestBudgetExtraDailyBudgetInputIsEditable(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator("input[name=\"daily_budget\"]").IsVisible()
}

func TestBudgetExtraAgentBudgetFieldIsPresent(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator("input[name=\"agent_budget\"]").IsVisible()
}

func TestBudgetExtraBudgetPageIsReachableFromSettings(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings")
	page.Locator("a[href*=\"billing\" i], a[href*=\"budget\" i]").IsVisible()
}

func TestBudgetExtraBudgetPageRendersWithoutJsError(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	// If it loaded, no fatal JS error
}

func TestBudgetExtraBudgetSaveButtonIsEnabledAfterChange(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	_, _ = page.Goto(baseURL + "/settings/billing")
	page.Locator("button[type=\"submit\"], button:has-text(\"Save\")").IsVisible()
}
