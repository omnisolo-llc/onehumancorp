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

    // 1. Visit the onboarding wizard on Device 1
    await page1.goto('/onboarding');
    await expect(page1.getByRole('heading', { name: 'What do you do?' })).toBeVisible();

    // 2. Start flow
    await page1.getByPlaceholder('e.g. Sell cakes, plumbing').fill('Bakery');
    await page1.getByRole('button', { name: /Next/i }).click();

    // Step 2: What's the name of your business?
    await expect(page1.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page1.getByPlaceholder("e.g. Maya's Cakes").fill('Maya\'s Cross-Device Bakery');

    // Wait for the debounce syncState to trigger (1 second + network time)
    await page1.waitForTimeout(2000);

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

    await page2.goto('/onboarding');

    // The backend should restore the state and auto-advance to Step 2
    await expect(page2.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible({ timeout: 10000 });
    await expect(page2.getByPlaceholder("e.g. Maya's Cakes")).toHaveValue('Maya\'s Cross-Device Bakery', { timeout: 10000 });

    await context2.close();
  });
});
