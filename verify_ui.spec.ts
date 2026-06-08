import { test, expect } from '@playwright/test';

test('verify plan page - desktop', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 1080 });
  await page.goto('http://localhost:3000/plan');
  await page.waitForSelector('text=My Plan');
  await page.screenshot({ path: 'plan-desktop.png', fullPage: true });
});

test('verify plan page - mobile', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 812 });
  await page.goto('http://localhost:3000/plan');
  await page.waitForSelector('text=My Plan');
  await page.screenshot({ path: 'plan-mobile.png', fullPage: true });
});

test('verify cost dashboard - desktop', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 1080 });
  await page.goto('http://localhost:3000/cost-dashboard');
  await page.waitForSelector('text=Business Advisory Dashboard');
  await page.screenshot({ path: 'cost-dashboard-desktop.png', fullPage: true });
});

test('verify cost dashboard - mobile', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 812 });
  await page.goto('http://localhost:3000/cost-dashboard');
  await page.waitForSelector('text=Business Advisory Dashboard');
  await page.screenshot({ path: 'cost-dashboard-mobile.png', fullPage: true });
});
