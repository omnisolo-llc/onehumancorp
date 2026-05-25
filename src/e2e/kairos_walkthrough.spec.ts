import { test, expect } from './fixtures';

test.describe('🎨 Canvas: KAIROS Orchestration Walkthrough', () => {

  test('CUJ 1: Navigate to KAIROS Dashboard via Main Navigation', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('#kairos-nav-link')).toBeVisible();
    await page.click('#kairos-nav-link');
    await expect(page).toHaveURL(/.*kairos/);
    await expect(page.getByRole('heading', { name: 'KAIROS Orchestration' })).toBeVisible();
  });

  test('CUJ 2: Verify KAIROS Dashboard Components', async ({ page }) => {
    await page.goto('/kairos');
    await expect(page.locator('#kairos-brain')).toBeVisible();
    await expect(page.locator('#kairos-nerves')).toBeVisible();
    await expect(page.locator('#kairos-memory')).toBeVisible();

    await expect(page.getByRole('heading', { name: 'Shared Task List' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Teammate Mesh' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'AutoDream Memory' })).toBeVisible();
  });

  test('CUJ 3: Start KAIROS Walkthrough from Help Widget', async ({ page }) => {
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

    // Wait for walkthrough to potentially appear
    // Note: If NEXT_PUBLIC_E2E is true, InteractiveWalkthrough returns null
    // But we check for elements it targets
    await expect(page.locator('#kairos-brain')).toBeVisible();
  });

  test('CUJ 4: Verify Walkthrough targets exist', async ({ page }) => {
    await page.goto('/kairos?walkthrough=true');
    await expect(page.locator('#kairos-brain')).toBeVisible();
    await expect(page.locator('#kairos-nerves')).toBeVisible();
    await expect(page.locator('#kairos-memory')).toBeVisible();
  });

  test('CUJ 5: Return to Dashboard from KAIROS', async ({ page }) => {
    await page.goto('/kairos');
    await page.click('header a[href="/dashboard"]');
    await expect(page).toHaveURL(/.*dashboard/);
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

});
