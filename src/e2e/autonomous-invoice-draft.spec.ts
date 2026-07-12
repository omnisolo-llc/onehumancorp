import { test, expect } from './fixtures';

test.describe('Autonomous Invoice Generation Engine', () => {
    test('should display draft invoice in feed and allow approval', async ({ page }) => {
        // Simulate the draft invoice creation
        const res = await page.request.post('/api/dev/simulate-invoice-draft');
        expect(res.ok()).toBeTruthy();

        // Navigate to dashboard
        await page.goto('/dashboard.html');

        // Wait for the specific triage card
        const card = page.locator('div', { hasText: 'Draft Invoice ready for Phase 1 Complete' }).first();
        await expect(card).toBeVisible({ timeout: 10000 });

        // Verify the line items and amount
        await expect(card.getByText('Phase 1 Design')).toBeVisible();
        await expect(card.getByText('$25.00')).toBeVisible();

        // Verify the email preview
        await expect(card.getByText('Hi team, attached is the invoice for the completion of the design phase...')).toBeVisible();

        // Click Approve & Send
        const approveBtn = card.getByText('Approve & Send');
        await expect(approveBtn).toBeVisible();
        await approveBtn.click();

        // The card should disappear after approval
        await expect(card).not.toBeVisible({ timeout: 10000 });
    });
});
