import { test, expect } from './fixtures';

test.describe('Billing Limits & Cost Monitoring', () => {
  test('navigates to cost transparency dashboard from plan page', async ({ page }) => {
    // Navigate to Plan page
    await page.goto('/plan');
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();

    // Click "View Cost Details"
    await page.getByText('View Cost Details', { exact: true }).click();

    // Verify Cost Transparency dashboard is visible
    await expect(page.getByRole('heading', { name: 'Cost Transparency' })).toBeVisible();
    await expect(page.locator('span.font-medium').filter({ hasText: 'Token Usage' })).toBeVisible();
    await expect(page.locator('span.font-medium').filter({ hasText: 'Storage' })).toBeVisible();
    await expect(page.getByText('Current Storage:')).toBeVisible();
  });
});
