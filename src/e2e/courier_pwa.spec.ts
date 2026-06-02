import { test, expect } from '@playwright/test';

// Persona: Courier (Local Mesh)
// Business Concept: A local teenager running deliveries via the OHC Fractional Delivery Mesh.
// Operation CUJ: The courier wants to see available jobs nearby, claim one, and mark it delivered to get paid.
test.describe('Autonomous Fractional Local Delivery Network (AFLDN) - Courier PWA', () => {
  test('Courier can view available jobs, claim a job, and mark it delivered', async ({ page }) => {
    // 1. Visit the Courier PWA list view
    await page.goto('/courier');

    // Expect the header
    await expect(page.locator('h1')).toHaveText('Available Jobs');

    // Expect at least one job to be listed (mock data creates one)
    // Wait for the mock loading delay
    await page.waitForTimeout(600);

    const jobItem = page.locator('text=$7.50');
    await expect(jobItem).toBeVisible();

    // 2. Click on the available job to view details
    await page.click('text=Lat 40.7128');

    // 3. Details View
    await expect(page.locator('h1')).toHaveText('Job Details');
    await expect(page.locator('text=$7.50')).toBeVisible();
    await expect(page.locator('text=Pickup')).toBeVisible();
    await expect(page.locator('text=Dropoff')).toBeVisible();

    // 4. Claim the job
    const claimButton = page.locator('button', { hasText: 'Claim Job' });
    await expect(claimButton).toBeVisible();
    await claimButton.click();

    // Wait for mock processing
    await page.waitForTimeout(600);

    // The button should now say 'Mark Delivered'
    const deliverButton = page.locator('button', { hasText: 'Mark Delivered' });
    await expect(deliverButton).toBeVisible();

    // 5. Mark the job as delivered
    await deliverButton.click();

    // Wait for mock processing
    await page.waitForTimeout(600);

    // Expect success state
    await expect(page.locator('text=Delivered!')).toBeVisible();
    await expect(page.locator('text=Payout is being processed.')).toBeVisible();
  });
});
