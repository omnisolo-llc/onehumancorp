import { test, expect } from './fixtures';

test.describe('Zero-Dashboard Activity Feed', () => {
  test('Grandmother Test: User sees actionable Agent Activity Feed on dashboard', async ({ page }) => {
    // Navigate to the Dashboard
    await page.goto('/dashboard');

    // Check that we see the 'Agent Activity Feed' heading instead of generic updates
    await expect(page.getByRole('heading', { name: 'Agent Activity Feed' })).toBeVisible();

    // Look for an item that is mapped from the mock API or existing approvals
    // The feed data might take a second to load, wait for elements.
    const editButton = page.locator('button', { hasText: 'Edit' }).first();
    const approveButton = page.locator('button', { hasText: 'Approve & Send' }).first();

    await expect(editButton).toBeVisible();
    await expect(approveButton).toBeVisible();

    // Verify visual constraints: Plain language and no "Advanced Settings" technical payload by default
    const advancedPayload = page.locator('text=Technical Payload:');
    await expect(advancedPayload).toBeHidden();
  });
});
