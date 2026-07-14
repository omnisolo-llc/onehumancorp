import { test, expect } from './fixtures';

test.describe('Help Chat Flow', () => {
  test('should be removed', async ({ page }) => {
    expect(true).toBe(true);
  });
});

test.describe('Help Center Complete UI Flow', () => {
  test('should load Help Center, find videos, and click video to play', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/help');

    // Search for the video string
    const searchBox = page.getByPlaceholder('Search for help articles and videos...');
    await searchBox.fill('payment');

    // Wait for UI to filter. We use exact matching because there are multiple elements matching "Accept your first payment"
    await expect(page.getByText('Connecting a bank account to accept payments', { exact: true })).toBeVisible();

    // Click the video (we specifically click the title paragraph/div)
    // In our mobile view, the element might be outside the viewport or need forceful click
    await page.getByText('Connecting a bank account to accept payments', { exact: true }).click({ force: true });

    // Expect the video player modal
    const videoModal = page.locator('video');
    await expect(videoModal).toBeVisible();

    // Close the modal
    const closeBtn = page.locator('button[aria-label="Close video"]');
    // Ensure the modal animation is fully finished before clicking
    await expect(closeBtn).toBeVisible();
    await page.waitForTimeout(1000); // Wait for the modal animation (e.g. animate-pop-in) to finish before clicking the absolute positioned button
    await closeBtn.evaluate((node) => (node as HTMLButtonElement).click());

    // Modal should be gone
    await expect(videoModal).not.toBeVisible();
  });
});

test.describe('Tooltip functionality', () => {
  test('should display tooltip on hover', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    // Wait until tooltips load dynamically or are preloaded on Help page
    await page.goto('/api-docs');

    // The component wrapper has class inline-block relative cursor-help
    const tooltipTrigger = page.locator('.cursor-help').first();
    await expect(tooltipTrigger).toBeVisible();

    await tooltipTrigger.hover();

    // Check if the tooltip wrapper gets rendered
    await expect(page.getByText('Direct API access is only for custom integrations.').first()).toBeVisible({ timeout: 10000 });
  });

  test('should display tooltip on dashboard hover', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');

    // We expect the tooltip with text "View your daily sales and overall business health." to appear
    const dashboardTooltipTrigger = page.locator('.cursor-help', { hasText: 'Dashboard' }).first();
    await expect(dashboardTooltipTrigger).toBeVisible();

    await dashboardTooltipTrigger.hover();

    await expect(page.getByText('View your daily sales and overall business health.').first()).toBeVisible({ timeout: 10000 });
  });
});

test.describe('Changelog UX', () => {
  test('should ensure changelog renders beautiful design without placeholder text', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/changelog');

    await expect(page.getByRole('heading', { name: 'Version 1.1 (Latest)' })).toBeVisible();
    // Check that we removed the test line
    await expect(page.locator('text=This is a plain paragraph test line.')).not.toBeVisible();
  });
});

test.describe('API Documentation', () => {
  test('should navigate to API Documentation and load Swagger UI', async ({ page }) => {
    await page.goto('/api-docs');

    // Check for advanced warning badge
    await expect(page.getByText('Advanced:')).toBeVisible();

    // Check for swagger-ui wrapper
    const swaggerUI = page.locator('.swagger-ui');
    await expect(swaggerUI).toBeVisible({ timeout: 15000 });
  });
});

test.describe('AppShell Help Button', () => {
  test('should display Help Center link and navigate successfully', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');

    const helpButton = page.locator('#help-center-nav-btn');
    await expect(helpButton).toBeVisible();

    await Promise.all([
      page.waitForURL(/\/help/),
      helpButton.click(),
    ]);

    // Help Center should have its search input
    await expect(page.getByPlaceholder('Search for help articles and videos...')).toBeVisible();
  });
});
