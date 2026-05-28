import { test, expect } from './fixtures';

test.describe('Landing Screen Visual Audit', () => {
  test('should display hybrid landing page with download buttons', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'The Hybrid Agentic OS' })).toBeVisible();
    await expect(page.getByRole('button', { name: /Download for Mac/i })).toBeVisible();
    await expect(page.getByRole('button', { name: /Download for Windows/i })).toBeVisible();
    await expect(page.getByRole('button', { name: /Download for Linux/i })).toBeVisible();
  });

  test('should display navigation on dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
  });
});