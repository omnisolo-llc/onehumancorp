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
    await expect(page1.getByRole('heading', { name: "Tell us about your business" })).toBeVisible({ timeout: 15000 });

    // 2. Start flow and type business description
    await page1.getByPlaceholder('e.g. I am Maya. I bake vegan cakes in Austin. Prices start at $50.').fill('I am Maya. I run a cross-device bakery.');

    // Wait for the debounce save state to trigger
    await page1.waitForTimeout(3000);

    // Close context 1 to prove we aren't relying on it
    await context1.close();

    // 3. Open a completely new incognito browser session (Device 2)
    const context2 = await browser.newContext();
    const page2 = await context2.newPage();

    // Set the SAME tenant ID, but nothing else (empty state in localstorage)
    await page2.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-cross-device-tenant');
      localStorage.setItem('user_id', 'e2e-cross-device-user');
    });

    await page2.goto('/onboarding');
    await page2.waitForTimeout(1000); // Give the fetch request time to complete and set state
    await expect(page2.getByRole('heading', { name: "Tell us about your business" })).toBeVisible({ timeout: 15000 });

    // The backend should restore the state, filling the input
    await expect(page2.getByPlaceholder('e.g. I am Maya. I bake vegan cakes in Austin. Prices start at $50.')).toHaveValue('I am Maya. I run a cross-device bakery.');

    await context2.close();
  });
});
