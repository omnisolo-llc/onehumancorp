import { test, expect } from '@playwright/test';

test.describe('POS Tap-to-Pay Flow', () => {
    test('verifies catalog, cart updates, and successful checkout', async ({ page }) => {
        // Mock backend API
        await page.route('**/api/v1/pos/session', async route => {
            await route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify({ session_token: 'mock_token', expires_at: 'mock_date' })
            });
        });

        await page.route('**/api/v1/pos/transaction', async route => {
            await route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify({ success: true, transaction_id: 'mock_txn' })
            });
        });

        // Go to POS page directly
        await page.goto('http://localhost:3000/pos');

        // Check page title and styling elements
        await expect(page.locator('h1').filter({ hasText: 'Point of Sale' })).toBeVisible();

        // Check if catalog is rendered
        await expect(page.locator('text=Custom Cake')).toBeVisible();
        await expect(page.locator('text=Artisan Bread')).toBeVisible();

        // The cart should initially be empty
        await expect(page.locator('text=Cart is empty')).toBeVisible();
        const payButton = page.locator('button:has-text("Tap to Pay")');
        await expect(payButton).toBeDisabled();

        // Add "Custom Cake" to cart
        await page.locator('text=Custom Cake').click();

        // Cart should update
        await expect(page.locator('text=1x Custom Cake')).toBeVisible();
        await expect(page.locator('h2.font-bold', { hasText: 'Current Sale' })).toBeVisible();
        await expect(payButton).toBeEnabled();

        // Add another item
        await page.locator('text=Artisan Bread').click();
        await expect(page.locator('text=1x Artisan Bread')).toBeVisible();

        // Mock window.alert for tap to pay interactions
        let alerts: string[] = [];
        page.on('dialog', dialog => {
            alerts.push(dialog.message());
            dialog.accept();
        });

        // Click Pay
        await payButton.click();

        // Wait for the simulated transaction to complete
        await page.waitForTimeout(2000);

        // Verify the mock native UI and success alerts were shown
        expect(alerts).toContain('Mock Native UI: Please hold card near phone...');
        expect(alerts).toContain('Payment successful!');

        // Cart should be empty again
        await expect(page.locator('text=Cart is empty')).toBeVisible();
    });
});
