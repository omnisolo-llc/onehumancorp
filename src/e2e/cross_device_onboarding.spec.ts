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
    await page1.goto('/website-builder');
    await expect(page1.locator('#setup-screen')).toBeVisible();

    // 2. Start flow and type business name
    await page1.getByRole('button', { name: /Start My Business Next/ }).click();
    await page1.getByRole('button', { name: /Online Store/ }).click();
    await page1.getByPlaceholder('What is your business called?').fill('Maya\'s Cross-Device Bakery');

    // Wait for the debounce saveWizardState to trigger
    await page1.waitForTimeout(1000);

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

    await page2.goto('/website-builder');
    await expect(page2.locator('#setup-screen')).toBeVisible();

    // The backend should restore the state and auto-advance, or at least fill the inputs
    await expect(page2.getByPlaceholder('What is your business called?')).toHaveValue('Maya\'s Cross-Device Bakery', { timeout: 10000 });

    await context2.close();
  });
  test('persists state correctly across separate sessions for new onboarding flow', async ({ browser }) => {
    const context1 = await browser.newContext();
    const page1 = await context1.newPage();

    // Set mock credentials to ensure deterministic tenant ID
    await page1.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-cross-device-tenant-new');
      localStorage.setItem('user_id', 'e2e-cross-device-user-new');
    });

    // 1. Visit the onboarding on Device 1
    await page1.goto('/onboarding');
    await expect(page1.getByRole('heading', { name: "What do you do?" })).toBeVisible();

    // 2. Start flow and type business name
    await page1.getByRole('button', { name: 'Food & Beverage' }).click();
    await page1.getByRole('button', { name: /Next/i }).click();
    await expect(page1.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page1.getByPlaceholder("e.g. Maya's Cakes").fill("Test Restaurant Cross Device");

    // Wait for the debounce syncStateToBackend to trigger
    await page1.waitForTimeout(1000);

    // Close context 1 to prove we aren't relying on it
    await context1.close();

    // 3. Open a completely new incognito browser session (Device 2)
    const context2 = await browser.newContext();
    const page2 = await context2.newPage();

    // Set the SAME tenant ID, but nothing else (empty wizard state in localstorage)
    await page2.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-cross-device-tenant-new');
      localStorage.setItem('user_id', 'e2e-cross-device-user-new');
    });

    await page2.goto('/onboarding');

    // The backend should restore the state and show step 2 with the name filled
    await expect(page2.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible({ timeout: 10000 });
    await expect(page2.getByPlaceholder("e.g. Maya's Cakes")).toHaveValue('Test Restaurant Cross Device', { timeout: 10000 });

    await context2.close();
  });
});
