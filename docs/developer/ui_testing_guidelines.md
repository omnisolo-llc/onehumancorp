<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# OHC Playwright UI Testing Guidelines

Welcome to the **UI Testing Guidelines**. In the One Human Corp environment, our mission demands absolute perfection across all frontend applications. The Teammate Mesh and Swarm UI components are highly sophisticated, necessitating rigorous automated testing protocols.

## 1. Core Principles of UI Testing

Our automated tests serve as the vanguard against UI instability. All developers and Implementer Agents must adhere to the following principles:

1. **Deterministic Execution:** Tests must run consistently across Cloud and Standalone mode environments without flaky behaviors.
2. **Resilience to Change:** Selectors should be robust. Prefer `data-testid` attributes over brittle DOM structure or CSS class selectors.
3. **Comprehensive Coverage:** Ensure all user flows—from initial login via SPIFFE identity to Agent memory retrieval—are fully verifiable.

## 2. Setting Up Playwright

We use [Playwright](https://playwright.dev/) as our primary UI testing framework due to its strong cross-browser support and native auto-waiting mechanisms.

### Installation & Configuration

Ensure Playwright is installed via our workspace package manager:

```bash
pnpm install -D @playwright/test
```

Reference the global `playwright.config.ts` located in the root. Ensure your tests adhere to the configured timeouts and retry policies to maintain CI/CD stability.

## 3. Strict Playwright Requirements

To ensure UI stability and vulnerability verification, tests must comply with the following strict requirements:

### 3.1. Robust Locators

Never rely on unstable selectors.

**Do This:**
```typescript
// Use data-testid for resilient locators
const loginButton = page.getByTestId('login-submit-button');
await loginButton.click();
```

**Do Not Do This:**
```typescript
// Avoid brittle XPath or complex CSS chains
const loginButton = page.locator('div.container > form > button:nth-child(3)');
await loginButton.click();
```

### 3.2. Auto-Waiting & Assertions

Leverage Playwright's native auto-waiting capabilities instead of arbitrary timeouts.

**Do This:**
```typescript
// Native web-first assertions automatically wait for conditions
await expect(page.getByTestId('dashboard-header')).toBeVisible();
```

**Do Not Do This:**
```typescript
// Avoid hardcoded sleeps which lead to flaky tests
await page.waitForTimeout(3000);
const isVisible = await page.locator('.header').isVisible();
expect(isVisible).toBe(true);
```

### 3.3. Vulnerability Verification (XSS/CSRF Testing)

Playwright tests must ensure input forms are not susceptible to basic XSS or CSRF vectors. Add specific test cases to inject scripts into inputs and verify they are safely encoded or rejected.

```typescript
test('Sanitization Check: Prevents basic XSS injection in memory search', async ({ page }) => {
  const xssPayload = "<script>alert('XSS')</script>";
  await page.getByTestId('memory-search-input').fill(xssPayload);
  await page.getByTestId('memory-search-submit').click();

  // Verify payload is rendered as text and script is not executed
  const resultText = await page.getByTestId('search-results').textContent();
  expect(resultText).toContain(xssPayload);

  // Verify no alert dialog was triggered
  page.on('dialog', dialog => {
    throw new Error('XSS Alert triggered!');
  });
});
```

### 3.4. Handling Hybrid Mode State

Tests must explicitly define the state they are validating. If testing Standalone mode features, mock the offline state appropriately using Playwright's network interception to block requests to the Orchestration Hub.

```typescript
test.use({ offline: true });

test('Standalone Mode: Shows local SQLite fallback indicator', async ({ page }) => {
  // Intercept requests to Hub API and simulate offline
  await page.route('**/api/hub/**', route => route.abort('internetdisconnected'));

  await page.goto('/dashboard');
  await expect(page.getByTestId('connection-status-standalone')).toBeVisible();
});
```

## 4. Visual Regression Testing

For critical components like the KAIROS Orchestration graphs or the AutoDream memory visualizers, incorporate Visual Regression Testing to catch unintended visual changes.

```typescript
test('Teammate Mesh dashboard visual comparison', async ({ page }) => {
  await page.goto('/mesh-dashboard');
  // Wait for dynamic elements to stabilize
  await expect(page.getByTestId('agent-node-active')).toBeVisible();

  // Compare screenshot against baseline
  await expect(page).toHaveScreenshot('mesh-dashboard-baseline.png', {
    maxDiffPixelRatio: 0.05
  });
});
```

## 5. Review & CI Execution

All Playwright tests are executed on our Bazel-driven CI pipeline. A PR will not be merged unless:
- `bazelisk test //...` passes, which includes the Playwright test suites.
- Coverage remains at 100% for modified interoperability components.

Maintain these standards strictly. The Swarm relies on perfect communication, which starts with perfect UI stability.

</div>
