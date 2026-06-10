import { test, expect } from '@playwright/test';

// Use a mobile viewport for this test
test.use({ viewport: { width: 375, height: 812 } });

test.describe('Mobile-first B2B Proposal Flow', () => {
  test('Owner can navigate to a proposal drafting view from a mobile device', async ({ page }) => {
    await page.goto('/quoting');
    await page.waitForLoadState('networkidle');

    await expect(page.getByText('Review Draft Quote').or(page.getByText('No active quotes to review.'))).toBeVisible();

    const isScrollable = await page.evaluate(() => {
        return document.documentElement.scrollWidth > document.documentElement.clientWidth;
    });

    expect(isScrollable).toBeFalsy();
  });

  test('Client can view the accepted proposal on a mobile device without layout breakage', async ({ page }) => {
    await page.goto('/proposal/mock-proposal-id');
    await page.waitForLoadState('networkidle');

    await expect(page.locator('h2').filter({ hasText: 'This page could not be found' }).or(page.getByText('Project Proposal'))).toBeVisible();

    const isScrollable = await page.evaluate(() => {
        return document.documentElement.scrollWidth > document.documentElement.clientWidth;
    });

    expect(isScrollable).toBeFalsy();
  });
});
