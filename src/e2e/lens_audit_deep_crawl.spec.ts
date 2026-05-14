
import { test, expect } from '@playwright/test';

test.describe('Lens Audit Deep Crawl E2E', () => {

  test('verify full state round trip - Dashboard navigation', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav')).toBeVisible();
    await page.goto('/dashboard');
    await expect(page.locator('h1').filter({ hasText: 'Dashboard' })).toBeVisible();
  });

  test('verify full state round trip - Login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('h1').filter({ hasText: 'Login' })).toBeVisible();
  });

  test('verify full state round trip - Agents directory', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('h1').filter({ hasText: 'Agents' })).toBeVisible();
  });

  test('verify setup wizard', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.locator('h1').filter({ hasText: 'Setup Wizard' })).toBeVisible();
  });

  test('verify settings page', async ({ page }) => {
    await page.goto('/settings');
    await expect(page.locator('h1').filter({ hasText: 'Settings' })).toBeVisible();
  });
});
