import { test, expect } from './fixtures';

test.describe('Dashboard Basic UI', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/dashboard');
  });

  test('should display dashboard with nav', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Dashboard', exact: true })).toBeVisible();
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should show Calendar link in nav', async ({ page }) => {
    const calendarLink = page.locator('nav a').filter({ hasText: 'Calendar' });
    await expect(calendarLink).toBeVisible();
  });

  test('should show Inbox link in nav', async ({ page }) => {
    const inboxLink = page.locator('nav a:has-text("Inbox")');
    await expect(inboxLink).toBeVisible();
  });

  test('should have working Agents nav link', async ({ page }) => {
    await page.locator('nav a').filter({ hasText: 'AI Departments' }).click();
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
  });
});

test.describe('Onboarding Page', () => {
  test('should display onboarding wizard', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.locator('h2').filter({ hasText: 'Tell us about your business' })).toBeVisible();
  });
});

test.describe('Agents Page', () => {
  test('should display AI Departments heading', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
  });
});