import { test, expect } from '@playwright/test';

test.describe('Help Features', () => {
  test('User can search and view articles in Help Center', async ({ page }) => {
    // Navigate directly to help page for the test
    await page.goto('/help');

    await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Getting Started' })).toBeVisible();

    // Search for articles
    await page.fill('input[placeholder="Search for help articles..."]', 'stock');
    await expect(page.locator('h2', { hasText: 'My Store' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Getting Started' })).toBeHidden();

    // Clear search and click on Getting Started article
    await page.fill('input[placeholder="Search for help articles..."]', '');
    await page.click('text=Getting Started');
    await expect(page.locator('h1', { hasText: 'Getting Started' })).toBeVisible();
    await expect(page.locator('text=Welcome to OneHumanCorp!')).toBeVisible();
  });

  test('User can view Changelog', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.locator('h1', { hasText: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Version 1.0 (Latest)' })).toBeVisible();
    await expect(page.locator('text=Interactive AI Store Builder:')).toBeVisible();
  });

  test('User can view API Documentation', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.locator('text=Advanced: This section is for developers directly integrating with our APIs.')).toBeVisible();
    // Swagger UI should load (might take a moment to mount the react component)
    await expect(page.locator('.swagger-ui')).toBeVisible({ timeout: 10000 });
  });

  test('User can open the Help Widget from the floating button', async ({ page }) => {
    await page.goto('/');
    // Check that the floating button is visible
    const helpBtn = page.locator('button[aria-label="Help"]');
    await expect(helpBtn).toBeVisible();
    await helpBtn.click();

    // Check that the widget container is visible and Help Center tab is selected by default
    await expect(page.locator('#help-widget-container')).toBeVisible();
    await expect(page.locator('button', { hasText: 'Help' })).toHaveAttribute('aria-pressed', 'true');
    await expect(page.locator('h3', { hasText: 'Help Center' })).toBeVisible();
  });

  test('User can navigate to the Ask AI tab and verify its content', async ({ page }) => {
    await page.goto('/');
    await page.click('button[aria-label="Help"]');

    // Click on Ask AI tab
    await page.click('button:has-text("Ask AI")');
    await expect(page.locator('button', { hasText: 'Ask AI' })).toHaveAttribute('aria-pressed', 'true');

    // Verify welcome message and input exist
    await expect(page.locator('text=Hi! I\'m your AI Support Agent')).toBeVisible();
    await expect(page.locator('input[placeholder="Ask anything..."]')).toBeVisible();
  });

  test('User can navigate to the Videos tab and open a video modal', async ({ page }) => {
    await page.goto('/');
    await page.click('button[aria-label="Help"]');

    // Click on Videos tab
    await page.click('button:has-text("Videos")');
    await expect(page.locator('h3', { hasText: 'Tutorials' })).toBeVisible();

    // Click on the first video (using text from the API response)
    const firstVideo = page.locator('p', { hasText: 'How to set up your first store easily' });
    // It might take a moment to load from the API
    await expect(firstVideo).toBeVisible({ timeout: 5000 });
    await firstVideo.click();

    // Verify modal is open and has the title
    await expect(page.locator('.fixed.inset-0.z-\\[100\\]')).toBeVisible();
    // Modal title should match
    await expect(page.locator('.fixed.inset-0.z-\\[100\\] h3', { hasText: 'How to set up your first store easily' })).toBeVisible();

    // Close the modal
    await page.locator('.fixed.inset-0.z-\\[100\\] button').first().click();
    await expect(page.locator('.fixed.inset-0.z-\\[100\\]')).toBeHidden();
  });

  test('User can navigate to the New tab (Changelog) and verify content', async ({ page }) => {
    await page.goto('/');
    await page.click('button[aria-label="Help"]');

    // Click on New tab
    await page.click('button:has-text("New")');
    await expect(page.locator('h3', { hasText: "What's New" })).toBeVisible();

    // Check some content
    await expect(page.locator('h4', { hasText: 'New AI Store Builder' })).toBeVisible();
    await expect(page.locator('a', { hasText: 'Read full changelog →' })).toBeVisible();
  });

  test('User can start the interactive Walkthrough from Help Center', async ({ page }) => {
    await page.goto('/');
    await page.click('button[aria-label="Help"]');

    // The "Help" tab should be active
    // Click on "Tour: Set up your store"
    await page.click('span:has-text("Tour: Set up your store")');

    // The walkthrough bubble should appear
    await expect(page.locator('.animate-pop-in')).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Quick Guide' })).toBeVisible();
    await expect(page.locator('text=Enter your business description.')).toBeVisible();

    // Click next
    await page.click('button:has-text("Next")');
    await expect(page.locator('text=Click to generate!')).toBeVisible();

    // Click finish
    await page.click('button:has-text("Finish")');
    await expect(page.locator('.animate-pop-in')).toBeHidden();
  });

});
