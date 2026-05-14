import { test, expect } from '@playwright/test';

test.describe('Scribe Documentation System', () => {
  test.beforeEach(async ({ page }) => {
    // Log in first
    await page.goto('/login');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.click('button:has-text("Sign In")');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
  });

  test('should show help FAB and open help modal', async ({ page }) => {
    const fab = page.locator('#help-fab');
    await expect(fab).toBeVisible();
    await fab.click();

    await expect(page.locator('#help-modal')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();
  });

  test('should search for help articles via backend', async ({ page }) => {
    await page.click('#help-fab');
    const searchInput = page.locator('#help-search');
    await searchInput.fill('payments');

    // Should show backend results
    await expect(page.locator('#help-content')).toContainText('Accept Payments with Stripe');
  });

  test('should show tooltips on navigation hover', async ({ page }) => {
    const dashNav = page.locator('#dashboard-nav');
    await dashNav.hover();

    const tooltip = page.locator('#active-tooltip');
    await expect(tooltip).toBeVisible();
    await expect(tooltip).toContainText('business overview');
  });

  test('should start and navigate a walkthrough from backend', async ({ page }) => {
    // Open menu to see walkthrough button
    await page.click('button:has-text("Menu")');
    await page.click('button:has-text("Start Walkthrough")');

    await expect(page.locator('.walkthrough-bubble')).toBeVisible();
    await expect(page.locator('.walkthrough-bubble')).toContainText('Go to Billing');

    await page.click('button:has-text("Next")');
    await expect(page.locator('.walkthrough-bubble')).toContainText('Pick a Plan');

    await page.click('button:has-text("Stop")');
    await expect(page.locator('.walkthrough-bubble')).not.toBeVisible();
  });

  test('should toggle advanced mode and show API docs', async ({ page }) => {
    await page.click('#api-nav');
    await expect(page.locator('.advanced-only')).not.toBeVisible();

    await page.click('button:has-text("Toggle Advanced Mode")');
    await expect(page.locator('.advanced-only')).toBeVisible();
    await expect(page.locator('.advanced-only')).toContainText('API Reference');
  });
});
