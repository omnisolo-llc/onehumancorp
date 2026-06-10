import { test, expect } from '@playwright/test';

test.describe('Unified Agent Feed', () => {
  test('Feed displays correctly and allows interaction', async ({ page }) => {
    // We are mocking a response that would typically be returned from our API
    await page.route('**/api/agent-feed*', async route => {
      const json = {
        items: [
          {
            id: 'test-item-1',
            tenant_id: 'default',
            event_source: 'sales',
            lifecycle_state: 'PENDING_APPROVAL',
            proposed_action: {
              feature_type: 'quote_draft',
              message: 'Test draft quote'
            }
          }
        ]
      };
      await route.fulfill({ json });
    });

    await page.route('**/api/agent-feed/test-item-1/state*', async route => {
        const json = {
          id: 'test-item-1',
          tenant_id: 'default',
          event_source: 'sales',
          lifecycle_state: 'APPROVED',
          proposed_action: {
            feature_type: 'quote_draft',
            message: 'Test draft quote'
          }
        };
        await route.fulfill({ json });
    });

    await page.goto('/dashboard');

    // Wait for the unified agent feed to load
    await expect(page.locator('button', { hasText: /Proposals/ }).first()).toBeVisible({ timeout: 15000 });

    await expect(page.locator('h3', { hasText: 'Test draft quote' })).toBeVisible({ timeout: 5000 });

    // Click Approve
    const approveBtn = page.getByTestId('approve-send-proposal').first();
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Verify it was removed optimistically
    await expect(page.locator('h3', { hasText: 'Test draft quote' })).toBeHidden();
  });
});
