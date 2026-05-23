import { test, expect } from './fixtures';

test.describe('Next.js Onboarding Wizard - Cross Device Resilience', () => {
  test('persists state correctly across separate sessions', async ({ browser }) => {
    const context1 = await browser.newContext();
    const page1 = await context1.newPage();

    // Set mock credentials to ensure deterministic tenant ID
    await page1.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-cross-device-nextjs-tenant');
      localStorage.setItem('user_id', 'e2e-cross-device-nextjs-user');
    });

    // 1. Visit the builder on Device 1
    await page1.goto('/onboarding');
    await expect(page1.getByRole('heading', { name: "What do you do?" })).toBeVisible();

    // 2. Start flow and type business type
    await page1.getByPlaceholder('e.g. Sell cakes, plumbing').fill('Next.js Cross-Device Setup');
    await page1.getByRole('button', { name: /Next/ }).click();

    // Wait for the debounce saveWizardState to trigger
    await page1.waitForTimeout(1000);

    // Close context 1 to prove we aren't relying on it
    await context1.close();

    // 3. Open a completely new incognito browser session (Device 2)
    const context2 = await browser.newContext();
    const page2 = await context2.newPage();

    // Set the SAME tenant ID, but nothing else (empty wizard state in localstorage)
    await page2.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-cross-device-nextjs-tenant');
      localStorage.setItem('user_id', 'e2e-cross-device-nextjs-user');
    });

    await page2.goto('/onboarding');

    // Check if it navigates back to the correct step based on restored state or at least fills the inputs
    // It should load the businessType. Note that in NextJS version it goes to step 2 after clicking Next
    // In our simplified store, it loads data. But since step is also synced, it should resume at step 2.
    await expect(page2.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible({ timeout: 10000 });

    // We can also click back and see if businessType is filled
    await page2.getByRole('button', { name: 'Back' }).click();
    await expect(page2.getByPlaceholder('e.g. Sell cakes, plumbing')).toHaveValue('Next.js Cross-Device Setup', { timeout: 10000 });

    await context2.close();
  });
});
