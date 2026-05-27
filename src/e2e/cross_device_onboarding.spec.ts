import { test, expect } from './fixtures';

test.describe('Onboarding Wizard - Cross Device Resilience', () => {
  test('persists state correctly across separate sessions', async ({ browser }) => {
    const context1 = await browser.newContext();
    const page1 = await context1.newPage();

    // Set mock credentials to ensure deterministic tenant ID
    await page1.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-cross-device-tenant');
      localStorage.setItem('user_id', 'e2e-cross-device-user');
    });

    // 1. Visit the builder on Device 1
    await page1.goto('/onboarding');
    await expect(page1.locator('#setup-screen')).toBeVisible({ timeout: 15000 });

    // 2. Start flow and type business description
    // Mock the backend state storage since the rust backend might not be available during npx playwright test locally
    let backendState = {};
    await page1.route('**/api/onboarding/state', async (route) => {
        if (route.request().method() === 'POST') {
            const body = route.request().postDataJSON();
            backendState = body;
            await route.fulfill({ status: 200, json: { success: true } });
        } else {
            await route.fulfill({ status: 200, json: backendState });
        }
    });

    await page1.getByPlaceholder('e.g. I bake custom vegan cakes in Portland, OR...').fill('I bake custom vegan cakes');

    // Wait for the debounce saveWizardState to trigger
    await page1.waitForTimeout(1000);

    // Close context 1 to prove we aren't relying on it
    await context1.close();

    // 3. Open a completely new incognito browser session (Device 2)
    const context2 = await browser.newContext();
    const page2 = await context2.newPage();

    // Mock the backend state retrieval
    await page2.route('**/api/onboarding/state', async (route) => {
        if (route.request().method() === 'POST') {
            const body = route.request().postDataJSON();
            backendState = body;
            await route.fulfill({ status: 200, json: { success: true } });
        } else {
            await route.fulfill({ status: 200, json: backendState });
        }
    });

    // Set the SAME tenant ID, but nothing else (empty wizard state in localstorage)
    await page2.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-cross-device-tenant');
      localStorage.setItem('user_id', 'e2e-cross-device-user');
    });

    await page2.goto('/onboarding');
    await expect(page2.locator('#setup-screen')).toBeVisible({ timeout: 15000 });

    // The backend should restore the state and the input should have the value
    // We need to wait for loadState to complete and set the value
    await page2.waitForTimeout(500);
    await expect(page2.getByPlaceholder('e.g. I bake custom vegan cakes in Portland, OR...')).toHaveValue('I bake custom vegan cakes');

    await context2.close();
  });
});
