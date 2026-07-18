import { test, expect } from './fixtures';

test.describe('Intelligent Accounts Receivable & Dunning Engine', () => {
    test('should display overdue invoice nudge in triage and allow approval', async ({ page }) => {
        // Simulate the overdue invoice
        const res = await page.request.post('/api/dev/simulate-invoice-followup');
        expect(res.ok()).toBeTruthy();

        // Navigate to dashboard
        await page.goto('/dashboard.html');

        // Wait for the specific triage card
        const card = page.getByTestId('invoice-followup-card').first();
        await expect(card).toBeVisible({ timeout: 10000 });

        // Verify the title
        await expect(card.getByText('Action Required: Overdue Invoice')).toBeVisible();

        // Verify the nudge content
        await expect(card.getByText('Invoice INV-1029 is overdue.')).toBeVisible();

        // Click Approve & Send
        const approveBtn = card.getByText('Approve & Send');
        await expect(approveBtn).toBeVisible();
        await approveBtn.click();

        // The card should disappear after approval (status changes to APPROVED and is filtered out of pending)
        await expect(card).not.toBeVisible({ timeout: 10000 });
    });
});
