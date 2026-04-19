package e2e

import (
	"os"
	"strings"
	"testing"
	"time"

	playwright "github.com/playwright-community/playwright-go"
)

var (
	adminUser = getEnvOr("OHC_E2E_ADMIN_USER", "admin")
	adminPass = getEnvOr("OHC_E2E_ADMIN_PASS", "admin")
)

const (
	shortTimeout  = float64(5_000)
	mediumTimeout = float64(10_000)
	longTimeout   = float64(30_000)
)

func getEnvOr(key, def string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return def
}

func newPage(t *testing.T) playwright.Page {
	t.Helper()
	page, err := bCtx.NewPage()
	if err != nil {
		t.Fatalf("newPage: %v", err)
	}
	return page
}

func openApp(t *testing.T, page playwright.Page) {
	t.Helper()
	if _, err := page.Goto(baseURL + "/"); err != nil {
		t.Fatalf("openApp goto: %v", err)
	}
	if err := page.WaitForLoadState(playwright.PageWaitForLoadStateOptions{State: playwright.LoadStateNetworkidle}); err != nil {
		t.Logf("openApp networkidle: %v", err)
	}
}

func loginAsAdmin(t *testing.T, page playwright.Page) {
	t.Helper()
	openApp(t, page)

	loginForm := page.Locator(`form, [data-testid="login-form"], [aria-label*="login" i], [aria-label*="sign in" i]`)
	url := page.URL()
	formCount, _ := loginForm.Count()
	isLoginPage := strings.Contains(url, "/login") || strings.Contains(url, "/signin") || formCount > 0

	if isLoginPage {
		emailInput := page.Locator(`input[type="email"], input[name="email"], input[placeholder*="email" i], input[placeholder*="username" i]`).First()
		passwordInput := page.Locator(`input[type="password"], input[name="password"], input[placeholder*="password" i]`).First()

		if err := emailInput.Fill(adminUser); err != nil {
			t.Logf("loginAsAdmin fill email: %v", err)
		}
		if err := passwordInput.Fill(adminPass); err != nil {
			t.Logf("loginAsAdmin fill password: %v", err)
		}

		submitBtn := page.Locator(`button[type="submit"], button:has-text("Login"), button:has-text("Sign In"), button:has-text("Log In")`).First()
		if err := submitBtn.Click(); err != nil {
			t.Logf("loginAsAdmin click submit: %v", err)
		}

		_ = page.WaitForURL("**", playwright.PageWaitForURLOptions{Timeout: playwright.Float(15000)})
		_ = page.WaitForLoadState(playwright.PageWaitForLoadStateOptions{State: playwright.LoadStateNetworkidle})
	}
}

func clickNext(t *testing.T, page playwright.Page) {
	t.Helper()
	nextBtn := page.Locator(`button:has-text("Next"), button:has-text("Continue"), button:has-text("Proceed")`).First()
	if err := nextBtn.Click(); err != nil {
		t.Logf("clickNext: %v", err)
	}
}

func navigateTo(t *testing.T, page playwright.Page, label string) {
	t.Helper()
	navLink := page.Locator(`nav a, nav button, [role="navigation"] a, [role="menuitem"], aside a`).
		Filter(playwright.LocatorFilterOptions{HasText: playwright.String(label)}).First()
	if err := navLink.Click(); err != nil {
		t.Fatalf("navigateTo %q: %v", label, err)
	}
	_ = page.WaitForLoadState(playwright.PageWaitForLoadStateOptions{State: playwright.LoadStateNetworkidle})
}

func sleepMs(ms int) {
	time.Sleep(time.Duration(ms) * time.Millisecond)
}
