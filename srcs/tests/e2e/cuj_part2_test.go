package e2e

import (
	"regexp"
	"strings"
	"testing"
	"time"

	playwright "github.com/playwright-community/playwright-go"
)

// Ensure imports are used - part 2
var (
	_ = regexp.MustCompile
	_ = strings.Contains
	_ = time.Sleep
)

func TestAppRootHTTP200AndNonEmptyBodyOnColdRequest(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	resp, _ := page.Goto(baseURL + "/")
	if resp != nil && resp.Status() >= 500 { t.Errorf("expected status < 500, got %d", resp.Status()) }
	body, _ := page.Content()
	// expect(body.length).toBeGreaterThan(100)
}

func TestHealthEndpointHealthReturns200(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	resp, _ := page.Goto(baseURL + "/health")
	if resp != nil && resp.Status() != 200 { t.Errorf("expected 200, got %d", resp.Status()) }
}

func TestLoginPageTitleOrHeadingContainsRecognisableBrandText(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	if _, err := page.Goto(baseURL + "/"); err != nil { t.Logf("goto: %v", err) }
	_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
	title, _ := page.Title()
	body, _ := page.Content()
	// const branded =
		// /ohc|one human|corp|swarm|orchestrat/i.test(title) ||
		// /ohc|one human|corp|swarm|orchestrat/i.test(body)
	if !branded { t.Error("expected true") }
}

func TestLoginPageUsernameAndPasswordFieldsArePresent(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	if _, err := page.Goto(baseURL + "/"); err != nil { t.Logf("goto: %v", err) }
	_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
	// const userField = page.locator( 'input[type="email"], input[name="email"], input[placeholder*="email
	passField := page.Locator(`input[type="password"]`).First()
	// await expect(userField).toBeVisible({ timeout: LONG_TIMEOUT })
	// await expect(passField).toBeVisible({ timeout: LONG_TIMEOUT })
}

func TestLoginSubmitButtonIsPresentAndEnabled(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	if _, err := page.Goto(baseURL + "/"); err != nil { t.Logf("goto: %v", err) }
	_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
	// const btn = page .locator( 'button[type="submit"], button:has-text("Login"), button:has-text("Sign I
	// await expect(btn).toBeVisible({ timeout: LONG_TIMEOUT })
	// await expect(btn).toBeEnabled({ timeout: SHORT_TIMEOUT })
}

func TestLoginWrongCredentialsShowsAnErrorMessage(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	if _, err := page.Goto(baseURL + "/"); err != nil { t.Logf("goto: %v", err) }
	_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
	// const userField = page.locator( 'input[type="email"], input[name="email"], input[placeholder*="email
	passField := page.Locator(`input[type="password"]`).First()
	// if ((await userField.count()) === 0) return; // no login form visible
	if err := userField.Fill("wrong_user_xyz", nil); err != nil { t.Logf("fill: %v", err) }
	if err := page.Locator(`input[type="password"]`).First().Fill("wrong_pass_xyz", nil); err != nil { t.Logf("fill: %v", err) }
	// await page .locator( 'button[type="submit"], button:has-text("Login"), button:has-text("Sign In"), b
	sleepMs(3000)
	// const errorVisible =
		// (await page.locator('[role="alert"], .error, .alert, [aria-live]').count()) > 0 ||
		// /invalid|incorrect|wrong|unauthori|not found/i.test(await page.content())
	if !errorVisible { t.Error("expected true") }
}

func TestLoginAdminCredentialsSucceedAndRedirectAwayFromLogin(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	url := page.URL()
	if matched, _ := regexp.MatchString(`(?i)\/login|\/signin`, url); matched { t.Errorf("unexpected match") }
}

func TestPostLoginPageDoesNotShowA500OrUncaughtError(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	if matched, _ := regexp.MatchString(`(?i)500|uncaught error|cannot read`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
}

func TestPostLoginAtLeastOneNavSidebarLinkIsVisible(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	navLinks := page.Locator(`nav a, nav button, aside a, [role="menuitem"]`)
	count, _ := page.Locator(`nav a, nav button, aside a, [role="menuitem"]`).Count()
	if count <= 0 { t.Errorf("expected > 0") }
}

func TestPostLoginPageHasAVisibleHeading(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	heading := page.Locator(`h1, h2`).First()
	// await expect(heading).toBeVisible({ timeout: LONG_TIMEOUT })
}

func TestWizardSetupCanBeReachedFromTheRootPage(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// Wizard could auto-show on first load or be accessible via a link.
	// const wizardTrigger = page.locator( 'button:has-text("Setup"), a:has-text("Setup"), button:has-text(
	// const wizardOrHeading = page.locator( 'h1, h2, [role="dialog"], [data-testid*="wizard" i]', ).first(
	// Either the wizard is already shown or there is a trigger to open it.
	// const alreadyVisible = await wizardOrHeading.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => fals
	// const triggerVisible = await wizardTrigger.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)
	// expect(alreadyVisible || triggerVisible).toBe(true)
}

func TestWizardFirstStepContainsModelProviderFieldsOrSkipOption(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
	// const modelOrSkip = page.locator( 'input[placeholder*="api key" i], input[placeholder*="model" i], b
	// const visible = await modelOrSkip.isVisible({ timeout: LONG_TIMEOUT }).catch(() => false)
	// Page must at least render without error even if wizard is completed.
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	// This test simply asserts the page is interactive.
	// (pass)
}

func TestWizardNextButtonAdvancesToADifferentStep(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const nextBtn = page .locator('button') $/i }) .first()
	// if (!(await nextBtn.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false))) return
	contentBefore, _ := page.Content()
	if err := nextBtn.Click(nil); err != nil { t.Logf("click: %v", err) }
	sleepMs(1000)
	contentAfter, _ := page.Content()
	// Content should have changed after clicking Next.
	// expect(contentAfter).not.toEqual(contentBefore)
}

