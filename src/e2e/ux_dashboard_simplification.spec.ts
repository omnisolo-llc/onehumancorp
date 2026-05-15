import { test, expect } from '@playwright/test';

test.describe('Dashboard Simplification & UX Polish', () => {
  test.beforeEach(async ({ page }) => {
    // Start at the home page (no pre-authenticated shortcuts)
    await page.goto('/');
    await page.waitForTimeout(1000);
  });

  test('Metric is visible and uses plain language label', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    // Verify plain language
    const metricCard = page.getByTestId('metric-sales');
    await expect(metricCard).toBeVisible();
    await expect(metricCard).toContainText("Today's Sales");
    await expect(metricCard).not.toContainText("Revenue TTD");
  });

  test('Navigation has 5 primary actions within 2 taps', async ({ page }) => {
    const nav = page.locator('#main-nav');
    await expect(page.getByTestId('nav-home')).toBeVisible();
    await expect(page.getByTestId('nav-orders')).toBeVisible();
    await expect(page.getByTestId('nav-add')).toBeVisible();
    await expect(page.getByTestId('nav-messages')).toBeVisible();
    await expect(page.getByTestId('nav-share')).toBeVisible();

    const addBtn = page.getByTestId('nav-add');
    const box = await addBtn.boundingBox();
    expect(box?.width).toBeGreaterThanOrEqual(44);
    expect(box?.height).toBeGreaterThanOrEqual(44);
  });

  test('First-Time User Tour is a single step and plain language', async ({ page }) => {
    const hintBtn = page.getByTestId('hint-btn');
    await expect(hintBtn).toBeVisible();
    await hintBtn.click();

    const hintContainer = page.locator('#tour-hint');
    await expect(page.locator('.hint-text')).toContainText('Tap "Add Product" below to start selling');
  });

  test('Grandmother Test - Jargon removed from tasks', async ({ page }) => {
    const storeTasks = page.getByTestId('store-tasks');
    await expect(storeTasks).toContainText('Connect my Instagram');
    await expect(storeTasks).not.toContainText('API Keys');
    await expect(storeTasks).toContainText('Get order notifications');
    await expect(storeTasks).not.toContainText('Webhook');

    const connectBtn = page.getByTestId('btn-connect-ig');
    const box = await connectBtn.boundingBox();
    expect(box?.width).toBeGreaterThanOrEqual(44);
    expect(box?.height).toBeGreaterThanOrEqual(44);
  });

  test('Error states use plain language and no technical jargon', async ({ page }) => {
    await page.waitForTimeout(1000);
    const errorContainer = page.locator('#error-container');
    await expect(errorContainer).toContainText("We couldn't connect to your Instagram");
    await expect(errorContainer).toContainText("check your password and try again");
    await expect(errorContainer).not.toContainText("API error");
    await expect(errorContainer).not.toContainText("500");
    await expect(errorContainer).not.toContainText("null pointer");
  });
});
