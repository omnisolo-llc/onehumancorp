import { test, expect } from '@playwright/test';

test.describe('Business Setup Wizard E2E - Course Persona', () => {
  test.use({ viewport: { width: 375, height: 667 } });

  test('Complete setup wizard flow for Course', async ({ page }) => {
    await page.goto('/?signup=true');
    await page.evaluate(() => localStorage.clear());
    await page.goto('/?signup=true');

    await expect(page.locator('#signup-screen')).toBeVisible();
    await page.click('button:has-text("Start Wizard")');

    await expect(page.locator('#step-1')).toBeVisible();
    await page.click('button:has-text("Start My Business")');

    await expect(page.locator('#step-2')).toBeVisible();
    await page.click('div.card-btn:has-text("Other")');

    await expect(page.locator('#step-3')).toBeVisible();
    await page.fill('#biz-name', "Online Course");
    await page.click('button:has-text("Next")');

    await expect(page.locator('#step-4')).toBeVisible();
    await page.click('div.card-btn:has-text("Subscriptions")');
    await page.click('button:has-text("Next")');

    await expect(page.locator('#step-5')).toBeVisible();
    await page.click('div.card-btn:has-text("Online only")');

    await expect(page.locator('#step-6')).toBeVisible();
    await page.fill('input[placeholder="Your Full Name"]', 'Course User');
    await page.fill('input[placeholder="Email Address"]', 'Course@ohc.app');
    await page.fill('input[placeholder="Password"]', 'securepassword123');
    await page.click('button:has-text("Next")');

    await expect(page.locator('#step-7')).toBeVisible();
    await expect(page.locator('#review-name')).toHaveText("Online Course");

    await page.click('button:has-text("Launch My Business")');
    await expect(page.locator('#dashboard-screen')).toBeVisible({ timeout: 15000 });
  });
});
