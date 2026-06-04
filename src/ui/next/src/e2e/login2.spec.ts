import { test, expect } from '@playwright/test';

test.describe('Login Page Glassmorphism Updates', () => {
  test('should display the login card with blur effect', async ({ page }) => {
    await page.goto('/login');
    const loginCard = page.locator('div.max-w-sm');
    await expect(loginCard).toBeVisible();

    // Verify CSS styles for glassmorphism
    const backdropFilter = await loginCard.evaluate((el) => {
      return window.getComputedStyle(el).backdropFilter || window.getComputedStyle(el).webkitBackdropFilter;
    });
    expect(backdropFilter).toContain('blur');
  });

  test('should have transparent inputs', async ({ page }) => {
    await page.goto('/login');
    const emailInput = page.getByPlaceholder('Email or Username');
    await expect(emailInput).toBeVisible();
    await expect(emailInput).toHaveClass(/bg-white\/50/);
  });

  test('should have a centered title', async ({ page }) => {
    await page.goto('/login');
    const title = page.getByRole('heading', { name: 'Login' });
    await expect(title).toBeVisible();
    await expect(title).toHaveClass(/text-center/);
  });

  test('should redirect to dashboard on click', async ({ page }) => {
    await page.goto('/login');
    const loginBtn = page.getByRole('button', { name: 'Login' });
    await loginBtn.click();
    await page.waitForURL('**/dashboard**');
    expect(page.url()).toContain('/dashboard');
  });

  test('should be mobile responsive', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/login');
    const loginCard = page.locator('div.max-w-sm');
    const box = await loginCard.boundingBox();
    expect(box?.width).toBeLessThanOrEqual(375);
  });
});
