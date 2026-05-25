import { test, expect } from '@playwright/test';

test.describe('Scribe Mission Track', () => {
  test('should display the Scribe Mission Track dashboard and filter correctly', async ({ page }) => {
    await page.goto('/scribe-mission-track');

    // Check header
    await expect(page.locator('h1')).toContainText('Mission Control');
    await expect(page.locator('text=Scribe Track')).toBeVisible();

    // Check all missions are visible initially
    await expect(page.locator('text=Convention Audit & Identity')).toBeVisible();
    await expect(page.locator('text=README Overhaul')).toBeVisible();
    await expect(page.locator('text=Public Interface Documentation')).toBeVisible();

    // Filter by Active
    await page.click('button:has-text("active")');
    await expect(page.locator('text=Convention Audit & Identity')).toBeVisible();
    await expect(page.locator('text=README Overhaul')).not.toBeVisible();
    await expect(page.locator('text=Public Interface Documentation')).not.toBeVisible();

    // Filter by Completed
    await page.click('button:has-text("completed")');
    await expect(page.locator('text=Convention Audit & Identity')).not.toBeVisible();
    await expect(page.locator('text=README Overhaul')).not.toBeVisible();
    await expect(page.locator('text=Public Interface Documentation')).not.toBeVisible();
  });
});
