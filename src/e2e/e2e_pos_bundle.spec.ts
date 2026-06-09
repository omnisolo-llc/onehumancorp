import { test, expect } from '@playwright/test';

test.describe('In-Person Payment (POS) Flow - Offline Bundling', () => {
  test('should complete a tap-to-pay transaction offline with bundle pricing', async ({ page, context }) => {
    // Navigate to a safe api route first to set local storage before loading the main page
    await page.goto('/api/staff');

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

    // Navigate to the POS terminal page
    await page.goto('/pos/terminal');
    await expect(page.locator('text=Terminal Locked')).toBeVisible({ timeout: 15000 });

    // Enter PIN: 1234
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    // Verify unlocked and shows staff name
    await expect(page.locator('text=Carlos')).toBeVisible();

    // Set network to offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Trigger New Order
    await page.getByRole('button', { name: 'New Order' }).click();

    // Verify Payment total and offline
    await expect(page.locator('text=Payment Saved Offline - 50 USD')).toBeVisible();

    // Restore network
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));
  });
});
