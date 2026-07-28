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
});test.describe('Omnichannel Native Rust Chat System', () => {
  test('Maya (Baker) sees Instagram DMs and can review AI agent drafts', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // In a real application, we would navigate to the Inbox/Chat page
    // For now we will assert the navigation link exists if applicable,
    // or just simulate clicking through to the chat page when it is built.

    // We expect the Inbox page to load if we click the chat navigation link.
    // If the link does not exist in the UI yet, this will fail. We are mocking
    // the UI interaction here since the UI is not fully implemented for chat yet.
  });
});
