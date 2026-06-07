import { test, expect } from '@playwright/test';

test.describe('Smart Pricing Feature CUJ', () => {
  test('User can approve a smart pricing suggestion from the agent feed', async ({ page }) => {
    // 1. Navigate to dashboard feed (Agent proposals)
    await page.goto('/dashboard');

    // We expect the UnifiedAgentFeed to be populated by the mock/seed data.
    // However, since we might not have a reliable seed here, we use Playwright route mock for API
    await page.route('**/api/agents/approvals?tenant_id=**', async route => {
      const json = {
        pending_approvals: [
          {
            id: 'mock-approval-1',
            tenant_id: 'tenant-123',
            department: 'business_advisory',
            description: 'Smart Price Suggestion: Winter Scarf',
            status: 'PendingApproval',
            action_risk: 'HIGH',
            payload: {
              context: {
                feature_type: 'smart_pricing',
                product_id: 'prod-winter-scarf',
                product_name: 'Winter Scarf',
                days_stagnant: 60,
                suggested_discount_percent: 15,
                margin_safe: true,
                potential_revenue: 120.00
              }
            }
          }
        ]
      };
      await route.fulfill({ json });
    });

    await page.route('**/api/agents/approvals/activity?tenant_id=**', async route => {
      await route.fulfill({ json: { pending_approvals: [] } });
    });

    await page.route('**/api/agents/approvals/mock-approval-1', async route => {
      await route.fulfill({ status: 200, json: { success: true } });
    });

    await page.reload();

    // 2. See Suggestion
    await expect(page.getByText('Smart Price Suggestion: Winter Scarf')).toBeVisible();
    await expect(page.getByText('15% OFF')).toBeVisible();
    await expect(page.getByText('Protected Margin Safe')).toBeVisible();

    // 3. Approve Suggestion
    const approveButton = page.getByRole('button', { name: 'Approve proposal' });
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // 4. Verify Optimistic UI update
    await expect(page.getByText('All caught up!')).toBeVisible();
  });
});
