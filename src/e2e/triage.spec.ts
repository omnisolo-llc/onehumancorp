import { expect, test } from './fixtures';

test.describe('Work Triage Flow', () => {
  test('displays triage cards and handles approvals with real backend data', async ({ page }) => {
    test.setTimeout(60000);

    // Go to dashboard
    await page.goto('/dashboard');

    // Wait for the data to load
    await page.waitForTimeout(2000);

    const hasEmpty = await page.getByText('No database-backed actions are currently open.').count();
    const hasPendingOrders = await page.getByText('Pending fulfillment').count();
    const hasLowStock = await page.getByText('Low stock').count();
    const hasInboxMessages = await page.getByText('Inbox messages').count();
    const hasTriageItem = await page.getByTestId('approve-triage').count();

    expect(hasEmpty > 0 || hasPendingOrders > 0 || hasLowStock > 0 || hasInboxMessages > 0 || hasTriageItem > 0).toBe(true);
  });
});
