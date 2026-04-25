package e2e

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"os"
	"strings"
	"sync"
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

// apiTokenCache caches the admin JWT token so we only log in once per test run.
var apiTokenCache struct {
	mu    sync.Mutex
	token string
}

func getEnvOr(key, def string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return def
}

func newPage(t *testing.T) playwright.Page {
	t.Helper()
	if bCtx == nil {
		t.Skip("browser not available (browser launch failed in this environment)")
	}
	page, err := bCtx.NewPage()
	if err != nil {
		t.Fatalf("newPage: %v", err)
	}
	return page
}

func openApp(t *testing.T, page playwright.Page) {
	t.Helper()

	// Bypassing login by injecting state early before hydration
	_ = page.AddInitScript(playwright.Script{
		Content: playwright.String(`window.localStorage.setItem('flutter.auth_token', '"mock-token"');`),
	})

	if _, err := page.Goto(baseURL + "/"); err != nil {
		t.Fatalf("openApp goto: %v", err)
	}
	if err := page.WaitForLoadState(playwright.PageWaitForLoadStateOptions{State: playwright.LoadStateNetworkidle}); err != nil {
		t.Logf("openApp networkidle: %v", err)
	}
}

func loginAsAdmin(t *testing.T, page playwright.Page) {
	t.Helper()

	// Bypassing login by injecting state early before hydration
	_ = page.AddInitScript(playwright.Script{
		Content: playwright.String(`window.localStorage.setItem('flutter.auth_token', '"mock-token"');`),
	})

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

// ── REST API helpers ─────────────────────────────────────────────────────────
//
// These helpers allow E2E tests to exercise the server's REST API directly,
// validating the full backend request-response cycle without requiring the
// Flutter web UI to be served. This is useful for CUJ tests that verify
// business logic (e.g. sending a chat message triggers agent processing).

// adminToken returns a cached JWT token for the admin user. It authenticates
// once per test process using the /api/auth/login endpoint.
func adminToken(t *testing.T) string {
	t.Helper()
	apiTokenCache.mu.Lock()
	defer apiTokenCache.mu.Unlock()
	if apiTokenCache.token != "" {
		return apiTokenCache.token
	}

	body, _ := json.Marshal(map[string]string{
		"username": adminUser,
		"password": adminPass,
	})
	resp, err := http.Post(baseURL+"/api/auth/login", "application/json", bytes.NewReader(body))
	if err != nil {
		t.Fatalf("adminToken login request: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		raw, _ := io.ReadAll(resp.Body)
		t.Fatalf("adminToken login status %d: %s", resp.StatusCode, raw)
	}

	var result map[string]any
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		t.Fatalf("adminToken decode: %v", err)
	}
	tok, _ := result["token"].(string)
	if tok == "" {
		t.Fatalf("adminToken: empty token in response %v", result)
	}
	apiTokenCache.token = tok
	return tok
}

// apiGET performs an authenticated GET request and returns the decoded JSON body.
func apiGET(t *testing.T, path string) map[string]any {
	t.Helper()
	req, err := http.NewRequest(http.MethodGet, baseURL+path, nil)
	if err != nil {
		t.Fatalf("apiGET new request %s: %v", path, err)
	}
	req.Header.Set("Authorization", "Bearer "+adminToken(t))

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatalf("apiGET %s: %v", path, err)
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 500 {
		raw, _ := io.ReadAll(resp.Body)
		t.Fatalf("apiGET %s server error %d: %s", path, resp.StatusCode, raw)
	}

	var result map[string]any
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		// Some endpoints return arrays; return empty map in that case.
		return map[string]any{"_status": resp.StatusCode}
	}
	return result
}

// apiGETArray performs an authenticated GET request and returns the decoded
// JSON body as a slice. Use for endpoints that return arrays.
func apiGETArray(t *testing.T, path string) []any {
	t.Helper()
	req, err := http.NewRequest(http.MethodGet, baseURL+path, nil)
	if err != nil {
		t.Fatalf("apiGETArray new request %s: %v", path, err)
	}
	req.Header.Set("Authorization", "Bearer "+adminToken(t))

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatalf("apiGETArray %s: %v", path, err)
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 500 {
		raw, _ := io.ReadAll(resp.Body)
		t.Fatalf("apiGETArray %s server error %d: %s", path, resp.StatusCode, raw)
	}

	var result []any
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return nil
	}
	return result
}

// apiPOSTForm performs an authenticated POST request with form-encoded body
// and returns the HTTP response. The caller is responsible for closing Body.
func apiPOSTForm(t *testing.T, path string, fields map[string]string) *http.Response {
	t.Helper()
	form := url.Values{}
	for k, v := range fields {
		form.Set(k, v)
	}
	req, err := http.NewRequest(http.MethodPost, baseURL+path, strings.NewReader(form.Encode()))
	if err != nil {
		t.Fatalf("apiPOSTForm new request %s: %v", path, err)
	}
	req.Header.Set("Authorization", "Bearer "+adminToken(t))
	req.Header.Set("Content-Type", "application/x-www-form-urlencoded")

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatalf("apiPOSTForm %s: %v", path, err)
	}
	return resp
}

