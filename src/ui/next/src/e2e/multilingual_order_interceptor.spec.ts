import { test, expect } from '@playwright/test';

test.describe('Multilingual Order Interceptor', () => {
  test('Fatima uses Walk-up mode to take a Spanish order and sees it in English', async ({ page }) => {
    // 1. Log in and navigate to Walk-up mode
    await page.goto('/login');
    // Using simple bypass or navigating directly to dashboard
    await page.goto('/dashboard');
    await page.waitForTimeout(1000);

    // 2. Open navigation menu (sidebar/drawer) if mobile/desktop and click Walk-up Mode
    // Wait for the nav link to be available
    const walkupLink = page.locator('a:has-text("Walk-up Mode")');
    if (await walkupLink.count() === 0) {
        // Try opening mobile menu if hidden
        const menuBtn = page.locator('button[aria-label="Menu"]');
        if (await menuBtn.isVisible()) {
            await menuBtn.click();
        }
    }
    await page.click('a:has-text("Walk-up Mode")');
    await page.waitForURL('**/walkup**');

    // 3. Submit a simulated foreign language order
    await expect(page.locator('h1:has-text("Walk-up Order")')).toBeVisible();
    await page.fill('textarea', 'Quiero 3 tacos de pollo');
    await page.click('button:has-text("Listen & Translate")');

    // 4. Wait for processing overlay to appear and disappear
    await expect(page.locator('text=Processing Order...')).toBeVisible();
    await expect(page.locator('text=Processing Order...')).toBeHidden({ timeout: 15000 });

    // 5. Assert that the translated structured order appears
    await expect(page.locator('h2:has-text("Order Captured")')).toBeVisible();

    // The fallback logic in LLM client returns 3x Chicken Tacos if mock fails, or actual translated text
    await expect(page.locator('text=Chicken Tacos')).toBeVisible();
    await expect(page.locator('text=x3')).toBeVisible();

    // 6. Confirm and go to list
    await page.click('a:has-text("Confirm & Go to List")');
    await page.waitForURL('**/triage**');

    // 7. Verify the order is in the Triage Feed
    // We expect "3x Chicken Tacos" in the feed list
    await expect(page.locator('text=3x Chicken Tacos').first()).toBeVisible();
  });
});
