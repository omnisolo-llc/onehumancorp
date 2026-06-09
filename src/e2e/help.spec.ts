import { test, expect } from './fixtures';
test.describe('Help Center', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/dashboard');
  });
  test('should display dashboard with nav', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.getByRole('navigation', { name: 'Primary' })).toBeVisible();
  });
  test('should show dashboard link in nav', async ({ page }) => {
    const dashLink = page.getByRole('link', { name: 'Dashboard', exact: true });
    await expect(dashLink).toBeVisible();
  });
  test('should show agents link in nav', async ({ page }) => {
    const agentsLink = page.getByRole('link', { name: 'Agents' });
    await expect(agentsLink).toBeVisible();
  });
  test('should show setup link in nav', async ({ page }) => {
    const setupLink = page.getByRole('link', { name: 'Setup', exact: true });
    await expect(setupLink).toBeVisible();
  });
  test('should display welcome message', async ({ page }) => {
    await expect(page.locator('text=Welcome back')).toBeVisible();
  });
  test('should display agents working message', async ({ page }) => {
    await expect(page.locator('text=Your agents are working on your behalf')).toBeVisible();
  });
});
test.describe('Agents Page', () => {
  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
  });
});
test.describe('Business Setup Page', () => {
  test('should display setup page', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' })).toBeVisible();
  });
  test('should show setup wizard text', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.locator('text=Your business, live in minutes')).toBeVisible();
  });
});
test.describe('Dashboard', () => {
  test('should have working nav links', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('link', { name: 'Agents', exact: true }).click();
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
  });
});

test.describe('Documentation Pages', () => {
  test('should display Help Center main page', async ({ page }) => {
    await page.goto('/help');
    await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();
  });

  test('should display Changelog page', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.locator('h1', { hasText: 'Release Notes & Changelog' })).toBeVisible();
  });

  test('should display API Docs page', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.locator('text=Advanced:')).toBeVisible();
  });

  test('should display Video Tutorials page', async ({ page }) => {
    await page.goto('/help/videos');
    await expect(page.locator('h1', { hasText: 'Video Guides' })).toBeVisible();
  });
});
