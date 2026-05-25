import { test, expect } from './fixtures';

test.describe('Onboarding Wizard - Cross Device Resilience', () => {
  test('persists state correctly across separate sessions', async ({ browser }) => {
    const context1 = await browser.newContext();
    const page1 = await context1.newPage();

    // Set mock credentials to ensure deterministic tenant ID
    await page1.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-cross-device-tenant-v2');
      localStorage.setItem('user_id', 'e2e-cross-device-user-v2');
    });

    // 1. Visit the builder on Device 1
    await page1.goto('/onboarding');
    await expect(page1.getByRole('heading', { name: "What do you do?" })).toBeVisible();

    // 2. Start flow and type business type
    await page1.getByPlaceholder('e.g. Sell cakes, plumbing').fill('Sell custom cakes');

    // Wait for the debounce syncState to trigger (it has a 1000ms delay in page.tsx)
    await page1.waitForTimeout(2000);

    // Close context 1 to prove we aren't relying on it
    await context1.close();

    // 3. Open a completely new incognito browser session (Device 2)
    const context2 = await browser.newContext();
    const page2 = await context2.newPage();

    // Set the SAME tenant ID, but nothing else (empty wizard state in localstorage)
    await page2.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-cross-device-tenant-v2');
      localStorage.setItem('user_id', 'e2e-cross-device-user-v2');
    });

    await page2.goto('/onboarding');
    await expect(page2.getByRole('heading', { name: "What do you do?" })).toBeVisible();

    // The backend should restore the state and fill the input
    await expect(page2.getByPlaceholder('e.g. Sell cakes, plumbing')).toHaveValue('Sell custom cakes', { timeout: 10000 });

    await context2.close();
  });
});
