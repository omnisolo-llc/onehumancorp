import { test, expect } from '@playwright/test';

test.describe('Proactive Agent Feed', () => {
  test.beforeEach(async ({ page }) => {
    // Mock the backend feed endpoint to return test action cards
    await page.route('**/api/agent-feed*', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          pending_actions: [
            {
              id: 'test-action-1',
              tenant_id: 'test-tenant',
              agent_id: 'MarketingAgent',
              action_type: 'draft_campaign',
              payload: { description: 'Drafted an email for 3 abandoned carts' },
              status: 'pending'
            },
            {
              id: 'test-action-2',
              tenant_id: 'test-tenant',
              agent_id: 'CustomerSuccessAgent',
              action_type: 'reply_inquiry',
              payload: { description: 'Vegan cake availability inquiry' },
              status: 'pending'
            }
          ]
        })
      });
    });
  });

  test('should display proactive action cards on the dashboard', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Verify Agent Feed header exists
    await expect(page.getByRole('heading', { name: 'Agent Feed' })).toBeVisible();

    // Verify Action Cards are displayed
    await expect(page.getByText('Drafted an email for 3 abandoned carts')).toBeVisible();
    await expect(page.getByText('Vegan cake availability inquiry')).toBeVisible();

    // Verify badges
    await expect(page.getByText('MarketingAgent')).toBeVisible();
    await expect(page.getByText('CustomerSuccessAgent')).toBeVisible();

    // Verify interaction buttons
    const approveBtns = await page.getByRole('button', { name: 'Approve action' }).all();
    expect(approveBtns.length).toBe(2);

    const dismissBtns = await page.getByRole('button', { name: 'Dismiss action' }).all();
    expect(dismissBtns.length).toBe(2);

    // Click Dismiss on the first card
    await dismissBtns[0].click();

    // Verify optimistic UI update removed the card
    await expect(page.getByText('Drafted an email for 3 abandoned carts')).not.toBeVisible();
    await expect(page.getByText('Vegan cake availability inquiry')).toBeVisible();
  });
});
