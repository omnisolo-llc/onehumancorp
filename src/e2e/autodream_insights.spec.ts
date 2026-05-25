import { test, expect } from './fixtures';

test.describe('🎨 Canvas: AutoDream Insights Walkthrough', () => {

  test('CUJ 1: Navigate to KAIROS Dashboard via Main Navigation', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('#kairos-nav-link')).toBeVisible();
    await page.click('#kairos-nav-link');
    await expect(page).toHaveURL(/.*kairos/);
    await expect(page.getByRole('heading', { name: 'KAIROS Orchestration' })).toBeVisible();
  });

  test('CUJ 2: Verify AutoDream Memory Panel Exists', async ({ page }) => {
    await page.goto('/kairos');
    await expect(page.locator('#kairos-memory')).toBeVisible();
    await expect(page.getByText('AutoDream Memory')).toBeVisible();
  });

  test('CUJ 3: Verify Jargon-Free Customer Insights', async ({ page }) => {
    await page.goto('/kairos');
    await expect(page.locator('#kairos-memory')).toBeVisible();
    await expect(page.getByText('Customer Insights')).toBeVisible();
    await expect(page.getByText('842 Items')).toBeVisible();
  });

  test('CUJ 4: Verify Jargon-Free Business Patterns', async ({ page }) => {
    await page.goto('/kairos');
    await expect(page.locator('#kairos-memory')).toBeVisible();
    await expect(page.getByText('Business Patterns Recognized')).toBeVisible();
    await expect(page.getByText('12 Patterns')).toBeVisible();
  });

  test('CUJ 5: Start KAIROS Walkthrough and Verify Memory Target', async ({ page }) => {
    await page.goto('/dashboard');

    // Open Help Widget
    await page.click('button[aria-label="Help"]');
    await expect(page.locator('#help-widget-container')).toBeVisible();

    // Find and click the KAIROS tour button
    const kairosTourBtn = page.locator('#kairos-walkthrough-btn');
    await expect(kairosTourBtn).toBeVisible();
    await kairosTourBtn.click();

    // Verify navigation and query param
    await expect(page).toHaveURL(/.*kairos\?walkthrough=true/);

    // Wait for walkthrough elements to load and AutoDream memory to be visible
    await expect(page.locator('#kairos-memory')).toBeVisible();
  });

});
