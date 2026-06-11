import { test, expect } from '@playwright/test';

test.describe('Autonomous Work Scheduling - Daily Run', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/daily-run');
  });

  test('should display jobs and allow state transitions', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });

    await expect(page.locator('h1', { hasText: "Today's Run" })).toBeVisible();

    const optimizeBtn = page.locator('button', { hasText: 'Optimize' });
    await expect(optimizeBtn).toBeVisible();
    const optimizeBox = await optimizeBtn.boundingBox();
    expect(optimizeBox?.width).toBeGreaterThanOrEqual(44);
    expect(optimizeBox?.height).toBeGreaterThanOrEqual(44);

    const headingBtn = page.locator('button', { hasText: 'Heading to Job' }).first();
    await expect(headingBtn).toBeVisible();

    const headingBox = await headingBtn.boundingBox();
    expect(headingBox?.height).toBeGreaterThanOrEqual(44);

    await headingBtn.click();

    const startWorkBtn = page.locator('button', { hasText: 'Start Work' }).first();
    await expect(startWorkBtn).toBeVisible();
    await startWorkBtn.click();

    const jobDoneBtn = page.locator('button', { hasText: 'Job Done' }).first();
    await expect(jobDoneBtn).toBeVisible();
    await jobDoneBtn.click();

    const completedBtn = page.locator('button', { hasText: 'Completed' }).first();
    await expect(completedBtn).toBeVisible();
    await expect(completedBtn).toBeDisabled();
  });
});
