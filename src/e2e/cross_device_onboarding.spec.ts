import { test, expect } from './fixtures';

test.describe('Onboarding Wizard - Cross Device Resilience', () => {
  test('persists state correctly across separate sessions', async ({ browser }) => {
    const context1 = await browser.newContext();
    const page1 = await context1.newPage();

    // Mock intake and start API calls just in case, though we don't proceed that far
    await page1.route('**/api/onboarding/intake', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ business_name: "Cross Device Biz", business_type: "Bakery", categories: ["food"], initial_products: [{ name: "Cake", price: "20" }] }) }));

    // Set mock credentials to ensure deterministic tenant ID
    await page1.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-cross-device-tenant');
      localStorage.setItem('user_id', 'e2e-cross-device-user');
    });

    // 1. Visit the builder on Device 1
    await page1.goto('/onboarding');
    await expect(page1.getByRole('heading', { name: "Tell us about your business" })).toBeVisible({ timeout: 15000 });

    // 2. Type business description
    const descriptionInput = page1.getByPlaceholder('e.g. I bake custom vegan cakes in Portland, OR...');
    await descriptionInput.fill('I am testing cross device sync in Miami');

    // Trigger blur/change so zustand state updates (it's bound to onChange, so filling is enough)

    // Click Generate to move to Step 2
    await page1.getByRole('button', { name: /Generate My Business/i }).click();

    // Wait for Step 2: Review Details
    await expect(page1.getByRole('heading', { name: "Review Details" })).toBeVisible({ timeout: 15000 });

    // Modify business name to verify sync
    const businessNameInput = page1.locator('input').first(); // The first input on Step 2 is Business Name
    await businessNameInput.fill('Cross Device Mastery Store');

    // Wait for the debounce saveWizardState to trigger (1 second + network time)
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

    // Wait for page to load and fetch state
    // Because we were on Step 2 when we closed Device 1, we should be restored to Step 2!
    await expect(page2.getByRole('heading', { name: "Review Details" })).toBeVisible({ timeout: 15000 });

    // Verify the inputs were restored!
    const restoredBusinessName = page2.locator('input').first();
    await expect(restoredBusinessName).toHaveValue('Cross Device Mastery Store');

    await context2.close();
  });
});
