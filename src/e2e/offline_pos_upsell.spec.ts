import { test, expect } from '@playwright/test';

test.describe('Offline POS Dynamic Bundle & Upsell', () => {
  test('should propose an upsell offline and apply bundle pricing', async ({ page, context }) => {
    await page.goto('/pos/terminal');

    // Wait for the UI to load and auto-fetch the staff data
    await page.waitForResponse(response => response.url().includes('/api/staff') && response.status() === 200);

    await expect(page.locator('text=Terminal Locked')).toBeVisible();

    // Setup local storage mock for offline rules and inventory
    await page.evaluate(() => {
        localStorage.setItem('ohc_offline_rules', JSON.stringify([
            {
                id: 'rule_1',
                trigger_product_id: 'prod_shawarma',
                upsell_product_id: 'prod_drink_fries',
                upsell_price_cents: 300,
                prompt_message: 'Make it a combo? +$3.00 for Drink and Fries.',
            }
        ]));
        localStorage.setItem('ohc_offline_inventory', JSON.stringify([
            { id: 'prod_shawarma', name: 'Chicken Shawarma', inventory_count: 50, price_cents: 800 },
            { id: 'prod_drink_fries', name: 'Drink and Fries', inventory_count: 5, price_cents: 500 }
        ]));
    });

    // Enter PIN: 1234
    await page.getByRole('button', { name: '1' }).click();
    await page.getByRole('button', { name: '2' }).click();
    await page.getByRole('button', { name: '3' }).click();
    await page.getByRole('button', { name: '4' }).click();

    // Verify unlocked and shows staff name
    await expect(page.locator('text=Carlos')).toBeVisible();

    // Set network to offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Add Chicken Shawarma to cart
    await page.locator('text=Chicken Shawarma').click();

    // Wait for upsell prompt
    await expect(page.locator('text=Make it a combo? +$3.00 for Drink and Fries.')).toBeVisible();
    await expect(page.locator('text=5 left in stock')).toBeVisible();

    // Accept upsell
    await page.getByRole('button', { name: 'Accept' }).click();

    // Verify cart total ($8.00 + $3.00 = $11.00)
    await expect(page.locator('text=$11.00')).toBeVisible();

    // Checkout
    await page.getByRole('button', { name: 'Checkout & Pay' }).click();

    // Verify Payment Saved Offline
    await expect(page.locator('text=Payment Saved Offline - 11 USD')).toBeVisible();

    // Restore network
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));
  });
});
