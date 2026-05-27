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

    // 2. Start flow and type business description
    await page1.getByPlaceholder('e.g. I bake custom vegan cakes in Portland, OR...').fill('I bake custom vegan cakes');

    // Wait for the debounce save state to trigger
    await page1.waitForTimeout(3000);

    // Close context 1 to prove we aren't relying on local storage across sessions
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

    // The backend should restore the state and auto-advance, or at least fill the inputs
    const input_box = page2.getByPlaceholder('e.g. I bake custom vegan cakes in Portland, OR...');
    await expect(input_box).toHaveValue('I bake custom vegan cakes', { timeout: 15000 });

    // Continue the flow
    await page2.route('**/api/onboarding/intake', async route => route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
            business_name: "Maya's Cakes",
            business_type: "Bakery",
            categories: ["food", "physical"],
            initial_products: [{ name: "Custom Vegan Cake", price: "45.00" }]
        })
    }));
    await page2.waitForSelector('button:not([disabled])');
    await page2.getByRole('button', { name: /Generate My Business/i }).click();

    // Wait for step 2 (Review Details)
    await expect(page2.getByRole('heading', { name: 'Review Details' })).toBeVisible({ timeout: 15000 });

    // Modify a value
    const businessNameInput = page2.locator('label', { hasText: 'Business Name' }).locator('..').locator('input');
    await businessNameInput.fill('Maya\'s Cross-Device Bakery');

    // Wait for debounce save
    await page2.waitForTimeout(3000);
    await context2.close();

    // 4. Open yet another browser session (Device 3)
    const context3 = await browser.newContext();
    const page3 = await context3.newPage();

    await page3.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-cross-device-tenant');
      localStorage.setItem('user_id', 'e2e-cross-device-user');
    });

    await page3.goto('/onboarding');
    await expect(page3.locator('#setup-screen')).toBeVisible({ timeout: 15000 });

    // We should resume directly to step 2 with the updated business name
    await expect(page3.getByRole('heading', { name: 'Review Details' })).toBeVisible({ timeout: 15000 });
    const businessNameInput3 = page3.locator('label', { hasText: 'Business Name' }).locator('..').locator('input');
    await expect(businessNameInput3).toHaveValue('Maya\'s Cross-Device Bakery');

    await context3.close();
  });
});
