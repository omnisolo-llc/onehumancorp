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

    // Trigger a backend action dispatch to add an approval request explicitly for tests
    await page.evaluate(async () => {
      await fetch('/api/agents/mission/dispatch', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-tenant-id': 'e2e-tenant',
          'x-user-id': 'e2e-admin-user',
        },
        body: JSON.stringify({
          action_type: "DraftCustomerMessage",
          action_description: "Draft reply about vegan cakes",
          risk_level: "HIGH",
          payload: {
            customer_id: "e2e-customer-1",
            message_body: "Yes, we have vegan options!"
          }
        })
      });
    });

    // Reload to see the new proposal
    await page.reload();
    await expect(page.locator('button', { hasText: 'Proposals' }).first()).toBeVisible();

    await expect(page.getByText(/All caught up!|Requires Review|Loading Agent Proposals/).first()).toBeVisible();

    // The new message should be present
    await expect(page.getByText(/Message Preview/)).toBeVisible();
    await expect(page.getByText(/Yes, we have vegan options!/)).toBeVisible();

    // Approve the message
    await page.getByTestId('approve-send-message').first().click();

    // The proposal should disappear
    await expect(page.getByText(/Yes, we have vegan options!/)).not.toBeVisible();

    await page.getByRole('button', { name: 'Activity Feed' }).click();
    await expect(page.getByRole('button', { name: 'Activity Feed' })).toBeVisible();
  });
});
