import { test, expect } from '@playwright/test';

test.describe('Dashboard UX Simplification (Grandmother Test)', () => {

  test('should navigate to login page successfully', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    // We assume default route acts like dashboard but we'll click login route if nav is hidden or go directly
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login', exact: true })).toBeVisible();
  });

  test('should display simplified dashboard with "Today\'s Sales" prominent metric', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Check for plain language elements
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.locator('text=Today\'s Sales')).toBeVisible();
    await expect(page.locator('text=Welcome back')).toBeVisible();

    // Assure tap targets and classes are present
    const addProductBtn = page.getByRole('button', { name: 'Add Product' });
    await expect(addProductBtn).toBeVisible();
  });

  test('should navigate to Inbox via "Check Messages" button', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    const messagesBtn = page.getByRole('button', { name: 'Check Messages' });
    await expect(messagesBtn).toBeVisible();
    await messagesBtn.click();

    await expect(page.getByRole('heading', { name: 'Customer Inbox' })).toBeVisible();
  });

  test('should navigate to Setup Wizard via "Business Setup" button', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    const setupBtn = page.getByRole('button', { name: 'Business Setup' });
    await expect(setupBtn).toBeVisible();
    await setupBtn.click();

    // Verify we reached setup page
    await expect(page.locator('text=Your business, live in minutes')).toBeVisible();
  });

  test('should navigate to Referrals via "Share Store" quick link', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    const shareBtn = page.getByRole('button', { name: 'Share Store' });
    await expect(shareBtn).toBeVisible();
    await shareBtn.click();

    await expect(page.getByRole('heading', { name: 'Referral Dashboard' })).toBeVisible();
  });

});