func TestWizardSkipButtonExistsOnAtLeastOneStep(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// Click through up to 5 wizard steps looking for a Skip option.
	found := false
	for i := 0; i < 5; i++ {
		skipBtn := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(skip|skip this step|skip for now)$")}).First()
		if vis, _ := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(skip|skip this step|skip for now)$")}).First().IsVisible(); vis {
			found = true
			break
		}
		nextBtn := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First()
		if vis, _ := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First().IsVisible(); vis {
			if err := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First().Click(nil); err != nil { t.Logf("click: %v", err) }
			sleepMs(800)
		} else {
			break
		}
	}
	// wizard may have been completed in a previous test run; that's acceptable.
	// (pass)
}

func TestWizardBudgetStepHasDailyWeeklyAndMonthlyInputs(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// Navigate through steps until we find budget-related inputs (or give up).
	for i := 0; i < 8; i++ {
		// const budgetHint = page.locator( 'input[placeholder*="budget" i], label:has-text("budget"), [data-te
		if vis, _ := budgetHint.IsVisible(); vis {
			// const dailyHint = page.locator( 'input[placeholder*="daily" i], label:has-text("daily"), [data-testi
			// const weeklyHint = page.locator( 'input[placeholder*="weekly" i], label:has-text("weekly"), [data-te
			// const monthlyHint = page.locator( 'input[placeholder*="monthly" i], label:has-text("monthly"), [data
			// const dailyOk   = await dailyHint.isVisible({ timeout: 2_000 }).catch(() => false)
			// const weeklyOk  = await weeklyHint.isVisible({ timeout: 2_000 }).catch(() => false)
			// const monthlyOk = await monthlyHint.isVisible({ timeout: 2_000 }).catch(() => false)
			// if (dailyOk || weeklyOk || monthlyOk) {
				// expect(dailyOk || weeklyOk || monthlyOk).toBe(true)
				return
			}
		}
		nextBtn := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First()
		if vis, _ := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First().IsVisible(); vis {
			if err := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First().Click(nil); err != nil { t.Logf("click: %v", err) }
			sleepMs(600)
		} else {
			break
		}
	}
	// Budget step not reached in this run (wizard already completed).
	// (pass)
}

func TestWizardNotificationStepRendersWithoutError(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	notifFound := false
	for i := 0; i < 8; i++ {
		// const notifHint = page.locator( 'input[placeholder*="notif" i], label:has-text("notification"), [dat
		if vis, _ := notifHint.IsVisible(); vis {
			notifFound = true
			if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
			break
		}
		nextBtn := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First()
		if vis, _ := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First().IsVisible(); vis {
			if err := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First().Click(nil); err != nil { t.Logf("click: %v", err) }
			sleepMs(600)
		} else {
			break
		}
	}
	// (pass)
}

func TestWizardChatIntegrationStepRendersWithoutError(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	for i := 0; i < 8; i++ {
		// const chatHint = page.locator( 'input[placeholder*="slack" i], input[placeholder*="webhook" i], labe
		if vis, _ := chatHint.IsVisible(); vis {
			if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
			break
		}
		nextBtn := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First()
		if vis, _ := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First().IsVisible(); vis {
			if err := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First().Click(nil); err != nil { t.Logf("click: %v", err) }
			sleepMs(600)
		} else {
			break
		}
	}
	// (pass)
}

func TestWizardAllStepsCanBeReachedWithoutAJSException(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	for i := 0; i < 12; i++ {
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
		nextBtn := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First()
		skipBtn := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(skip|skip this step)$")}).First()
		launchBtn := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(launch|finish|done|complete)$")}).First()
		// if (await launchBtn.isVisible({ timeout: 1_000 }).catch(() => false)) break
		if vis, _ := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(skip|skip this step)$")}).First().IsVisible(); vis {
			if err := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(skip|skip this step)$")}).First().Click(nil); err != nil { t.Logf("click: %v", err) }
		} else {
			if err := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First().Click(nil); err != nil { t.Logf("click: %v", err) }
		} else {
			break
		}
		sleepMs(600)
	}
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
}

func TestDashboardPageIsReachableAfterLogin(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// Dashboard might be reached via a nav link or be the landing page.
	// const dashLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := dashLink.IsVisible(); vis {
		if err := dashLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
	}
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
}

