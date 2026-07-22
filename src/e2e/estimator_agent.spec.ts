import { test, expect } from '@playwright/test';

test.describe('Estimator Agent Quoting Flow (Mobile)', () => {
    test.use({
        viewport: { width: 375, height: 667 }, // Mobile-first constraint
        userAgent: 'Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Mobile/15E148 Safari/604.1',
    });

    test('intercepts inquiry, generates quote, and allows one-tap approval', async ({ page }) => {
        // Step 1: Trigger the incoming inquiry via the UI contact form
        await page.goto('/ui/storefront.html');
        await page.locator('text=Contact Us').click();
        await page.fill('input[name="email"]', 'test-customer-123@example.com');
        await page.fill('textarea[name="message"]', 'Need kitchen painted, 200 sq ft');
        await page.click('button[type="submit"]');

        // Step 2: Owner logs in
        await page.goto('/ui/kairos.html');
        // Ensure app loads
        await expect(page.locator('text=OmniSolo').first()).toBeVisible({ timeout: 10000 });

        // Look for the action card title specific to quotes
        const draftReadyTitle = page.locator('text=Draft Quote Ready').first();
        await expect(draftReadyTitle).toBeVisible({ timeout: 15000 });

        // Ensure line items or project description appears
        // The mocked context uses "Kitchen Painting"
        await expect(page.locator('text=Kitchen Painting').first()).toBeVisible();

        // Step 4: The owner taps "Approve & Request Deposit"
        const approveBtn = page.getByTestId('feed-approve-btn');
        await expect(approveBtn).toHaveText('Approve & Request Deposit');

        // Setup interception to verify the stripe link generation call if needed,
        // or just click and ensure success UI
        await approveBtn.click();

        // Verify the card disappears from the feed after approval
        await expect(draftReadyTitle).not.toBeVisible();
    });
});
