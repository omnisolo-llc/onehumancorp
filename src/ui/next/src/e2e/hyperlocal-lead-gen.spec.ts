import { test, expect } from '../../../../e2e/fixtures';

test.describe('Hyperlocal Lead Gen CUJ', () => {
  test('Carlos the Handyman sets up a weekly lead gen campaign', async ({ page }) => {
    // Navigate to the dedicated campaign workspace.
    await page.goto('/marketing/lead-gen');

    // Wait for the new page to load
    await expect(page.getByRole('heading', { name: 'Local Lead Generator' })).toBeVisible();

    // 3. Fill in the budget and zip code.
    const budgetInput = page.getByLabel('Weekly Budget ($)');
    const zipInput = page.getByLabel('Target Zip Code');

    await expect(budgetInput).toBeVisible();
    await expect(zipInput).toBeVisible();

    await budgetInput.fill('50');
    await zipInput.fill('90210');

    // 4. Submit the campaign.
    const submitBtn = page.getByRole('button', { name: 'Start Finding Jobs' });
    await submitBtn.click();

    // 5. Verify the UI updates to show the active campaign.
    await expect(page.getByRole('heading', { name: /Campaign Started!/ })).toBeVisible({ timeout: 10000 });
    await expect(page.getByText(/within 10 miles of 90210/)).toBeVisible();
  });
});
