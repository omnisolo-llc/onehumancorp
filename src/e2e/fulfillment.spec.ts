import { test, expect } from '@playwright/test';

test.describe('Fulfillment Engine', () => {
    test('displays dynamic fulfillment options on checkout', async ({ page }) => {
        await page.goto('/pricing');

        const checkoutBtn = page.getByRole('button', { name: 'Upgrade to Starter via Stripe' });
        await expect(checkoutBtn).toBeVisible();
        await checkoutBtn.click();

        await expect(page.getByRole('heading', { name: 'Checkout' })).toBeVisible();

        const fulfillmentOptions = page.locator('#fulfillment-options-container', { hasText: /Shipping|Local Delivery|Pickup/ });
        await expect(fulfillmentOptions).toBeVisible();

        const payBtn = page.getByRole('button', { name: 'Pay Now' });
        await expect(payBtn).toBeVisible();

        page.on('dialog', dialog => dialog.accept());
        await payBtn.click();
    });
});
