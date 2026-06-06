# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: lead_gen.spec.ts >> Hyperlocal Lead Generation Agent >> should allow a business owner to start a lead generation campaign and see inbox conversion
- Location: src/e2e/lead_gen.spec.ts:5:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: getByRole('link', { name: /Want more local jobs this week\?/i })
Expected: visible
Timeout: 5000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 5000ms
  - waiting for getByRole('link', { name: /Want more local jobs this week\?/i })

```

```yaml
- text: missing required error components, refreshing...
```

# Test source

```ts
  1  | import { test, expect } from './fixtures';
  2  | import { currentAppSmoke } from './current_app_smoke';
  3  |
  4  | test.describe('Hyperlocal Lead Generation Agent', () => {
  5  |   test('should allow a business owner to start a lead generation campaign and see inbox conversion', async ({ page }) => {
  6  |     // We mock the backend API since we don't have the rust server running in this test environment.
  7  |     // However, the acceptance criteria states "zero mock UI data" and "exercise the real frontend-to-backend-to-database path".
  8  |     // Playwright in this CI setup is hitting Next.js dev server which cannot reach `localhost:8080` (Rust Backend).
  9  |     // The previous instructions specifically say: "E2E tests must exercise the real frontend-to-backend-to-database path."
  10 |     // Given the environment constraints, we use the `fixtures.ts` which mounts the DB logic locally in some tests, or we have to rely on `test_backend` to be up.
  11 |
  12 |     // In order to pass E2E tests when the real backend isn't up in this sandbox,
  13 |     // we must mock the response just for the test if it's acceptable, OR we just trust the test.
  14 |     // The instructions say NO MOCK NETWORK REQUESTS IN E2E TESTS.
  15 |     // This means the Rust backend *must* be running.
  16 |
  17 |     // 1. Owner opens the mobile app and navigates to the Marketing/Dashboard
  18 |     await page.goto('/dashboard');
  19 |
  20 |     // Look for the new Lead Gen card
  21 |     const leadGenCard = page.getByRole('link', { name: /Want more local jobs this week\?/i });
> 22 |     await expect(leadGenCard).toBeVisible();
     |                               ^ Error: expect(locator).toBeVisible() failed
  23 |     await leadGenCard.click();
  24 |
  25 |     // 2. Owner inputs a weekly budget and service radius
  26 |     await expect(page).toHaveURL(/.*lead-gen/);
  27 |     await expect(page.getByRole('heading', { name: 'Start Finding Jobs' })).toBeVisible();
  28 |
  29 |     const budgetInput = page.getByLabel('Weekly Budget ($)');
  30 |     await budgetInput.fill('50');
  31 |
  32 |     const zipCodeInput = page.getByLabel('Target Zip Code / Radius');
  33 |     await zipCodeInput.fill('90210');
  34 |
  35 |     // 3. The platform initiates the LeadGenCampaign via the backend AI job queue
  36 |     const startButton = page.getByRole('button', { name: 'Start Finding Jobs' });
  37 |
  38 |     // Wait for navigation
  39 |     await Promise.all([
  40 |       page.waitForURL(/.*dashboard\?lead_gen_started=1/),
  41 |       startButton.click(),
  42 |     ]);
  43 |
  44 |     // 4. Navigate to inbox to verify
  45 |     await page.goto('/inbox');
  46 |
  47 |     // Check for the "New booking received from local lead generation campaign!" message
  48 |     const message = page.getByText('New booking received from local lead generation campaign!');
  49 |     // In our test environment without a full backend, this might fail, but we'll try.
  50 |     await expect(message).toBeVisible({ timeout: 10000 });
  51 |   });
  52 | });
  53 |
```