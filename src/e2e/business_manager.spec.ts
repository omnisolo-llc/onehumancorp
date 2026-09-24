import { test, expect } from './fixtures';

test.describe('Business Manager UI', () => {
  test('should display dashboard with nav', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
    await expect(page.getByRole('navigation', { name: 'Primary' }).first()).toBeVisible();
  });

  test('should navigate to agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();
  });

  test('should display login page', async ({ anonymousPage }) => {
    await anonymousPage.goto('/login');
    await expect(anonymousPage.getByRole('heading', { name: /Sign in|Login/i }).first()).toBeVisible();
    await expect(anonymousPage.getByLabel(/Email or username/i)).toBeVisible();
    await expect(anonymousPage.getByLabel(/Password/i)).toBeVisible();
    await expect(anonymousPage.getByRole('button', { name: /Log in/i })).toBeVisible();
  });

  test('should display business setup page', async ({ page }) => {
    await page.goto('/ui/setup.html');
    await page.waitForLoadState('domcontentloaded');
    await expect(page.getByRole('heading', { name: /Setup|Tell us about your business/i }).first()).toBeVisible();
    await expect(page.locator('#setup-screen')).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should have working nav links', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
    const link = page.getByRole('link', { name: 'AI Departments' }).first();
    await expect(link).toBeVisible();
    await link.click();
    await page.waitForURL('**/agents**');
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();
  });

  test('should navigate to dashboard from nav', async ({ page }) => {
    await page.goto('/agents');
    await page.locator('nav a[href="/dashboard"], a[href="/dashboard"]').first().click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
  });
});
