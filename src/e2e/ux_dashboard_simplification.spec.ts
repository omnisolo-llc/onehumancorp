import { test, expect } from './fixtures';

test.describe('Dashboard UX Simplification (Grandmother Test)', () => {
  test('should display dashboard with nav and check glassmorphism classes', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.locator('nav')).toBeVisible();

    // Verify business snapshot panel exists with the correct glassmorphism class
    const snapshotPanel = page.locator('text=Business Snapshot').locator('..').locator('.ohc-hybrid-panel').first();
    await expect(snapshotPanel).toBeVisible();
  });

  test('should verify AI Insights panel uses glassmorphism and is clearly labeled', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    const insightsPanel = page.locator('text=Unlock Advanced Store Analytics').locator('..').locator('..');
    await expect(insightsPanel).toHaveClass(/ohc-hybrid-panel/);
    await expect(page.locator('text=Pro Feature')).toBeVisible();
  });

  test('should display Growth & Promotions panel with premium styling', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    const growthPanel = page.locator('text=Boost Sales with AI Campaigns').locator('..').locator('..');
    await expect(growthPanel).toHaveClass(/ohc-hybrid-panel/);
  });

  test('should have rounded buttons matching Apple/UniFi specs', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    const viewInsightsBtn = page.getByRole('button', { name: /View AI Insights/i });
    // Check if the tailwind rounded class is present
    await expect(viewInsightsBtn).toHaveClass(/rounded-\[8px\]/);
  });

  test('should render Team Activity panel correctly with glassmorphism', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    const activityPanel = page.locator('text=Waiting for team activity...').locator('..').locator('..');
    await expect(activityPanel).toHaveClass(/ohc-hybrid-panel/);
  });

  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
  });

  test('should display business setup page', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('text=Your business, live in minutes')).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should navigate via nav links', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav')).toBeVisible();
    await page.locator('nav a:has-text("AI Departments")').click();
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
  });

  test('should show welcome message on dashboard', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Welcome back')).toBeVisible();
  });
});