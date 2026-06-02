import { test, expect } from '@playwright/test';

test.describe('Predictive Supply Chain E2E', () => {
    test('Business owner can approve a predictive PO', async ({ page }) => {
        // Mock the PO approval API call so we don't need real backend side-effects
        // since we are testing the UI flow.
        await page.route('/api/supply/approve_po', async route => {
            const json = { success: true };
            await route.fulfill({ status: 200, json });
        });

        await page.goto('/');

        // Login as business owner (assuming simple flow based on other E2E tests, maybe just wait for dashboard)
        // Ensure dashboard is visible
        await page.waitForSelector('#dashboard-screen', { state: 'visible' });

        // Navigate to the supply chain / inventory if needed, or check dashboard card
        const poCard = page.locator('#restock-predictions-widget');
        await expect(poCard).toBeVisible();

        const approveBtn = poCard.locator('button', { hasText: 'Approve & Send PO' });
        await expect(approveBtn).toBeVisible();

        // Intercept dialogs
        page.once('dialog', dialog => {
            expect(dialog.message()).toContain('PO Sent');
            dialog.accept();
        });

        await approveBtn.click();
    });
});