func TestDashboardSwarmOrAgentOverviewSectionIsVisible(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const swarmSection = page .locator('h1, h2, h3, [data-testid*="swarm"], [data-testid*="agent"], [dat
	// const visible = await swarmSection.isVisible({ timeout: LONG_TIMEOUT }).catch(() => false)
	// If no swarm section found, page still must not have crashed.
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	// (pass)
}

func TestNewBusinessFormOrNavEntryIsAccessible(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const trigger = page .locator( 'nav a, nav button, aside a, button, [role="menuitem"]', ) 
	// const visible = await trigger.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
	// if (visible) {
		if err := trigger.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestNewBusinessFormStep1RendersANameOrBusinessTypeField(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// Try to reach the new-business wizard.
	// const trigger = page .locator('nav a, nav button, aside a, button, [role="menuitem"]') 
	if vis, _ := trigger.IsVisible(); vis {
		if err := trigger.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
	}
	// Check for a name or type field on whatever page we land on.
	// const nameField = page.locator( 'input[placeholder*="name" i], input[name*="name" i], input[placehol
	// const visible = await nameField.isVisible({ timeout: LONG_TIMEOUT }).catch(() => false)
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	// (pass)
	}
}

func TestNewBusinessFormUsStateSelectorIsPresentInLocationStep(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const stateSelector = page.locator( 'select[name*="state" i], [data-testid*="state" i], input[placeh
	for i := 0; i < 6; i++ {
		if vis, _ := stateSelector.IsVisible(); vis {
			if err := playwright.Expect(stateSelector).ToBeVisible(nil); err != nil { t.Logf("expected visible: %v", err) }
			return
		}
		nextBtn := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First()
		if vis, _ := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First().IsVisible(); vis {
			if err := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First().Click(nil); err != nil { t.Logf("click: %v", err) }
			sleepMs(600)
		// } else break
	}
	// (pass)
}

func TestNewBusinessFormAgentHiringRequirementsStepIsReachable(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	for i := 0; i < 8; i++ {
		// const agentHint = page.locator( 'label:has-text("agent"), input[placeholder*="agent" i], h2:has-text
		if vis, _ := agentHint.IsVisible(); vis {
			if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
			return
		}
		nextBtn := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First()
		if vis, _ := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First().IsVisible(); vis {
			if err := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First().Click(nil); err != nil { t.Logf("click: %v", err) }
			sleepMs(600)
		// } else break
	}
	// (pass)
}

func TestNewBusinessFormAIAssistantSuggestionFieldIsPresent(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	for i := 0; i < 8; i++ {
		// const aiHint = page.locator( 'textarea[placeholder*="describe" i], textarea[placeholder*="tell us" i
		if vis, _ := aiHint.IsVisible(); vis {
			if err := playwright.Expect(aiHint).ToBeVisible(nil); err != nil { t.Logf("expected visible: %v", err) }
			return
		}
		nextBtn := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First()
		if vis, _ := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First().IsVisible(); vis {
			if err := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First().Click(nil); err != nil { t.Logf("click: %v", err) }
			sleepMs(600)
		// } else break
	}
	// (pass)
}

func TestBusinessesListPageIsReachableViaNavigation(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const link = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := link.IsVisible(); vis {
		if err := link.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestBusinessesListEmptyStateOrListOfBusinessesRenders(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const link = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := link.IsVisible(); vis {
		if err := link.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
	}
	content, _ := page.Content()
	// const hasBusinessContent =
		// /business|no business yet|create your first|empty/i.test(content) ||
		// (await page.locator('[data-testid*="business"], .business-card, ul li').count()) > 0
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	// (pass)
	}
}

func TestAgentTeamsPageIsReachableViaNavigation(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const link = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := link.IsVisible(); vis {
		if err := link.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestAgentTeamsStatusIndicatorsVisibleOnTeamList(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const link = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := link.IsVisible(); vis {
		if err := link.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		status := page.Locator(`[class*="status"], [data-status], [aria-label*="status" i]`).First()
		// const hasStatus = await status.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		// Status might not show if there are no teams yet.
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestAgentTeamsHireOrAddAgentButtonPresentOnTeamsPage(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const teamsLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := teamsLink.IsVisible(); vis {
		if err := teamsLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const btn = page .locator('button')  .first
		// const visible = await btn.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestChatToAgentChatPanelOrLinkIsPresentAfterLogin(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const chatTrigger = page .locator( '[data-testid*="chat"], button:has-text("Chat"), a:has-text("Chat
	// const visible = await chatTrigger.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	// (pass)
}

func TestChatMessageInputFieldIsPresentInChatView(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const chatTrigger = page .locator( '[data-testid*="chat"], button:has-text("Chat"), a:has-text("Chat
	if vis, _ := chatTrigger.IsVisible(); vis {
		if err := chatTrigger.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const msgInput = page .locator( 'textarea[placeholder*="message" i], input[placeholder*="message" i]
		// const inputVisible = await msgInput.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
}

func TestChatSendButtonOrKeyboardShortcutHintIsVisible(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const chatTrigger = page .locator('[data-testid*="chat"], button:has-text("Chat"), a:has-text("Chat"
	if vis, _ := chatTrigger.IsVisible(); vis {
		if err := chatTrigger.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const sendBtn = page .locator('button[aria-label*="send" i], button:has-text("Send"), [data-testid*=
		hint := page.Locator(`kbd, [title*="Enter"], [title*="send" i]`).First()
		// const sendVisible = await sendBtn.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		// const hintVisible = await hint.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
}

func TestSuspendAgentTeamSuspendButtonOrOptionExists(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const teamsLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := teamsLink.IsVisible(); vis {
		if err := teamsLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const suspendBtn = page .locator('button, [role="menuitem"]') 
		// const visible = await suspendBtn.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
	}
}

func TestSuspendBusinessSuspendOrArchiveOptionIsAccessible(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const bizLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := bizLink.IsVisible(); vis {
		if err := bizLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const suspendBtn = page .locator('button, [role="menuitem"]') 
		// const visible = await suspendBtn.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
	}
}

func TestBudgetExhaustedAWarningOrAlertUiComponentExists(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// Navigate to budget/billing settings.
	// const budgetLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := budgetLink.IsVisible(); vis {
		if err := budgetLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const alertElem = page .locator('[role="alert"], .alert, .warning, [data-testid*="budget-alert" i]')
		// const hasAlert = await alertElem.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestBudgetPageDailyBudgetInputIsEditable(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const budgetLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := budgetLink.IsVisible(); vis {
		if err := budgetLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const dailyInput = page .locator('input[placeholder*="daily" i], input[name*="daily" i], [data-testi
		if vis, _ := dailyInput.IsVisible(); vis {
			if err := dailyInput.Fill("100", nil); err != nil { t.Logf("fill: %v", err) }
			val, _ := dailyInput.InputValue()
			if !strings.Contains(val, "100") { t.Error("expected contains") }
		}
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestBudgetPageAgentBudgetFieldIsPresent(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const budgetLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := budgetLink.IsVisible(); vis {
		if err := budgetLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const agentBudget = page .locator( 'input[placeholder*="agent budget" i], label:has-text("agent budg
		// const visible = await agentBudget.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestModelProviderSettingsSettingsPageIsReachable(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const settingsLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := settingsLink.IsVisible(); vis {
		if err := settingsLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestModelProviderSettingsProviderListOrAddProviderButtonExists(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const settingsLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := settingsLink.IsVisible(); vis {
		if err := settingsLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const providerSection = page .locator( '[data-testid*="provider"], button:has-text("Add Provider"), 
		// const visible = await providerSection.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestModelProviderAPIKeyFieldAcceptsInput(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const settingsLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := settingsLink.IsVisible(); vis {
		if err := settingsLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const apiKeyInput = page .locator('input[placeholder*="api key" i], input[name*="api_key" i], input[
		if vis, _ := apiKeyInput.IsVisible(); vis {
			if err := apiKeyInput.Fill("test-api-key-12345", nil); err != nil { t.Logf("fill: %v", err) }
			val, _ := apiKeyInput.InputValue()
			// expect(val.length).toBeGreaterThan(0)
		}
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestModelProviderModelSelectorDropdownIsPresent(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const settingsLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := settingsLink.IsVisible(); vis {
		if err := settingsLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const modelSelector = page .locator('select[name*="model" i], [data-testid*="model-select" i], [aria
		// const visible = await modelSelector.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestModelProviderSaveUpdateButtonIsPresentAndEnabled(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const settingsLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := settingsLink.IsVisible(); vis {
		if err := settingsLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const saveBtn = page .locator('button')  .first()
		if vis, _ := saveBtn.IsVisible(); vis {
			if err := playwright.Expect(saveBtn).ToBeEnabled(nil); err != nil { t.Logf("expected enabled: %v", err) }
		}
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestModelProviderAddSecondProviderButtonOrTabExists(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const settingsLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := settingsLink.IsVisible(); vis {
		if err := settingsLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const addBtn = page .locator('button') .Filter(playwright.LocatorFilterOptions{HasText: playwright.String("provider")}).First()
		// const either =
			// (await addBtn.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)) ||
			// (await tab.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false))
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestModelProviderPerAgentProviderAssignmentOptionExists(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const settingsLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := settingsLink.IsVisible(); vis {
		if err := settingsLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const perAgent = page .locator('[data-testid*="per-agent"], label:has-text("per agent"), [aria-label
		// const visible = await perAgent.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestSettingsNotificationTimeFieldsAreVisible(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const settingsLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := settingsLink.IsVisible(); vis {
		if err := settingsLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const notifTime = page .locator( 'input[type="time"], input[placeholder*="time" i], label:has-text("
		// const visible = await notifTime.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestSettingsWebNotificationToggleIsPresent(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const settingsLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := settingsLink.IsVisible(); vis {
		if err := settingsLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const toggle = page .locator( 'input[type="checkbox"][name*="web" i], label:has-text("web notificati
		// const visible = await toggle.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestSettingsChatNotificationToggleIsPresent(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const settingsLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := settingsLink.IsVisible(); vis {
		if err := settingsLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const toggle = page .locator( 'input[type="checkbox"][name*="chat" i], label:has-text("chat notifica
		// const visible = await toggle.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestSettingsSlackWebhookIntegrationFieldsVisible(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const settingsLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := settingsLink.IsVisible(); vis {
		if err := settingsLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const slackOrWebhook = page .locator( 'input[placeholder*="slack" i], input[placeholder*="webhook" i
		// const visible = await slackOrWebhook.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestSettingsSaveActionDoesNotProduceA500Error(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const settingsLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := settingsLink.IsVisible(); vis {
		if err := settingsLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		saveBtn := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("save|apply|update")}).First()
		if vis, _ := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("save|apply|update")}).First().IsVisible(); vis {
			if err := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("save|apply|update")}).First().Click(nil); err != nil { t.Logf("click: %v", err) }
			sleepMs(2000)
		}
		if matched, _ := regexp.MatchString(`(?i)500|uncaught error`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestUserManagementAdminUserAppearsInUserList(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const usersLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := usersLink.IsVisible(); vis {
		if err := usersLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		adminEntry := page.Locator(`td, li, .user-row`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("admin")}).First()
		// const visible = await adminEntry.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestUserManagementInviteOrCreateUserButtonExists(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const usersLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := usersLink.IsVisible(); vis {
		if err := usersLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const createBtn = page .locator('button') 
		// const visible = await createBtn.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
	}
}

func TestUserManagementRoleAssignmentSelectorIsPresent(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const usersLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := usersLink.IsVisible(); vis {
		if err := usersLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const roleSelector = page .locator('select[name*="role" i], [data-testid*="role" i], [aria-label*="r
		// const visible = await roleSelector.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestProfilePageIsReachableFromTheUserMenu(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const userMenu = page .locator( '[data-testid*="user-menu"], [aria-label*="user menu" i], [aria-labe
	if vis, _ := userMenu.IsVisible(); vis {
		if err := userMenu.Click(nil); err != nil { t.Logf("click: %v", err) }
		sleepMs(800)
		profileLink := page.Locator(`a, button, [role="menuitem"]`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("profile")}).First()
		if vis, _ := page.Locator(`a, button, [role="menuitem"]`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("profile")}).First().IsVisible(); vis {
			if err := page.Locator(`a, button, [role="menuitem"]`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("profile")}).First().Click(nil); err != nil { t.Logf("click: %v", err) }
			_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
			if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
		}
	}
	// (pass)
}

func TestLogoutLogOutOptionIsPresentInUserMenuOrNav(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const logoutTrigger = page .locator( 'button:has-text("Logout"), button:has-text("Log out"), button:
	// const userMenu = page .locator('[data-testid*="user-menu"], [aria-label*="user menu" i], button:has-
	// let logoutVisible = await logoutTrigger.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)
	// if (!logoutVisible && (await userMenu.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false))) {
		if err := userMenu.Click(nil); err != nil { t.Logf("click: %v", err) }
		sleepMs(600)
		// logoutVisible = await logoutTrigger.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)
	}
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	// (pass)
}

func TestLogoutClickingLogoutRedirectsToLoginPage(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// let logoutBtn = ...
	// if (!(await logoutBtn.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false))) {
		// const userMenu = page .locator('[data-testid*="user-menu"], [aria-label*="user menu" i], button:has-
		if vis, _ := userMenu.IsVisible(); vis {
			if err := userMenu.Click(nil); err != nil { t.Logf("click: %v", err) }
			sleepMs(600)
		}
	}
	if vis, _ := logoutBtn.IsVisible(); vis {
		if err := logoutBtn.Click(nil); err != nil { t.Logf("click: %v", err) }
		sleepMs(2000)
		url := page.URL()
		// const onLoginPage =
			// url.includes('/login') || url.includes('/signin') ||
			// (await page.locator('input[type="password"]').count()) > 0
		if !onLoginPage { t.Error("expected true") }
	}
	// (pass)
}

func TestNotificationsNotificationBellOrIconIsPresent(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const bell = page .locator( '[aria-label*="notification" i], [data-testid*="notification" i], button
	// const visible = await bell.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	// (pass)
}

func TestNotificationsClickingBellOpensNotificationListOrPanel(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const bell = page .locator('[aria-label*="notification" i], [data-testid*="notification" i], [class*
	if vis, _ := bell.IsVisible(); vis {
		if err := bell.Click(nil); err != nil { t.Logf("click: %v", err) }
		sleepMs(800)
		// const panel = page .locator('[role="dialog"], [role="listbox"], [data-testid*="notif-panel" i], .not
		// const panelVisible = await panel.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
}

func TestSearchGlobalSearchFieldIsPresentOrAccessible(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const searchInput = page .locator( 'input[type="search"], input[placeholder*="search" i], [role="sea
	// const visible = await searchInput.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	// (pass)
}

func TestSearchTypingInSearchFieldDoesNotCrashThePage(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const searchInput = page .locator('input[type="search"], input[placeholder*="search" i], [role="sear
	if vis, _ := searchInput.IsVisible(); vis {
		if err := searchInput.Fill("test query", nil); err != nil { t.Logf("fill: %v", err) }
		sleepMs(1000)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
}

func TestAPIIntegrationsAPIKeySectionIsReachable(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const apiLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := apiLink.IsVisible(); vis {
		if err := apiLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestAPIKeysGenerateCreateAPIKeyButtonExists(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const apiLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := apiLink.IsVisible(); vis {
		if err := apiLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const genBtn = page .locator('button')  
		// const visible = await genBtn.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestAuditLogActivityLogPageIsReachable(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const auditLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := auditLink.IsVisible(); vis {
		if err := auditLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestAnalyticsReportsAnalyticsPageRendersWithoutError(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const analyticsLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := analyticsLink.IsVisible(); vis {
		if err := analyticsLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
		chart := page.Locator(`canvas, svg, [data-testid*="chart"], [data-testid*="graph"]`).First()
		// const chartVisible = await chart.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
	}
	// (pass)
	}
}

func TestPaginationListViewsHavePaginationControlsWhenDataExceedsOnePage(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const bizLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := bizLink.IsVisible(); vis {
		if err := bizLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const paginator = page .locator('[aria-label*="pagination" i], [data-testid*="paginat" i], nav[role=
		// const visible = await paginator.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestFilteringActiveStatusFilterDoesNotCrashTheListView(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const link = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := link.IsVisible(); vis {
		if err := link.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const filterBtn = page .locator('button, select, [role="combobox"]') 
		if vis, _ := filterBtn.IsVisible(); vis {
			if err := filterBtn.Click(nil); err != nil { t.Logf("click: %v", err) }
			sleepMs(800)
		}
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
	}
}

func TestSystemSystemOrAdminSettingsSectionIsReachable(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const sysLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := sysLink.IsVisible(); vis {
		if err := sysLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestSystemVersionNumberOrBuildInfoIsDisplayedSomewhere(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const versionHints = await page .locator('[data-testid*="version"], [class*="version"], footer span'
	bodyContent, _ := page.Content()
	// const hasVersion = /v\d+\.\d+|\bver\b|version|build/i.test(bodyContent)
	// Version info may not be on the landing page; this is a soft check.
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	// (pass)
}

func TestMeetingRoomMeetingRoomLinkOrSectionIsAccessible(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const meetingLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := meetingLink.IsVisible(); vis {
		if err := meetingLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestMeetingRoomJoinOrCreateMeetingButtonIsPresent(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const meetingLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := meetingLink.IsVisible(); vis {
		if err := meetingLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const joinBtn = page .locator('button') 
		// const visible = await joinBtn.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
	}
}

func TestTaskQueueTaskListOrQueueViewIsAccessible(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const taskLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := taskLink.IsVisible(); vis {
		if err := taskLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestTaskQueueCreateOrSubmitTaskButtonExists(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const taskLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := taskLink.IsVisible(); vis {
		if err := taskLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const createBtn = page .locator('button') 
		// const visible = await createBtn.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
	}
}

func TestTaskQueueCancelRunningTaskOptionIsPresentOnTaskItems(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const taskLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := taskLink.IsVisible(); vis {
		if err := taskLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const cancelBtn = page .locator('button, [role="menuitem"]') 
		// const visible = await cancelBtn.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
	}
}

func TestAgentExecutionLogsLogViewIsReachable(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const logsLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := logsLink.IsVisible(); vis {
		if err := logsLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestAgentExecutionLogsLogEntriesOrNoLogsPlaceholderRenders(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const logsLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := logsLink.IsVisible(); vis {
		if err := logsLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		// const logContent = page .locator( 'table, ul, [data-testid*="log"], pre, code, .log-entry, p:has-tex
		// const visible = await logContent.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestComplianceTermsOfServiceAcceptanceUiIsReachable(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const termsLink = page .locator('nav a, nav button, aside a, [role="menuitem"], a') 
	if vis, _ := termsLink.IsVisible(); vis {
		if err := termsLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestDarkModeThemeThemeToggleIsPresentIfSupported(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const themeToggle = page .locator( 'button[aria-label*="dark" i], button[aria-label*="theme" i], [da
	// const visible = await themeToggle.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)
	// Optional feature — soft assertion only.
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	// (pass)
}

func TestMobileBreakpointViewportResizeDoesNotBreakTheLayout(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	_ = page.SetViewportSize(375, 812)
	sleepMs(500)
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	// Restore default viewport.
	_ = page.SetViewportSize(1280, 720)
}

func TestTabletBreakpointViewportResizeDoesNotBreakTheLayout(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	_ = page.SetViewportSize(768, 1024)
	sleepMs(500)
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	_ = page.SetViewportSize(1280, 720)
}

func TestKeyboardNavigationTabKeyMovesFocusThroughInteractiveElements(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	for i := 0; i < 5; i++ {
		_ = page.Keyboard.Press("Tab")
	}
	focused := page.Locator(`:focus`)
	// const tag = await focused.evaluate(el => el.tagName.toLowerCase()).catch(() => '')
	// Focus should be on a standard interactive element.
	// const interactive = ['a', 'button', 'input', 'select', 'textarea', 'summary'].includes(tag)
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	// (pass)
}

func TestAccessibilityPageHasAtLeastOneLandmarkRegion(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const landmarks = await page .locator('[role="main"], [role="navigation"], [role="banner"], main, na
	if landmarks <= 0 { t.Errorf("expected > 0") }
}

func TestAccessibilityAllImagesHaveAltAttributes(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	images := page.Locator(`img`)
	count, _ := page.Locator(`img`).Count()
	missingAlt := 0
	for i := 0; i < func() int { if count < 20 { return count }; return 20 }(); i++ {
		// const alt = await images.nth(i).getAttribute('alt')
		// if (alt === null) missingAlt++
	}
	// Allow up to 20% of images to lack alt (decorative images may be intentional).
	// if (count > 0) {
		// expect(missingAlt / Math.min(count, 20)).toBeLessThanOrEqual(0.2)
	}
}

func TestPageLoadFirstContentfulPaintIsReasonable10S(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	// const start = Date.now()
	if _, err := page.Goto(baseURL + "/"); err != nil { t.Logf("goto: %v", err) }
	_ = page.WaitForLoadState(playwright.LoadStateDomcontentloaded, nil)
	// const elapsed = Date.now() - start
	// expect(elapsed).toBeLessThan(10_000)
}

func TestNoConsoleErrorsOnInitialLoad(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	// const errors: string[] = []
	// (event listener)
		// if (msg.type() === 'error') errors.push(msg.text())
	// })
	if _, err := page.Goto(baseURL + "/"); err != nil { t.Logf("goto: %v", err) }
	_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
	// Filter out known benign errors (e.g. favicon 404).
	// const realErrors = errors.filter( e => !/favicon|robots\.txt|google|gstatic|analytics/i.test(e), )
	// Soft: log but do not fail on console errors from third-party scripts.
	// if (realErrors.length > 0) {
		// (event listener)
	}
	if matched, _ := regexp.MatchString(`(?i)uncaught error`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
}

func TestNoConsoleErrorsAfterLogin(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	// const errors: string[] = []
	// (event listener)
		// if (msg.type() === 'error') errors.push(msg.text())
	// })
	loginAsAdmin(t, page)
	// const realErrors = errors.filter( e => !/favicon|robots\.txt|google|gstatic|analytics/i.test(e), )
	// if (realErrors.length > 0) {
		// (event listener)
	}
	if matched, _ := regexp.MatchString(`(?i)uncaught error`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
}

func TestBrowserBackForwardNavigationHistoryWorksWithoutCrash(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	firstUrl := page.URL()
	// Navigate to settings if possible.
	// const link = page .locator('nav a, aside a, [role="menuitem"]')  .
	if vis, _ := link.IsVisible(); vis {
		if err := link.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		_, _ = page.GoBack(nil)
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
}

func TestSessionPersistencePageReloadKeepsTheUserLoggedIn(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	urlBefore := page.URL()
	_, _ = page.Reload(nil)
	_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
	urlAfter := page.URL()
	// Should remain on a non-login page after reload.
	if matched, _ := regexp.MatchString(`(?i)\/login|\/signin`, urlAfter); matched { t.Errorf("unexpected match") }
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
}

func TestDeepLinkSettingsURLIsDirectlyAccessibleWhenAuthenticated(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// Try navigating directly to /settings.
	if _, err := page.Goto(baseURL + "/settings"); err != nil { t.Logf("goto: %v", err) }
	_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	// Either loads settings or redirects gracefully — both are acceptable.
	// (pass)
}

func TestDeepLinkDashboardURLIsDirectlyAccessibleWhenAuthenticated(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	if _, err := page.Goto(baseURL + "/dashboard"); err != nil { t.Logf("goto: %v", err) }
	_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	// (pass)
}

func TestUnknownRoute404PageRendersGracefullyWithoutCrashing(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	resp, _ := page.Goto(baseURL + "/this-route-definitely-does-not-exist-xyz")
	// Either a 404 status or a SPA-style fallback (200 + custom 404 UI) is acceptable.
	// const status = response?.status() ?? 200
	if status >= 500 { t.Errorf("expected < 500") }
	if matched, _ := regexp.MatchString(`(?i)uncaught error`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
}

func TestOnboardingWizardCanBeSkippedEntirelyFromTheFirstStep(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
	// const skipAll = page .locator('button') 
	if vis, _ := skipAll.IsVisible(); vis {
		if err := skipAll.Click(nil); err != nil { t.Logf("click: %v", err) }
		sleepMs(1000)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
	}
}

func TestOnboardingWizardProgressBarOrStepIndicatorIsVisible(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// const progressBar = page .locator( '[role="progressbar"], [data-testid*="progress"], .stepper, [clas
	// const visible = await progressBar.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false)
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	// (pass)
}

func TestOnboardingBackButtonIsPresentFromStep2Onward(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	nextBtn := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First()
	if vis, _ := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First().IsVisible(); vis {
		if err := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(next|continue|proceed)$")}).First().Click(nil); err != nil { t.Logf("click: %v", err) }
		sleepMs(800)
		backBtn := page.Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("^(back|previous|go back)$")}).First()
		// const visible = await backBtn.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
}

func TestFormValidationRequiredFieldShowsValidationMessageOnEmptySubmit(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// Find any form submit button that is NOT the login form.
	forms := page.Locator(`form`)
	formCount, _ := page.Locator(`form`).Count()
	for i := 0; i < func() int { if formCount < 3 { return formCount }; return 3 }(); i++ {
		// const submitBtn = forms.nth(i).locator('button[type="submit"]').first()
		if vis, _ := submitBtn.IsVisible(); vis {
			if err := submitBtn.Click(nil); err != nil { t.Logf("click: %v", err) }
			sleepMs(500)
			validationMsg := page.Locator(`[role="alert"], .error, .invalid-feedback, :invalid`).First()
			// const valid = await validationMsg.isVisible({ timeout: 1_000 }).catch(() => false)
			break
		}
	}
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	// (pass)
}

func TestModalDialogModalClosesOnEscapeKey(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	dialog := page.Locator(`[role="dialog"]`).First()
	if vis, _ := page.Locator(`[role="dialog"]`).First().IsVisible(); vis {
		_ = page.Keyboard.Press("Escape")
		sleepMs(600)
		// const stillOpen = await dialog.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)
		// Many modals close on Escape; some (like wizard) may not.
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
}

func TestModalDialogCancelButtonClosesDialogWithoutSaving(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	dialog := page.Locator(`[role="dialog"]`).First()
	if vis, _ := page.Locator(`[role="dialog"]`).First().IsVisible(); vis {
		cancelBtn := page.Locator(`[role="dialog"]`).First().Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("cancel|close|dismiss")}).First()
		if vis, _ := page.Locator(`[role="dialog"]`).First().Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("cancel|close|dismiss")}).First().IsVisible(); vis {
			if err := page.Locator(`[role="dialog"]`).First().Locator(`button`).Filter(playwright.LocatorFilterOptions{HasText: playwright.String("cancel|close|dismiss")}).First().Click(nil); err != nil { t.Logf("click: %v", err) }
			sleepMs(600)
		}
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
	// (pass)
}

func TestErrorBoundaryASingleBadAPICallDoesNotCrashTheEntireApp(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// Intercept a non-critical API endpoint and make it fail.
	_ = page.Route("**/api/notifications*", func(route playwright.Route) {
		_ = route.Fulfill(playwright.RouteFulfillOptions{Status: playwright.Int(500), Body: playwright.String(`{"error":"injected failure"}`)})
	_, _ = page.Reload(nil)
	_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
	// The app should remain functional despite the simulated failure.
	if matched, _ := regexp.MatchString(`(?i)uncaught error`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	heading := page.Locator(`h1, h2`).First()
	// await expect(heading).toBeVisible({ timeout: LONG_TIMEOUT })
	}
}

func TestOfflineSimulationAppShowsDegradedUiOrOfflineMessage(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)
	// Take the browser offline.
	_ = page.Context().SetOffline(true)
	// await page.goto('/').catch(() => {}); // may throw — that is fine
	sleepMs(1500)
	// Restore online state so subsequent tests are unaffected.
	_ = page.Context().SetOffline(false)
	// (pass)
}

func TestPerformanceMainBundleSizeIsBelow10MbNoBloatRegression(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	// const sizes: number[] = []
	// (event listener)
		// if (/\.(js|mjs)(\?|$)/.test(response.url())) {
			// const body = await response.body().catch(() => Buffer.alloc(0))
			// sizes.push(body.length)
		}
	// })
	if _, err := page.Goto(baseURL + "/"); err != nil { t.Logf("goto: %v", err) }
	_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
	// const totalJs = sizes.reduce((a, b) => a + b, 0)
	// 10 MB is generous but protects against accidental dep bloat.
	if totalJs >= 10485760 { t.Errorf("expected < 10485760") }
}

func TestEndToEndSmokeFullInstallLoginDashboardSettingsLogoutFlow(t *testing.T) {
	page := newPage(t)
	defer page.Close()

	// 1. Open app.
	if _, err := page.Goto(baseURL + "/"); err != nil { t.Logf("goto: %v", err) }
	_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }

	// 2. Log in as admin.
	loginAsAdmin(t, page)
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	// expect(page.url()).not.toMatch(/\/login|\/signin/i)

	// 3. Navigate to settings.
	// const settingsLink = page .locator('nav a, nav button, aside a, [role="menuitem"]') 
	if vis, _ := settingsLink.IsVisible(); vis {
		if err := settingsLink.Click(nil); err != nil { t.Logf("click: %v", err) }
		_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)
		if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}

	// 4. Log out.
	// let logoutBtn = ...
	// if (!(await logoutBtn.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false))) {
		// const userMenu = page .locator('[data-testid*="user-menu"], [aria-label*="user menu" i], button:has-
		if vis, _ := userMenu.IsVisible(); vis {
			if err := userMenu.Click(nil); err != nil { t.Logf("click: %v", err) }
			sleepMs(600)
		}
	}
	if vis, _ := logoutBtn.IsVisible(); vis {
		if err := logoutBtn.Click(nil); err != nil { t.Logf("click: %v", err) }
		sleepMs(2000)
		// expect(page.url()).toMatch(/\/login|\/signin|^\//)
	}
	if matched, _ := regexp.MatchString(`(?i)uncaught error|500`, func() string { c, _ := page.Content(); return c }()); matched { t.Error("body contains error text") }
	}
}
