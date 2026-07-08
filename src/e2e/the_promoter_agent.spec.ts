import { test, expect } from './fixtures';

test.describe('Promoter Agent E2E', () => {
  test('Persona: Maya approves a social post proposal', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);

    // Seed the database with a pending promoter approval
    await page.request.post('/api/v1/agents/approvals/simulate-promoter-draft', {
      headers: {
        Authorization: `Bearer ${await page.evaluate(() => localStorage.getItem('token'))}`
      }
    });

    await page.goto('/promoter');

    // Wait for the UI to load and display the new proposal
    await expect(page.locator('text="New Product Detected!"').first()).toBeVisible({ timeout: 10000 });

    // Click on the proposal to view details
    await page.locator('text="New Product Detected!"').first().click();

    // Ensure the proposal details are shown
    await expect(page.locator('h2', { hasText: 'Promoter Proposal' })).toBeVisible();
    await expect(page.locator('text="Approve & Publish"')).toBeVisible();

    // Approve the proposal
    await page.locator('text="Approve & Publish"').click();

    // The proposal should be removed and we should see the empty state
    await expect(page.locator('text="No new proposals generated."')).toBeVisible({ timeout: 10000 });
  });
});
