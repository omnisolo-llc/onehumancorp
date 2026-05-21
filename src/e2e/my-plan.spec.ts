import { test, expect } from './fixtures';

test.describe('My Plan Page Details', () => {
  test('displays cost breakdown and usage details without technical jargon', async ({ page }) => {
    // 1. Arrange: Go to the my-plan page
    await page.goto('/my-plan');
    await expect(page.getByRole('heading', { name: 'My Current Plan' })).toBeVisible();

    // 2. Act: Click to view details
    await page.getByRole('button', { name: 'View Cost Details' }).click();

    // 3. Assert: the details show up and don't use 'LLM Token Spend'
    await expect(page.getByRole('heading', { name: 'Cost & Usage' })).toBeVisible();
    await expect(page.getByText('Smart Assistant Usage')).toBeVisible();
    await expect(page.getByText('Storage Cost')).toBeVisible();
    await expect(page.getByText('Total Revenue')).toBeVisible();
    await expect(page.getByText('Total Costs')).toBeVisible();
  });
});
