import { test, expect } from '@playwright/test';

test('Dashboard screen uses plain-language labels', async ({ page }) => {
  await page.goto('/');
  await page.mouse.click(400, 300);
  await page.keyboard.press('Tab');
  await page.keyboard.press('Tab');
  await page.keyboard.type('admin');
  await page.keyboard.press('Tab');
  await page.keyboard.type('admin');
  await page.keyboard.press('Tab');
  await page.keyboard.press('Enter');
  await page.goto('/#/dashboard');

  await expect(page.locator('text=System Health').first()).toBeVisible();
  await expect(page.locator('text=System Health Status').first()).toBeVisible();
  await expect(page.locator('text=Recent Activity').first()).toBeVisible();
  await expect(page.locator('text=Speed (Avg)').first()).toBeVisible();
  await expect(page.locator('text=AI Team Health').first()).toBeVisible();
  await expect(page.locator('text=Connection Status').first()).toBeVisible();
  await expect(page.locator('text=Cloud connection: fast').first()).toBeVisible();
  await expect(page.locator('text=AI Team Live Activity').first()).toBeVisible();
  await expect(page.locator('text=Live Agent Activity').first()).toBeVisible();
  await expect(page.locator('text=Response Time').first()).toBeVisible();
});
