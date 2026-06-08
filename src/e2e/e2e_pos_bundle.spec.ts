import { test, expect } from '@playwright/test';

test.describe('In-Person Payment (POS) Flow - Offline Bundling', () => {
  test('should complete a tap-to-pay transaction offline with bundle pricing', async ({ page, context }) => {
    // Navigate to the POS terminal page
    await page.goto('/pos/terminal');

    // Wait for the UI to load and auto-fetch the staff data
    await page.waitForResponse(response => response.url().includes('/api/staff') && response.status() === 200);

    await expect(page.locator('text=Terminal Locked')).toBeVisible();

    // Setup local storage mock for offline staff, rules, and inventory
    await page.evaluate(() => {
        localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'Carlos', role: 'Manager', pin_hash: '1234' }]));
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

    // Trigger Add Chicken Shawarma
    await page.locator('text=Chicken Shawarma').click();

    // Check if the upsell is displayed
    await expect(page.locator('text=Make it a combo? +$3.00 for Drink and Fries.')).toBeVisible();
    await expect(page.locator('text=5 in stock')).toBeVisible();

    // Accept combo
    await page.getByRole('button', { name: 'Accept' }).click();

    // Checkout
    await page.getByRole('button', { name: 'Checkout' }).click();

    // Verify Payment total and offline
    await expect(page.locator('text=Payment Saved Offline - 11 USD')).toBeVisible();

    // Restore network
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));
  });
});
