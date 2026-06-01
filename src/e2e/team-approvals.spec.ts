import { test, expect } from './fixtures';

test.describe('Team Approvals', () => {
  test('should display and approve automated_review_request', async ({ page }) => {
    // Navigate to Team page
    await page.goto('/team');

    // Check that we are on the Team page
    await expect(page.locator('h1', { hasText: 'Your Team' })).toBeVisible();

    // Click on Customer Success department card (The Ambassador)
    // Wait for the department list to load
    await expect(page.locator('h3', { hasText: 'The Ambassador' })).toBeVisible();
    await page.locator('h3', { hasText: 'The Ambassador' }).click();

    // Now we are in the Approval Inbox for Customer Success
    // Wait for the new block to appear
    await expect(page.locator('text=Automated Review Request')).toBeVisible();

    // Check that the target details are displayed
    await expect(page.locator('text=Targeting: recent unreviewed orders')).toBeVisible();

    // Check that the payload count is displayed
    await expect(page.locator("text=3 customers haven't reviewed their orders. Request reviews?")).toBeVisible();

    // Click approve
    // There are multiple approvals potentially, we want the one related to automated_review_request
    // We can target the approve button within the parent card.
    const card = page.locator('div', { hasText: 'Automated Review Request' }).filter({ hasText: 'Targeting: recent unreviewed orders' }).locator('xpath=ancestor::div[contains(@class, "bg-white/65")]').first();
    const approveButton = card.locator('button', { hasText: 'Approve' });
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // Verify it disappears (approvals state updates)
    await expect(page.locator('text=Automated Review Request')).toBeHidden();
  });
});
