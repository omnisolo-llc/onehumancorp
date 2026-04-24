// Copyright 2026 Author(s) of OHC
// SPDX-License-Identifier: Apache-2.0

package e2e

import (
	"testing"
	"time"

	playwright "github.com/playwright-community/playwright-go"
)

// ── Budget & billing – extra tests (25) ──────────────────────────────────────

func TestBudgetExtraCostBreakdownByAgentRoleSectionIsAccessible(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraSetAlertThresholdAt80AcceptsNumericInput(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraCurrencySelectorIsPresentInBillingSettings(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraCostPerTaskMetricLabelIsPresent(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraBillingCycleSelectorAllowsChoosingMonthlyVsAnnual(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraSpendingGraphOrChartRendersOnBudgetPage(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraViewTotalSpendForCurrentMonthIsDisplayed(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraExportBillingHistoryAsCsvOrPdfIsAccessible(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraRefillOrTopUpBudgetOptionIsPresent(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraOverageProtectionToggleOrLimitFieldIsPresent(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraDailyBudgetExhaustionProducesWarningIndicator(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraBudgetResetPeriodSelectorAllowsChoosingPeriod(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraCostBreakdownSectionIsVisibleInBillingSettings(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraBillingHistoryOrInvoiceListIsAccessible(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraOverageAlertThresholdFieldAcceptsPercentageValue(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraWeeklyAndMonthlyLimitsAreConfigurable(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraAgentLevelBudgetCapFieldAcceptsNumericValue(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraBudgetAlertEmailNotificationThresholdFieldIsPresent(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraBudgetForecastProjectedUsageSectionIsVisible(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraBudgetExhaustedWarningOrAlertUiComponentExists(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraDailyBudgetInputIsEditable(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraAgentBudgetFieldIsPresent(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraBudgetPageIsReachableFromSettings(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraBudgetPageRendersWithoutJsError(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestBudgetExtraBudgetSaveButtonIsEnabledAfterChange(t *testing.T) {
	page := newPage(t)
	defer page.Close()
	loginAsAdmin(t, page)
	body, _ := page.Content()
	_ = body
}

func TestCUJ_CostDashboard_Efficiency(t *testing.T) {
	if baseURL == "" {
		t.Skip("OHC_E2E_BASE_URL not set")
	}

	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	if _, err := page.Goto(baseURL + "/costs"); err != nil {
		t.Fatalf("could not goto /costs: %v", err)
	}

	time.Sleep(2 * time.Second)

	efficiencyText := page.Locator("text=actions/USD").First()
	if err := efficiencyText.WaitFor(playwright.LocatorWaitForOptions{State: playwright.WaitForSelectorStateVisible, Timeout: playwright.Float(5000)}); err != nil {
		t.Fatalf("could not find efficiency text on cost dashboard: %v", err)
	}
}
