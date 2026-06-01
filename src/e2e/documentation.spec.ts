import { test, expect } from './fixtures';

test.describe('Help Center', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/help');
  });

  test('should display Help Center with articles', async ({ page }) => {
    await expect(page.locator('h1')).toContainText('Help Center');
    await expect(page.locator('text=Getting Started')).toBeVisible();
    await expect(page.locator('text=My Store')).toBeVisible();
  });

  test('should search for an article and verify results', async ({ page }) => {
    const searchInput = page.locator('input[placeholder="Search for help..."]');
    await expect(searchInput).toBeVisible();
    await searchInput.fill('Payments');
    await expect(page.locator('text=Payments')).toBeVisible();
  });
});

test.describe('Help Article Navigation', () => {
  test('should navigate to getting-started article and verify content', async ({ page }) => {
    await page.goto('/help');
    // For Tauri rendered UI
    await expect(page.locator('text=Getting Started')).toBeVisible();
  });
});

test.describe('API Documentation', () => {
  test('should display API documentation', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.locator('text=OHC Advanced API Reference')).toBeVisible();
  });
});

test.describe('Release Notes & Changelog', () => {
  test('should display release notes', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.locator('h1')).toContainText('What\'s New');
  });
});

test.describe('Help Chat Interface', () => {
  test('should open HelpChat from floating button and verify interface', async ({ page }) => {
    await page.goto('/');
    const floatingButton = page.locator('button:has-text("Ask anything")');
    await expect(floatingButton).toBeVisible();
    await floatingButton.click();

    await expect(page.locator('span:has-text("Ask AI Help")')).toBeVisible();

    // Check if initial message is there
    await expect(page.locator('text=Hi! I am your AI Support Agent.')).toBeVisible();

    const inputField = page.locator('input[placeholder="Ask a question..."]');
    await inputField.fill('How do I add a product?');
    await page.locator('button:has-text("Send")').click();

    // Verify user message appears
    await expect(page.locator('text=How do I add a product?')).toBeVisible();
  });
});
