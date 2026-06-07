import { test, expect } from './fixtures';

test.describe('Login Screen Visual Audit', () => {
  test('should display login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('input[type="password"]').filter({ visible: true }).first()).toBeVisible();
  });

  test('should verify "Email or Username" input field exists', async ({ page }) => {
    await page.goto('/login');
    const input = page.getByPlaceholder('Email or Username');
    await expect(input).toBeVisible();
    await expect(input).toHaveAttribute('type', 'text');
  });

  test('should verify "Login" heading exists', async ({ page }) => {
    await page.goto('/login');
    const heading = page.getByRole('heading', { name: 'Login' });
    await expect(heading).toBeVisible();
    await expect(heading).toHaveText('Login');
  });

  test('should verify "Start Business Setup" button exists', async ({ page }) => {
    await page.goto('/login');
    const button = page.getByRole('button', { name: 'Start Business Setup' });
    await expect(button).toBeVisible();
    await expect(button).toHaveText('Start Business Setup');
  });

  test('should verify "Log In" button exists', async ({ page }) => {
    await page.goto('/login');
    const button = page.getByRole('button', { name: 'Log In' });
    await expect(button).toBeVisible();
    await expect(button).toHaveText('Log In');
  });

  test('should verify "or" text divider exists', async ({ page }) => {
    await page.goto('/login');
    const divider = page.getByText('or', { exact: true });
    await expect(divider).toBeVisible();
    await expect(divider).toHaveClass(/text-gray-400|dark:text-gray-500/);
  });

  test('should navigate to dashboard', async ({ page }) => {
    await page.goto('/login');
    await page.getByRole('button', { name: 'Log In' }).click();
    await page.waitForURL('**/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });

  test('should navigate to onboarding', async ({ page }) => {
    await page.goto('/login');
    await page.getByRole('button', { name: 'Start Business Setup' }).click();
    await page.waitForURL('**/onboarding');
    await expect(page.getByText('Start Onboarding')).toBeVisible({ timeout: 15000 });
  });

  test('should display dashboard directly', async ({ page }) => {
    await page.goto('/dashboard');
    await page.waitForURL('**/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });

  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible({ timeout: 15000 });
  });
});

test.describe('Navigation', () => {
  test('should navigate via nav links', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('navigation', { name: 'Primary' })).toBeVisible({ timeout: 15000 });
    await page.getByRole('link', { name: 'Agents' }).click();
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible({ timeout: 15000 });
  });

  test('should show welcome message', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('text=Welcome back')).toBeVisible({ timeout: 15000 });
  });
});
