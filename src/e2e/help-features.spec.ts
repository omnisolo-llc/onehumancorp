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

    // Clear search and search for another specific item
    await page.fill('input[placeholder="Search for help articles..."]', 'payment');
    await expect(page.locator('h2', { hasText: 'Getting Paid' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Getting Started' })).toBeVisible(); // Because it contains "first payment"

    // Search for non-existent item
    await page.fill('input[placeholder="Search for help articles..."]', 'gibberish');
    await expect(page.locator('text=No articles found matching "gibberish"')).toBeVisible();

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

  test('User can trigger tooltip hover on UI elements', async ({ page }) => {
    // Navigate to a page with tooltips
    const responsePromise = page.waitForResponse('/api/tooltips');
    await page.goto('/');

    // Wait for the tooltips to be fetched
    const response = await responsePromise;
    expect(response.status()).toBe(200);

    // Hover the nav-agents link as it typically has a tooltip 'nav-agents-tooltip'
    const agentsLink = page.locator('nav a:has-text("Agents")');
    await expect(agentsLink).toBeVisible();
    await agentsLink.hover();

    // "See your AI team, give them tasks, or hire new helpers." should appear from the tooltip registry
    await expect(page.locator('text=See your AI team, give them tasks, or hire new helpers.')).toBeVisible({ timeout: 5000 });
  });
});