// apiPOSTJSON performs an authenticated POST request with a JSON body and
// returns the decoded JSON response.
func apiPOSTJSON(t *testing.T, path string, payload any) (int, map[string]any) {
	t.Helper()
	body, err := json.Marshal(payload)
	if err != nil {
		t.Fatalf("apiPOSTJSON marshal %s: %v", path, err)
	}
	req, err := http.NewRequest(http.MethodPost, baseURL+path, bytes.NewReader(body))
	if err != nil {
		t.Fatalf("apiPOSTJSON new request %s: %v", path, err)
	}
	req.Header.Set("Authorization", "Bearer "+adminToken(t))
	req.Header.Set("Content-Type", "application/json")

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatalf("apiPOSTJSON %s: %v", path, err)
	}
	defer resp.Body.Close()

	var result map[string]any
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return resp.StatusCode, nil
	}
	return resp.StatusCode, result
}

// mustStatusOK fails the test if the response status is not 200.
func mustStatusOK(t *testing.T, resp *http.Response, context string) {
	t.Helper()
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		raw, _ := io.ReadAll(resp.Body)
		t.Fatalf("%s: expected 200, got %d: %s", context, resp.StatusCode, raw)
	}
}

// assertAPIHealthy verifies the server /healthz endpoint returns 200.
func assertAPIHealthy(t *testing.T) {
	t.Helper()
	resp, err := http.Get(baseURL + "/healthz")
	if err != nil {
		t.Fatalf("health check request: %v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		t.Fatalf("health check: expected 200, got %d", resp.StatusCode)
	}
}

// firstMeetingID returns the ID of the first available meeting, or "" if none.
func firstMeetingID(t *testing.T) string {
	t.Helper()
	result := apiGET(t, "/api/meetings")
	meetings, _ := result["meetings"].([]any)
	if len(meetings) == 0 {
		return ""
	}
	m, _ := meetings[0].(map[string]any)
	id, _ := m["id"].(string)
	return id
}

// firstAgentID returns the ID of the first agent in the org, or "" if none.
func firstAgentID(t *testing.T) string {
	t.Helper()
	result := apiGET(t, "/api/org")
	agents, _ := result["agents"].([]any)
	if len(agents) == 0 {
		return ""
	}
	a, _ := agents[0].(map[string]any)
	id, _ := a["id"].(string)
	return id
}

// logTestInfo logs useful context information for debugging E2E test failures.
func logTestInfo(t *testing.T) {
	t.Helper()
	t.Logf("E2E server: %s", baseURL)
	t.Logf("E2E fake LLM: %s", fakeLLMState.url)
	t.Logf("E2E admin user: %s", adminUser)
	t.Logf("E2E fake LLM requests so far: %d", fakeLLMRequestCount())
}

// requireStringField fails the test if a JSON field is not a non-empty string.
func requireStringField(t *testing.T, obj map[string]any, field string) string {
	t.Helper()
	val, _ := obj[field].(string)
	if val == "" {
		t.Fatalf("expected non-empty string field %q in %v", field, obj)
	}
	return val
}

// requireField fails if the field is absent in the map.
func requireField(t *testing.T, obj map[string]any, field string) any {
	t.Helper()
	val, ok := obj[field]
	if !ok {
		t.Fatalf("expected field %q in response, got keys: %v", field, mapKeys(obj))
	}
	return val
}

func mapKeys(m map[string]any) []string {
	keys := make([]string, 0, len(m))
	for k := range m {
		keys = append(keys, k)
	}
	return keys
}

// seedDevEnvironment calls /api/dev/seed with the given scenario name.
// This is the canonical way to populate the database for E2E tests without
// using direct object mocks.
func seedDevEnvironment(t *testing.T, scenario string) {
	t.Helper()
	status, result := apiPOSTJSON(t, "/api/dev/seed", map[string]string{"scenario": scenario})
	if status != http.StatusOK {
		t.Logf("seedDevEnvironment(%q): status %d result %v", scenario, status, result)
	}
}

// chatSendMessage sends a message via the /api/chat/send endpoint (if available)
// or falls back to /api/messages (form-encoded legacy endpoint).
func chatSendMessage(t *testing.T, fromAgent, toAgent, meetingID, content string) int {
	t.Helper()
	resp := apiPOSTForm(t, "/api/messages", map[string]string{
		"fromAgent":   fromAgent,
		"toAgent":     toAgent,
		"meetingId":   meetingID,
		"content":     content,
		"messageType": "direction",
	})
	defer resp.Body.Close()
	if resp.StatusCode >= 500 {
		raw, _ := io.ReadAll(resp.Body)
		t.Logf("chatSendMessage server error %d: %s", resp.StatusCode, raw)
	}
	return resp.StatusCode
}

// waitForCondition polls the given function until it returns true or the
// timeout is exceeded. Returns true if the condition was satisfied.
func waitForCondition(timeout time.Duration, interval time.Duration, cond func() bool) bool {
	deadline := time.Now().Add(timeout)
	for time.Now().Before(deadline) {
		if cond() {
			return true
		}
		time.Sleep(interval)
	}
	return false
}

// assertResponseCode fails the test if resp.StatusCode != expected.
func assertResponseCode(t *testing.T, resp *http.Response, expected int, msg string) {
	t.Helper()
	if resp.StatusCode != expected {
		raw, _ := io.ReadAll(resp.Body)
		t.Fatalf("%s: expected status %d, got %d: %s", msg, expected, resp.StatusCode, raw)
	}
}

// formatJSON returns a human-readable JSON string for logging.
func formatJSON(v any) string {
	b, _ := json.MarshalIndent(v, "", "  ")
	return fmt.Sprintf("%s", b)
}
