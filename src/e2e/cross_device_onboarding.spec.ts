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

    // 1. Visit the new onboarding UI on Device 1
    await page1.goto('/onboarding');
    await expect(page1.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    // 2. Type business name and wait for debounce save
    await page1.getByPlaceholder("e.g. Maya's Cakes").fill("Maya's Cross-Device Bakery");

    // Click Next to advance step and force a store state save (since we save on Next click in the updated store actions)
    await page1.getByRole('button', { name: /Next/i }).click();

    // Wait for the next step heading
    await expect(page1.getByRole('heading', { name: "What's your niche?" })).toBeVisible();

    // Wait for the fetch to resolve
    await page1.waitForTimeout(1000);

    // Close context 1 to prove we aren't relying on it locally
    await context1.close();

    // 3. Open a completely new incognito browser session (Device 2)
    const context2 = await browser.newContext();
    const page2 = await context2.newPage();

    // Set the SAME tenant ID, but nothing else (empty wizard state in localstorage)
    await page2.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-cross-device-tenant');
      localStorage.setItem('user_id', 'e2e-cross-device-user');
    });

    await page2.goto('/onboarding');

    // The backend should restore the state and auto-advance to step 2 since we completed step 1
    await expect(page2.getByRole('heading', { name: "What's your niche?" })).toBeVisible({ timeout: 10000 });

    // Check that we can go back and see the input
    await page2.getByRole('button', { name: /Back/i }).click();

    await expect(page2.getByPlaceholder("e.g. Maya's Cakes")).toHaveValue("Maya's Cross-Device Bakery", { timeout: 10000 });

    await context2.close();
  });
});
