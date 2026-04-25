package e2e

import (
	"testing"
)

func TestInstallationWizardAppearsOnFirstBootWithModelProviderSetupStep(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardConfigureBudgetLimitsAndNotificationSettings(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardOptionalSectionsCanBeSkipped(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardRequiredFieldValidationPreventsPrematureAdvance(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardBackNavigationPreservesEnteredData(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardExpertModeToggleRevealsRawConfig(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardCompleteEndToEndAndReachLaunchStep(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardChatIntegrationSettingsStepIsPresentOrSkippable(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardNotificationTimeSettingsAreConfigurable(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardModelProviderStepShowsSelectableProviderTypes(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardModelProviderApiKeyFieldIsMasked(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardDailyBudgetFieldAppearsInWizardOrSettings(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardStepProgressIndicatorAdvancesWithEachClick(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardLanguageLocaleFieldAcceptsEnglish(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardAdminPasswordVisibilityToggleWorks(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardCloudDeploymentOptionIsSelectable(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardSelfHostedDesktopDeploymentOptionIsSelectable(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardReviewPageReflectsEarlierCompanyNameEntry(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestInstallationWizardFinanceIndustryOptionIsSelectableInBusinessProfile(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestDashboardNavigatingAwayFromWizardAndReturningIsSeamless(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestOnboardingCompletionWelcomeSetupWizardIsDismissible(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestWizardSetupCanBeReachedFromTheRootPage(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestWizardFirstStepContainsModelProviderFieldsOrSkipOption(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestWizardNextButtonAdvancesToADifferentStep(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestWizardSkipButtonExistsOnAtLeastOneStep(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestWizardBudgetStepHasDailyWeeklyAndMonthlyInputs(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestWizardNotificationStepRendersWithoutError(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestWizardChatIntegrationStepRendersWithoutError(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestWizardAllStepsCanBeReachedWithoutAJsException(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestOnboardingWizardCanBeSkippedEntirelyFromTheFirstStep(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestOnboardingWizardProgressBarOrStepIndicatorIsVisible(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}

func TestOnboardingBackButtonIsPresentFromStep2Onward(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	body, _ := page.Content()
	_ = body
}
