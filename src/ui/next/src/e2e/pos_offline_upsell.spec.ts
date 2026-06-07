import { test, expect } from '@playwright/test';

test.describe('Mobile POS Offline Dynamic Bundling', () => {
  test('evaluates bundles and applies upsells locally while offline', async ({ page, context }) => {
    // 1. Setup offline mode and load page
    await context.setOffline(false);
    await page.goto('/pos/terminal');

    // Set a known active staff to bypass PIN screen
    await page.evaluate(() => {
      localStorage.setItem('ohc_offline_staff', JSON.stringify([
        { id: 'staff_1', pin_hash: '1234', name: 'Fatima', role: 'Owner' }
      ]));
      localStorage.setItem('ohc_offline_inventory', JSON.stringify([
        { id: 'prod_shawarma', name: 'Chicken Shawarma', inventory_count: 50 },
        { id: 'prod_drink', name: 'Drink', inventory_count: 50 },
        { id: 'prod_fries', name: 'Fries', inventory_count: 5 }
      ]));
      localStorage.setItem('ohc_offline_rules', JSON.stringify([
        {
          id: "meal_deal",
          trigger_items: ["prod_shawarma"],
          upsell_items: ["prod_drink", "prod_fries"],
          bundled_price_cents: 8000,
          upsell_prompt: "Make it a combo? +$3.00 for Drink and Fries."
        }
      ]));
    });

    await page.reload();

    // Simulate PIN entry
    await page.click('text=1');
    await page.click('text=2');
    await page.click('text=3');
    await page.click('text=4');

    // Wait for staff UI to appear
    await expect(page.locator('text=Fatima')).toBeVisible();

    // 2. Go offline
    await context.setOffline(true);

    // Give react time to trigger offline listener
    await page.waitForTimeout(500);

    // Verify offline badge is visible
    await expect(page.locator('span:has-text("Offline Mode")').first()).toBeVisible();

    // 3. Add to cart ("Chicken Shawarma")
    // Use evaluate to click using DOM API if Playwright locator fails due to complex DOM
    await page.evaluate(() => {
        const buttons = Array.from(document.querySelectorAll('button'));
        const btn = buttons.find(b => b.textContent?.includes('Add Chicken Shawarma'));
        if (btn) btn.click();
    });

    // 4. Verify the macOS translucent upsell modal appears
    const modal = page.locator('h3:has-text("Special Offer!")');
    await expect(modal).toBeVisible();
    await expect(page.locator('text=Make it a combo? +$3.00 for Drink and Fries.')).toBeVisible();

    // 5. Accept the upsell
    await page.click('text=Accept');

    // 6. Verify cart updates to bundled price
    await expect(page.locator('text=Current Order')).toBeVisible();
    await expect(page.locator('text=1x Chicken Shawarma')).toBeVisible();
    await expect(page.locator('text=1x Drink')).toBeVisible();
    await expect(page.locator('text=1x Fries')).toBeVisible();

    // Total should be $80.00 (since 8000 cents / 100 = 80)
    // Wait, in my test rules I set it to 8000 cents. Let's make sure it shows $80.00
    await expect(page.locator('text=$80.00')).toBeVisible();

    // 7. Checkout offline
    await page.click('text=Checkout');

    // Verify offline success message
    await expect(page.locator('text=Payment Saved Offline')).toBeVisible();
  });
});
