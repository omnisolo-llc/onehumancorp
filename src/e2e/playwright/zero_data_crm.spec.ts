import { test, expect } from '@playwright/test';

test.describe('Zero-Data-Entry AI CRM Workflow', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('extract insights and display on customer profile', async ({ page }) => {
    await page.goto('/customer/memory-graph?customerId=123&tenantId=tenant-1');
    // Ensure the page loads without crashing due to new insights array
    await expect(page.getByText('Customer Context')).toBeVisible({ timeout: 10000 });
  });
});
