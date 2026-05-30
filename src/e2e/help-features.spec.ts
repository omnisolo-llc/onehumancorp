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

  test('User can use AI Help Chat', async ({ page }) => {
    await page.route('**/api/chat', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ reply: "This is a mocked success reply." })
      });
    });

    await page.goto('/help');

    // Open chat
    const openChatBtn = page.getByLabel('Open help chat');
    await expect(openChatBtn).toBeVisible();
    await openChatBtn.click();

    // Verify initial message is present
    await expect(page.locator('text=Need help setting up your store')).toBeVisible();

    // Send a message
    const input = page.getByPlaceholder('Ask me anything...');
    await input.fill('How do I add a product?');
    await page.getByLabel('Send message').click();

    // Verify user message appears
    await expect(page.locator('text=How do I add a product?')).toBeVisible();

    // Check if bot replies with the success message
    await expect(page.locator('text=This is a mocked success reply.')).toBeVisible({ timeout: 5000 });
  });

  test('User can view contextual tooltips on hover', async ({ page }) => {
    // team-activity-tooltip is present on dashboard
    await page.goto('/'); // assuming dashboard is at /

    // Dashboard has <h2 ...>Team Activity</h2> wrapped in <WithTooltip id="team-activity-tooltip" ...>
    const trigger = page.locator('text=Team Activity');

    // Sometimes tests run too fast for the page to fully load elements, wait for it
    await expect(trigger).toBeVisible();

    // Hover to trigger tooltip
    await trigger.hover();

    // The tooltip renders as a fixed div with the text
    const tooltipText = page.locator('text=Monitor the real-time actions and tasks being performed by your AI workforce.');
    await expect(tooltipText).toBeVisible();
  });
});
