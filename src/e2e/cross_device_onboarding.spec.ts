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
    await page1.getByPlaceholder('e.g. I bake custom vegan cakes in Portland, OR...').fill('Sell custom cakes');

    // Intercept intake request to return a successful mock since this is E2E without real AI backend keys
    await page1.route('**/api/onboarding/intake', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_name: "Maya's Cross-Device Bakery",
          business_type: "Bakery",
          categories: ["food", "physical"],
          initial_products: [{ name: "Custom Vegan Cake", price: "45.00" }]
        })
      });
    });

    await page1.getByRole('button', { name: /Generate My Business/i }).click();

    // Wait for step 2 (review details)
    await expect(page1.getByText("Maya's Cross-Device Bakery")).toBeVisible({ timeout: 15000 });

    // Change a field to test persistence
    await page1.locator('input').filter({ hasText: "Maya's Cross-Device Bakery" }).fill("Maya's Persistent Bakery");

    // Setup response listener before action
    const saveResponsePromise = page1.waitForResponse(response => response.url().includes('/api/onboarding/state') && response.status() === 200);
    // Click continue to step 3
    await page1.getByRole('button', { name: /Continue/i }).click();

    // Wait for the debounce saveWizardState to trigger via transition
    await expect(page1.getByRole('heading', { name: 'Style & Team' })).toBeVisible({ timeout: 15000 });
    await saveResponsePromise; // wait for state sync

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

    // The backend should restore the state and auto-advance to step 3 where we left off
    // However, since it's testing cross-device, wait for the Style & Team screen
    await expect(page2.getByRole('heading', { name: 'Style & Team' })).toBeVisible({ timeout: 15000 });

    await context2.close();
  });
});
