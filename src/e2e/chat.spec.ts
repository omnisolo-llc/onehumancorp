import { test, expect } from './fixtures';

test.describe('Chat Page', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
  });

  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
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
    await page.locator('nav a:has-text("Agents")').click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should show welcome message on dashboard', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Welcome back')).toBeVisible();
  });
});

test.describe('AI Help Chat', () => {
  test('should open chat and interact with agent', async ({ page }) => {
    await page.goto('/dashboard');

    // Open the chat widget
    const chatButton = page.locator('button:has-text("Ask anything")');
    await expect(chatButton).toBeVisible();
    await chatButton.click();

    // Verify chat UI opened
    const chatHeader = page.locator('h3:has-text("Help Agent")');
    await expect(chatHeader).toBeVisible();

    // Type a message
    const input = page.locator('input[placeholder="Ask me anything..."]');
    await input.fill('How do I setup my store?');

    // Submit message
    await page.locator('button[type="submit"]').click();

    // Verify user message appears
    await expect(page.locator('div.bg-blue-600', { hasText: 'How do I setup my store?' })).toBeVisible();

    // Verify AI response from real backend (or mock handling)
    // The real backend will respond based on HelpRegistry
    await expect(page.locator('div.bg-white.text-gray-800', { hasText: 'Based on our help center:' })).toBeVisible();
  });
});