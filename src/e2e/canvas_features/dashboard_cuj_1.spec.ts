
import { test, expect } from '@playwright/test';

test.describe('Dashboard Feature CUJ 1', () => {
  test('should display key business metrics clearly for non-technical users', async ({ page }) => {
    await page.goto('/');

    // Login phase
    await page.locator('text=Login').click();
    await page.getByPlaceholder('Email or Username').fill('owner@smallbusiness.com');
    await page.locator('input[type="password"]').fill('securepass123');
    await page.locator('button:has-text("Login")').click();

    // Dashboard metric validation
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.locator('.metric-card.revenue')).toBeVisible();
    await expect(page.locator('.metric-card.orders')).toBeVisible();

    // Drill down
    await page.locator('.metric-card.revenue').click();
    await expect(page.getByRole('heading', { name: 'Revenue Details' })).toBeVisible();
  });

  test('should show real-time agent observability cards', async ({ page }) => {
    await page.goto('/');
    await page.locator('text=Login').click();
    await page.getByPlaceholder('Email').fill('owner@smallbusiness.com');
    await page.locator('input[type="password"]').fill('pass');
    await page.locator('button:has-text("Login")').click();

    await expect(page.locator('.agent-status-panel')).toBeVisible();
    await expect(page.locator('.agent-card')).toHaveCount(3);
    await expect(page.locator('text=✅ Your Support Agent replied to 3 customers')).toBeVisible();
  });

  test('should have premium glassmorphism styling and correct typography', async ({ page }) => {
    await page.goto('/dashboard');
    const panel = page.locator('.dashboard-panel').first();
    await expect(panel).toHaveCSS('backdrop-filter', 'blur(20px) saturate(200%)');
    await expect(page.locator('h1').first()).toHaveCSS('font-family', /Outfit/);
    await expect(page.locator('p').first()).toHaveCSS('font-family', /Inter/);
  });
});
