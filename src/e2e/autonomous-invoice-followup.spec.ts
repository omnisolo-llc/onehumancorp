import { test, expect } from '@playwright/test';
import { e2eConfig, setupTenantAndUser } from './playwright.config';

test.describe('Autonomous Invoice Follow-Up', () => {
  let context: any;

  test.beforeEach(async ({ browser }) => {
    context = await setupTenantAndUser(browser);
  });

  test('Finance agent drafts polite reminder for overdue invoice', async () => {
    const page = await context.newPage();
    await page.setViewportSize({ width: 375, height: 812 });

    // 1. Trigger the simulate endpoint for the invoice followup.
    const response = await page.request.post(`${e2eConfig.baseURL}/api/agents/approvals/simulate-invoice-followup`, {
      headers: {
        'x-test-tenant-id': 'e2e-tenant',
      },
    });

    expect(response.ok()).toBeTruthy();
    const result = await response.json();
    expect(result.success).toBe(true);

    await test.step('Verify Agent Feed displays Invoice Followup drafts', async () => {
      // 2. Navigate to dashboard to see the feed.
      await page.goto(`${e2eConfig.baseURL}/dashboard`);
      await page.waitForLoadState('networkidle');

      // 3. Review UI
      await expect(page.locator('text=Action Required: Approve Invoice Reminder for Acme Corp')).toBeVisible();
      await expect(page.locator('text=$1000.00')).toBeVisible();
      await expect(page.locator('text=3 days')).toBeVisible();

      // 4. Tap "Approve & Send"
      const approveButton = page.getByTestId('feed-approve-btn').filter({ hasText: 'Approve & Send' }).first();
      await expect(approveButton).toBeVisible();
      await approveButton.click();

      // We expect the button to go to processing state or disappear
      await expect(approveButton).not.toBeVisible();
    });
  });
});
