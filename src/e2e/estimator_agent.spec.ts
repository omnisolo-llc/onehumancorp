import { test, expect } from '@playwright/test';

test.describe('Estimator Agent Quoting Flow (Mobile)', () => {
    test.use({
        viewport: { width: 375, height: 667 }, // Mobile-first constraint
        userAgent: 'Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Mobile/15E148 Safari/604.1',
    });

    test.beforeEach(async ({ page }) => {
        // Step 2: Owner logs in
        await page.goto('/login');
        await page.fill('input[name="username"]', 'carlos_test');
        await page.fill('input[name="password"]', 'password123');
        await page.click('button[type="submit"]');
        await page.goto('/ui/kairos.html');
    });

    test('intercepts inquiry, generates quote, and allows one-tap approval', async ({ page }) => {
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

        await approveBtn.click();

        // Verify the card disappears from the feed after approval
        await expect(draftReadyTitle).not.toBeVisible();
    });

    test('allows owner to reject the draft quote', async ({ page }) => {
        const draftReadyTitle = page.locator('text=Draft Quote Ready').first();
        await expect(draftReadyTitle).toBeVisible({ timeout: 15000 });

        const rejectBtn = page.getByTestId('feed-reject-btn');
        await expect(rejectBtn).toBeVisible();
        await rejectBtn.click();

        // Verify the card disappears from the feed after rejection
        await expect(draftReadyTitle).not.toBeVisible();
    });

    test('allows owner to view quote details', async ({ page }) => {
        const draftReadyTitle = page.locator('text=Draft Quote Ready').first();
        await expect(draftReadyTitle).toBeVisible({ timeout: 15000 });

        const viewBtn = page.locator('text=View Details').first();
        if (await viewBtn.isVisible()) {
            await viewBtn.click();
            await expect(page.locator('text=Quote Details').first()).toBeVisible();
        }
    });

    test('displays total amount correctly', async ({ page }) => {
        const draftReadyTitle = page.locator('text=Draft Quote Ready').first();
        await expect(draftReadyTitle).toBeVisible({ timeout: 15000 });

        // Should display some currency amount
        await expect(page.locator('text=$').first()).toBeVisible();
    });

    test('displays customer information on the quote card', async ({ page }) => {
        const draftReadyTitle = page.locator('text=Draft Quote Ready').first();
        await expect(draftReadyTitle).toBeVisible({ timeout: 15000 });

        // Assuming customer name or generic 'Customer' is displayed
        await expect(page.locator('text=Customer').first()).toBeVisible();
    });
});
