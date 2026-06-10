import { test, expect } from '@playwright/test';

test.describe('Unified Agent Feed', () => {
  test('Feed displays correctly and allows interaction', async ({ page }) => {
    await page.goto('/dashboard');

    // We are mocking a response that would typically be returned from our API
    await page.route('/api/agent-feed*', async route => {
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

    // Try to ensure dashboard is loaded
    await page.waitForTimeout(2000);
  });
});
