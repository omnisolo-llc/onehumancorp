import { test, expect } from '@playwright/test';

test.describe('Proactive Ambassador CUJ', () => {
  test('displays abandoned cart recovery action card and allows approval', async ({ page }) => {
    // Navigate to the login page and authenticate to access the dashboard
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('maya@ohc.test');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Login' }).click();

    // Verify successful login
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // Intercept /api/agents/approvals to return our mocked Ambassador approval
    await page.route('/api/agents/approvals*', async route => {
      const json = {
        pending_approvals: [
          {
            id: 'mock_abandoned_cart_1',
            tenant_id: 'test_tenant',
            department: 'The Ambassador',
            description: 'Recover Abandoned Carts',
            status: 'pending',
            action_risk: 'low',
            payload: {
              abandoned_carts_count: 3,
              potential_revenue: 120,
              draft_message: "Hey there! We noticed you left some items in your cart. Here's a 10% discount to complete your order!"
            }
          }
        ]
      };
      await route.fulfill({ json });
    });

    // Mock the approval POST endpoint
    await page.route('/api/agents/approvals/mock_abandoned_cart_1', async route => {
        if (route.request().method() === 'POST') {
            await route.fulfill({ status: 200, json: { success: true } });
        } else {
            await route.continue();
        }
    });

    // Go to the dashboard
    await page.goto('/dashboard');

    // Verify the feed renders the "Agent Proposals" section
    await expect(page.getByRole('heading', { name: 'Agent Proposals' })).toBeVisible();

    // Verify the department badge
    await expect(page.getByText('✨ The Ambassador').first()).toBeVisible();

    // Verify the context details from the payload
    await expect(page.getByText('Abandoned Carts:')).toBeVisible();
    await expect(page.getByText('3', { exact: true }).first()).toBeVisible();

    await expect(page.getByText('Potential Revenue:')).toBeVisible();
    await expect(page.getByText('$120')).toBeVisible();

    await expect(page.getByText('Draft Message:')).toBeVisible();
    await expect(page.getByText('"Hey there! We noticed you left some items in your cart. Here\'s a 10% discount to complete your order!"')).toBeVisible();

    // Ensure all 3 buttons are present
    const approveButton = page.getByRole('button', { name: 'Approve proposal' });
    const declineButton = page.getByRole('button', { name: 'Decline proposal' });
    const editButton = page.getByRole('button', { name: 'Edit proposal' });

    await expect(approveButton).toBeVisible();
    await expect(declineButton).toBeVisible();
    await expect(editButton).toBeVisible();

    // Click Approve
    await approveButton.click();

    // Wait for optimistic update and verify the card is removed and we see "All caught up!"
    await expect(page.getByText('All caught up!')).toBeVisible();
    await expect(page.getByText('Your agents are currently monitoring the business.')).toBeVisible();
  });
});
