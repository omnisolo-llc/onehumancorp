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

    // Mock the state endpoints to actually store and return state in memory
    let persistedState: any = {};

    await page1.route('**/api/onboarding/state', async (route, request) => {
      if (request.method() === 'POST') {
        const data = JSON.parse(request.postData() || '{}');
        persistedState = { ...persistedState, ...data };
        await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({}) });
      } else if (request.method() === 'GET') {
        await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify(persistedState) });
      } else {
        await route.continue();
      }
    });

    // 1. Visit the builder on Device 1
    await page1.goto('/onboarding');
    await expect(page1.locator('#setup-screen')).toBeVisible({ timeout: 15000 });
    await expect(page1.getByRole('heading', { name: "Tell us about your business" })).toBeVisible();

    // 2. Start flow and type business description
    await page1.getByPlaceholder('e.g. I bake custom vegan cakes in Portland, OR...').fill('Cross device testing business');

    // Wait for the debounce save state to trigger
    await page1.waitForTimeout(1500);

    // Verify it was saved by checking our mock memory
    expect(persistedState.businessDescription).toBe('Cross device testing business');

    // Close context 1 to prove we aren't relying on it
    await context1.close();

    // 3. Open a completely new incognito browser session (Device 2)
    const context2 = await browser.newContext();
    const page2 = await context2.newPage();

    // Set the SAME tenant ID, but nothing else (empty wizard state in localstorage)
    await page2.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-cross-device-tenant');
      localStorage.setItem('user_id', 'e2e-cross-device-user');
    });

    // Mock state route in context 2 using the state persisted from context 1
    await page2.route('**/api/onboarding/state', async (route, request) => {
      if (request.method() === 'POST') {
        const data = JSON.parse(request.postData() || '{}');
        persistedState = { ...persistedState, ...data };
        await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({}) });
      } else if (request.method() === 'GET') {
        await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify(persistedState) });
      } else {
        await route.continue();
      }
    });

    await page2.goto('/onboarding');
    await expect(page2.locator('#setup-screen')).toBeVisible({ timeout: 15000 });

    // The frontend should restore the state from backend and fill the input
    await expect(page2.getByPlaceholder('e.g. I bake custom vegan cakes in Portland, OR...')).toHaveValue('Cross device testing business');

    await context2.close();
  });
});
