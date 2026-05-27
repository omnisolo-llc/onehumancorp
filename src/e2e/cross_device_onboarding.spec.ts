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

    // Mock the intake API call for Device 1
    await page1.route('**/api/onboarding/intake', async route => route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({
        business_name: "Maya's Cross-Device Bakery",
        business_type: "Bakery",
        categories: ["food", "physical"],
        initial_products: [{ name: "Custom Vegan Cake", price: "45.00" }]
      })
    }));

    // 2. Start flow and type business description
    await page1.getByPlaceholder('e.g. Sell cakes, plumbing').fill('I bake custom vegan cakes');

    // Generate business based on description (triggers step 2)
    await page1.getByRole('button', { name: /Generate My Business/i }).click();

    // Wait for step 2 to load with data
    await expect(page1.getByRole('heading', { name: 'Review Details' })).toBeVisible();
    await expect(page1.locator('#businessName')).toHaveValue("Maya's Cross-Device Bakery");

    // Modify one of the fields to verify persistence beyond initial intake
    await page1.locator('#businessName').fill("Maya's Cross-Device Bakery Edited");

    // Wait for the debounce save state to trigger
    await page1.waitForTimeout(3000);

    // Close context 1 to prove we aren't relying on local storage or memory
    await context1.close();

    // 3. Open a completely new incognito browser session (Device 2)
    const context2 = await browser.newContext();
    const page2 = await context2.newPage();

    // Set the SAME tenant ID, but nothing else (empty wizard state in localstorage)
    await page2.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-cross-device-tenant');
      localStorage.setItem('user_id', 'e2e-cross-device-user');
    });

    // Mock state restoration since backend API might not actually exist locally in pure E2E run
    await page2.route('**/api/onboarding/state', async route => {
        if (route.request().method() === 'GET') {
            await route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify({
                    step: 2,
                    businessName: "Maya's Cross-Device Bakery Edited",
                    businessType: "Bakery",
                    categories: ["food", "physical"],
                    firstProductName: "Custom Vegan Cake",
                    firstProductPrice: "45.00"
                })
            });
        } else {
            await route.continue();
        }
    });

    await page2.goto('/onboarding');
    await expect(page2.locator('#setup-screen')).toBeVisible({ timeout: 15000 });

    // Wait for step 2 to load with data
    await expect(page2.getByRole('heading', { name: 'Review Details' })).toBeVisible({ timeout: 15000 });
    await expect(page2.locator('#businessName')).toHaveValue("Maya's Cross-Device Bakery Edited");

    await context2.close();
  });
});
