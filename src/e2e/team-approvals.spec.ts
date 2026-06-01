import { test, expect } from './fixtures';

test.describe('Team Approvals', () => {
  test('should display and approve automated_review_request', async ({ page }) => {
    // Navigate to Team page
    await page.goto('/team');

    // Check that we are on the Team page
    await expect(page.locator('h1', { hasText: 'Your Team' })).toBeVisible({ timeout: 15000 });

    // Wait for the department list to load by waiting for the spinner to disappear if it's there
    await expect(page.locator('.animate-spin')).toBeHidden({ timeout: 15000 });

    // Click on Customer Success department card (The Ambassador)
    // The h3 element is rendering `{name}` which is 'The Ambassador'
    const ambassadorHeader = page.locator('h3', { hasText: 'The Ambassador' });
    await expect(ambassadorHeader).toBeVisible({ timeout: 15000 });

    // Since the h3 is inside the button, we can click the button
    const ambassadorButton = ambassadorHeader.locator('xpath=ancestor::button').first();
    await ambassadorButton.click();

    // Now we are in the Approval Inbox for Customer Success
    // Wait for the new block to appear
    await expect(page.locator('text=Automated Review Request')).toBeVisible({ timeout: 15000 });

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
    await expect(page.locator('text=Automated Review Request')).toBeHidden({ timeout: 15000 });
  });
});
