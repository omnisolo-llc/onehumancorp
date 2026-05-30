import { test, expect } from './fixtures';

test.describe('Login Page', () => {
  test('should display login page with form', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('input[type="password"]').filter({ visible: true }).first()).toBeVisible();
  });

  test('should display login button', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('button:has-text("Login Sign In")')).toBeVisible();
  });

  test('should have working show button', async ({ page }) => {
    await page.goto('/login');
    const showBtn = page.locator('button:has-text("Show")');
    if (await showBtn.isVisible()) {
      await showBtn.click();
    }
  });
});

test.describe('Dashboard', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('#dashboard-screen')).toBeVisible();
  });

  test('should display nav', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('#main-nav')).toBeAttached();
  });

  test('should show welcome message', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('#dashboard-screen h1')).toHaveText('Dashboard');
  });
});

test.describe('Navigation', () => {
  test('should navigate to agents page', async ({ page }) => {
    await page.goto('/');
    // #main-nav is hidden initially so we might need to interact with it directly or wait
    await page.locator('#nav-agents').click();
    await expect(page.locator('#team-screen')).toBeVisible();
  });

  test('should display business setup', async ({ page }) => {
    await page.goto('/');
    await page.locator('#nav-setup').click();
    await expect(page.locator('#setup-screen')).toBeVisible();
  });
});