import { test, expect } from './fixtures';
test.describe.configure({ mode: 'serial' });

test.describe('Business Advisory & Cost Dashboard', () => {
  test('displays advisory summary and cost transparency', async ({ page }) => {
    // We navigate to /cost-dashboard which acts as the transparent dashboard screen
    await page.goto('/cost-dashboard');

    // Check if the dashboard title loads
    await expect(page.getByRole('heading', { name: 'Business Advisory Dashboard' })).toBeVisible();

    // Check for the summary text
    await expect(page.getByText('Advisory Summary')).toBeVisible();
    await expect(page.getByText('Recommendation: Consider running a seasonal promotion')).toBeVisible();

    // Check Cost Transparency section
    await expect(page.getByRole('heading', { name: 'Cost Transparency' })).toBeVisible();
    await expect(page.getByText('Total Costs')).toBeVisible();

    // Check Breakdown Section
    await expect(page.getByRole('heading', { name: 'Cost Breakdown' })).toBeVisible();
    await expect(page.getByText('LLM Usage')).toBeVisible();
    await expect(page.getByText('Storage')).toBeVisible();
    await expect(page.getByText('Payment Fees')).toBeVisible();
  });

  test('navigates back to My Plan', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await page.getByRole('button', { name: 'Back to My Plan' }).click();
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();
  });
});
