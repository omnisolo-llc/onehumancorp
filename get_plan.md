Wait, if it renders, maybe `page.Locator("text=Share my business")` is failing because in Flutter CanvasKit, text is not accessible like that?
Let's see if other tests use text locators.
`src/tests/e2e/cuj_extended_test.go` doesn't.
What about `src/tests/e2e/helpers_test.go`?
```go
func loginAsAdmin(t *testing.T, page playwright.Page) {
	...
	submitBtn := page.Locator(`button[type="submit"], button:has-text("Login"), button:has-text("Sign In"), button:has-text("Log In")`).First()
```
It uses `button:has-text("Login")` which works?! Wait, we established earlier that the tests pass because they don't actually test anything.
What about the `TestGrowthShareWidgetRendersOnDashboard` timeout? It timed out at `WaitForURL`!
```
	// Wait for dashboard to load
	err := page.WaitForURL("**/dashboard")
	assert.NoError(t, err) // THIS LINE TIMED OUT
```
Wait! If `page.WaitForURL("**/dashboard")` timed out, it means the URL never reached `**/dashboard`!
WHY?
Because `loginAsAdmin` uses `page.WaitForURL("**", ...)` which matches `/login`!
So it clicks submit, waits for `**` (any URL, which is currently `/login`), and then returns!
Then `TestGrowthShareWidgetRendersOnDashboard` waits for `**/dashboard`, but the app is stuck on `/login` because `emailInput.Fill(...)` failed silently!

Let's fix `loginAsAdmin` in `src/tests/e2e/helpers_test.go` so it actually logs in properly, or bypasses the UI?
Memory says: "To reliably bypass UI interactions like login, inject application state early via page.addInitScript() (e.g., window.localStorage.setItem('flutter.auth_token', '...')) and use page.route to mock required backend API responses so the app does not hang or redirect before the Flutter engine fully hydrates."
Wait, if I bypass login using `page.addInitScript()`, I should do that in `openApp` or `loginAsAdmin`!
Let's check `loginAsAdmin` again.
