import { test, expect } from './fixtures';

test.describe('Omni-Channel Work Triage Feed', () => {
  test('loads triage feed and allows approval of drafts natively on the dashboard', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // We expect the Triage widget ("Needs Your Attention") to be present
    await expect(page.getByText('Needs Your Attention')).toBeVisible();

    // Verify our seeded items are shown correctly
    await expect(page.getByText('Urgent: I need a cake delivered tomorrow!')).toBeVisible();

    // Tap Approve
    const approveButton = page.locator('button').filter({ hasText: '✨ Approve' }).first();
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // Item state optimistic updates or refreshes, verify no errors
    await expect(page.locator('body')).not.toContainText('Error approving message.');

    // Navigate to full Triage view
    await page.getByRole('link', { name: 'Open Triage' }).click();
    await expect(page.getByRole('heading', { name: 'Triage' })).toBeVisible();

    // See the remaining triage items (or updated statuses)
    await expect(page.getByText('Do you do cupcakes?')).toBeVisible();

    // Test approving from the detailed Triage page
    await page.getByRole('button', { name: /Do you do cupcakes\?/i }).click();
    await expect(page.getByText('Proposed Action / Draft Reply')).toBeVisible();

    const secondApproveButton = page.getByRole('button', { name: '✨ Approve' });
    await expect(secondApproveButton).toBeVisible();
    await secondApproveButton.click();

    await expect(page.getByText('Draft approved and sent.')).toBeVisible();
  });
});
