import { test, expect } from './fixtures';

test.describe('Unified Agent Feed', () => {
  test('should display agent feed and allow interaction', async ({ page }) => {

    // Ensure we are using the seeded e2e tenant explicitly to fetch the seed data
    await page.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-tenant');
      localStorage.setItem('user_id', 'e2e-admin-user');
    });

    // Go to dashboard
    await page.goto('/dashboard');

    // Verify we are on dashboard and the Unified Agent Feed is present
    await expect(page.locator('button', { hasText: 'Proposals' }).first()).toBeVisible();

    await expect(page.getByText(/All caught up!|Requires Review|Loading Agent Proposals/).first()).toBeVisible();

    // Verify the "Draft email for review" proposal exists from the seed data
    const proposalCard = page.locator('div.glassmorphism').filter({ hasText: 'Draft email for review' }).first();
    await expect(proposalCard).toBeVisible();

    // Click Approve on the general proposal
    const approveButton = proposalCard.getByTestId('approve-proposal');
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // Optimistic UI update should remove the card
    await expect(proposalCard).not.toBeVisible();

    await page.getByRole('button', { name: 'Activity Feed' }).click();
    await expect(page.getByRole('button', { name: 'Activity Feed' })).toBeVisible();
  });
});
