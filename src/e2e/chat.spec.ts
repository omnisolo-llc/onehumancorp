import { test, expect } from './fixtures';

test.describe('Chat Page', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
  });

  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
  });

  test('should display business setup page', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.locator('text=Setup Assistant')).toBeVisible();
  });
});

test.describe('Navigation', () => {
  test('should navigate via nav links', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('navigation', { name: 'Primary' })).toBeVisible();
    await page.getByRole('link', { name: 'AI Departments' }).click();
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
  });

  test('should show welcome message on dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('text=Welcome back')).toBeVisible();
  });
});

test.describe('Native Chatwoot Replacement - Unified Inbox', () => {
  test('should load the native chat view via toggle', async ({ page }) => {
    await page.goto('/inbox');
    const toggleButton = page.locator('button', { hasText: 'Toggle View' });
    await expect(toggleButton).toBeVisible();
  });

  test('should render the Conversations sidebar properly', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.locator('text=Conversations')).toBeVisible();
  });

  test('should render the Active Thread main area', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.locator('text=Active Thread')).toBeVisible();
  });

  test('should complete loading and show empty states if no data is seeded', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.locator('text=Loading...').first()).not.toBeVisible({ timeout: 10000 });
  });

  test('should have a functional message input and send button in the thread', async ({ page }) => {
    await page.goto('/inbox');
    const messageInput = page.getByPlaceholder('Type your message...');
    await expect(messageInput).toBeVisible();
    const sendButton = page.getByRole('button', { name: 'Send' });
    await expect(sendButton).toBeVisible();
  });
});
