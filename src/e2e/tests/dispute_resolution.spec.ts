import { test, expect } from '@playwright/test';

test.describe('Autonomous AI Dispute and Chargeback Resolution Engine', () => {
    test('Should display chargeback alert with compiled evidence and allow approval', async ({ page, request }) => {
        // First log in
        await page.goto('/');
        await page.fill('input[type="email"]', 'test-owner@example.com');
        await page.fill('input[type="password"]', 'password123');
        await page.click('button:has-text("Log in")');
        await expect(page).toHaveURL('/dashboard');

        // Simulate a dispute webhook
        const disputePayload = {
            type_field: 'charge.dispute.created',
            data: {
                object: {
                    id: 'dp_12345',
                    charge: 'ch_67890',
                    amount: 15000, // $150
                    currency: 'usd',
                    reason: 'product_not_received',
                    status: 'needs_response',
                    metadata: {
                        tenant_id: 'test-tenant-id'
                    }
                }
            }
        };

        const response = await request.post('/api/v1/webhooks/stripe', {
            data: disputePayload
        });

        expect(response.ok()).toBeTruthy();

        // Wait for job queue processing
        await page.waitForTimeout(10000);

        // Reload the feed
        await page.reload();

        // Verify the alert card is visible
        const alertCard = page.locator('text=🚨 Chargeback Alert');
        await expect(alertCard).toBeVisible({ timeout: 15000 });

        // Click to view details
        await alertCard.click();

        // Verify evidence package is displayed
        await expect(page.locator('text=Product Not Received')).toBeVisible();
        await expect(page.locator('text=Evidence Package')).toBeVisible();
        await expect(page.locator('text=Customer signed via mobile app')).toBeVisible();

        // Approve submission
        const submitButton = page.locator('button:has-text("Submit Evidence to Bank")');
        await expect(submitButton).toBeVisible();
        await submitButton.click();

        // Verify success state
        await expect(page.locator('text=Evidence Submitted Successfully')).toBeVisible();
    });
});
