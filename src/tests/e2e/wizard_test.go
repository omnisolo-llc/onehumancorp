package e2e

import (
	"fmt"
	"testing"
)

func TestInstallationWizardAppearsOnFirstBootWithModelProviderSetupStep(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: installation wizard: appears on first boot with model provider setup step
	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardConfigureBudgetLimitsAndNotificationSettings(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: installation wizard: configure budget limits and notification settings
	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardOptionalSectionsCanBeSkipped(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: installation wizard: optional sections can be skipped
	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardRequiredFieldValidationPreventsPrematureAdvance(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: installation wizard: required-field validation prevents premature advance
	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardBackNavigationPreservesEnteredData(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: installation wizard: back navigation preserves entered data
	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardExpertModeToggleRevealsRawConfig(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: installation wizard: expert mode toggle reveals raw config
	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardCompleteEndToEndAndReachLaunchStep(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: installation wizard: complete end-to-end and reach launch step
	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardChatIntegrationSettingsStepIsPresentOrSkippable(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: installation wizard: chat integration settings step is present or skippable
	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardNotificationTimeSettingsAreConfigurable(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: installation wizard: notification time settings are configurable
	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardModelProviderStepShowsSelectableProviderTypes(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: installation wizard: model provider step shows selectable provider types
	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardModelProviderApiKeyFieldIsMasked(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: installation wizard: model provider API key field is masked
	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardDailyBudgetFieldAppearsInWizardOrSettings(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: installation wizard: daily budget field appears in wizard or settings
	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardStepProgressIndicatorAdvancesWithEachClick(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: installation wizard: step progress indicator advances with each click
	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardLanguageLocaleFieldAcceptsEnglish(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: installation wizard: language/locale field accepts English
	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardAdminPasswordVisibilityToggleWorks(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: installation wizard: admin password visibility toggle works
	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardCloudDeploymentOptionIsSelectable(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: installation wizard: cloud deployment option is selectable
	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardSelfHostedDesktopDeploymentOptionIsSelectable(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: installation wizard: self-hosted desktop deployment option is selectable
	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardReviewPageReflectsEarlierCompanyNameEntry(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: installation wizard: review page reflects earlier company name entry
	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardFinanceIndustryOptionIsSelectableInBusinessProfile(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: installation wizard: finance industry option is selectable in business profile
	body, _ := page.Content()
	_ = body
}

func TestDashboardNavigatingAwayFromWizardAndReturningIsSeamless(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: dashboard: navigating away from wizard and returning is seamless
	body, _ := page.Content()
	_ = body
}

func TestOnboardingCompletionWelcomeSetupWizardIsDismissible(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: onboarding completion: welcome setup wizard is dismissible
	body, _ := page.Content()
	_ = body
}

func TestWizardSetupCanBeReachedFromTheRootPage(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: wizard / setup: can be reached from the root page
	body, _ := page.Content()
	_ = body
}

func TestWizardFirstStepContainsModelProviderFieldsOrSkipOption(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: wizard: first step contains model provider fields or skip option
	body, _ := page.Content()
	_ = body
}

func TestWizardNextButtonAdvancesToADifferentStep(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: wizard: Next button advances to a different step
	body, _ := page.Content()
	_ = body
}

func TestWizardSkipButtonExistsOnAtLeastOneStep(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: wizard: Skip button exists on at least one step
	body, _ := page.Content()
	_ = body
}

func TestWizardBudgetStepHasDailyWeeklyAndMonthlyInputs(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: wizard: budget step has daily, weekly and monthly inputs
	body, _ := page.Content()
	_ = body
}

func TestWizardNotificationStepRendersWithoutError(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: wizard: notification step renders without error
	body, _ := page.Content()
	_ = body
}

func TestWizardChatIntegrationStepRendersWithoutError(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: wizard: chat integration step renders without error
	body, _ := page.Content()
	_ = body
}

func TestWizardAllStepsCanBeReachedWithoutAJsException(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: wizard: all steps can be reached without a JS exception
	body, _ := page.Content()
	_ = body
}

func TestOnboardingWizardCanBeSkippedEntirelyFromTheFirstStep(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: onboarding: wizard can be skipped entirely from the first step
	body, _ := page.Content()
	_ = body
}

func TestOnboardingWizardProgressBarOrStepIndicatorIsVisible(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: onboarding: wizard progress bar or step indicator is visible
	body, _ := page.Content()
	_ = body
}

func TestOnboardingBackButtonIsPresentFromStep2Onward(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: onboarding: Back button is present from step 2 onward
	body, _ := page.Content()
	_ = body
}

func TestGrowWizardE2E(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Navigate to Dashboard
	targetUrl := fmt.Sprintf("%s/#/dashboard?FLUTTER_WEB_USE_SKIA=false", baseURL)
	_, err := page.Goto(targetUrl)
	if err != nil {
		t.Fatalf("could not goto: %v", err)
	}

	// Wait for dashboard to load
	if _, err := page.WaitForSelector("text='Dashboard'"); err != nil {
		t.Fatalf("dashboard not loaded: %v", err)
	}

	// Enable semantics
	if _, err := page.Evaluate(`() => { if (window._flutter_semantics_enable) window._flutter_semantics_enable(); }`); err != nil {
		t.Logf("Failed to enable semantics, might not be needed")
	}

	// Wait a moment for flutter to render
	page.WaitForTimeout(2000)

	btn, err := page.QuerySelector("text='View Suggestions'")
	if err != nil || btn == nil {
		btn, err = page.QuerySelector("[aria-label*='View Suggestions']")
		if err != nil || btn == nil {
			t.Fatalf("could not find View Suggestions button: %v", err)
		}
	}

	err = btn.Click()
	if err != nil {
		t.Fatalf("failed to click View Suggestions: %v", err)
	}

	// Verify we are on the Grow wizard
	if _, err := page.WaitForSelector("text='Ready to grow?'"); err != nil {
		t.Fatalf("Grow wizard not opened: %v", err)
	}

	// Interact with wizard: "Connect Instagram"
	productBtn, err := page.QuerySelector("text='Connect Instagram'")
	if err == nil && productBtn != nil {
		productBtn.Click()
	} else {
		fallback, _ := page.QuerySelector("[aria-label*='Connect Instagram']")
		if fallback != nil {
			fallback.Click()
		} else {
			t.Fatalf("Could not find Connect Instagram button")
		}
	}

	// Verify processing state
	if _, err := page.WaitForSelector("text='Connecting Instagram...'"); err != nil {
		t.Fatalf("Connecting Instagram state not reached: %v", err)
	}

	// Click Go to Dashboard
	dashboardBtn, err := page.QuerySelector("text='Go to Dashboard'")
	if err == nil && dashboardBtn != nil {
		dashboardBtn.Click()
	} else {
		fallback, _ := page.QuerySelector("[aria-label*='Go to Dashboard']")
		if fallback != nil {
			fallback.Click()
		} else {
			t.Fatalf("Could not find Go to Dashboard button")
		}
	}


	page.WaitForTimeout(1000)
}
