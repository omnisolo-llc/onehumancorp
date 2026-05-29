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

    // 2. Start flow and type business name
    await expect(page1.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page1.getByPlaceholder("e.g. Maya's Custom Cakes").fill("Maya's Cross-Device Bakery");
    await page1.getByRole('button', { name: 'Next', exact: true }).click();

    // Wait for step 1 chat 2
    await expect(page1.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
    await page1.getByPlaceholder('e.g. I bake custom vegan cakes for weddings and parties...').fill('Custom vegan cakes');
    await page1.getByRole('button', { name: 'Next', exact: true }).click();

    // Wait for step 1 chat 3
    await expect(page1.getByRole('heading', { name: 'Where are you located?' })).toBeVisible();
    await page1.getByPlaceholder('e.g. Portland, OR').fill('Portland, OR');

    // Click Generate to go to step 2 (Review Details)
    await page1.getByRole('button', { name: /Generate My Business/i }).click();

    // Wait for step 2 (Review Details)
    await expect(page1.getByRole('heading', { name: 'Review Details' })).toBeVisible({ timeout: 15000 });

    // Wait for the debounce saveWizardState to trigger
    await page1.waitForTimeout(3000);

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
    await expect(page2.locator('#setup-screen')).toBeVisible({ timeout: 15000 });

    // The backend should restore the state and auto-advance to step 2
    await expect(page2.getByRole('heading', { name: 'Review Details' })).toBeVisible({ timeout: 15000 });
    // And input should be populated with values returned from intake endpoint / restored from state
    // But mostly we check we are on Step 2.
    await expect(page2.getByRole('button', { name: 'Continue' })).toBeVisible();

    await context2.close();
  });
});
