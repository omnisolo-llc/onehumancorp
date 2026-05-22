import { test, expect } from '@playwright/test';

test('verify onboarding design', async ({ page }) => {
  await page.goto('http://localhost:3000/onboarding');
  await page.waitForTimeout(1000);
  await page.screenshot({ path: '/home/jules/verification/onboarding_screenshot.png' });
});

test('verify website builder design', async ({ page }) => {
  await page.goto('http://localhost:3000/website-builder');
  await page.waitForTimeout(1000);
  await page.screenshot({ path: '/home/jules/verification/builder_screenshot.png' });
});