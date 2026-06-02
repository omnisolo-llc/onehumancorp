import { test, expect } from '@playwright/test';

test.describe('Courier PWA UI', () => {
  test('should display available jobs and allow claiming and completing them', async ({ page }) => {
    // Navigate to the Courier PWA page
    await page.goto('http://localhost:3000/courier');

    // Verify the page title and header
    await expect(page.locator('h1')).toHaveText('Courier Jobs');
    await expect(page.locator('#available-jobs-header')).toBeVisible();

    // Verify that available jobs are displayed (we seeded Maya's Bakery and Carlos Repairs)
    await expect(page.locator('text=Maya\'s Bakery')).toBeVisible();
    await expect(page.locator('text=Carlos Repairs')).toBeVisible();

    // Claim the first job
    const claimButton = page.locator('[data-testid="claim-job_1"]');
    await expect(claimButton).toBeVisible();
    await claimButton.click();

    // Verify the UI changes to active job view
    await expect(page.locator('text=Active Job')).toBeVisible();
    await expect(page.locator('text=Maya\'s Bakery')).toBeVisible();
    await expect(page.locator('#mark-picked-up')).toBeVisible();

    // Mark as Picked Up
    await page.locator('#mark-picked-up').click();

    // Verify UI changes to dropoff step
    await expect(page.locator('#mark-delivered')).toBeVisible();

    // Complete the delivery
    await page.locator('#mark-delivered').click();

    // Verify the completion message is shown
    await expect(page.locator('text=Job Completed!')).toBeVisible();
  });
});
