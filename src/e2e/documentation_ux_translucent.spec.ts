import { test, expect } from './fixtures';

test.describe('Documentation Translucent UX', () => {

  test('Walkthrough bubble has translucent glass styling', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/api/ui/dashboard.html');
    const walkthroughBtn = page.locator('#dashboard-walkthrough-btn');
    await expect(walkthroughBtn).toBeVisible();
    await walkthroughBtn.click();

    const bubble = page.locator('.ohc-walkthrough-bubble');
    await expect(bubble).toBeVisible();
    // Validate classes
    await expect(bubble).toHaveClass(/backdrop-blur-xl/);
    await expect(bubble).toHaveClass(/bg-white\/80/);
    await expect(bubble).toHaveClass(/dark:bg-black\/50/);
  });

  test('Help Chat interface has translucent glass styling', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/api/ui/help.html');
    const helpChatButton = page.locator('button[aria-label="Open help chat"]');
    await expect(helpChatButton).toBeVisible();
    await helpChatButton.click();

    const chatInterface = page.locator('#ai-chat-interface');
    await expect(chatInterface).toBeVisible();
    // Validate classes
    await expect(chatInterface).toHaveClass(/backdrop-blur-xl/);
    await expect(chatInterface).toHaveClass(/bg-white\/80/);
    await expect(chatInterface).toHaveClass(/dark:bg-black\/50/);
  });

  test('Help Center search input has translucent glass styling', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/api/ui/help.html');

    const searchInput = page.getByTestId('help-search-input');
    await expect(searchInput).toBeVisible();
    // Validate classes
    await expect(searchInput).toHaveClass(/backdrop-blur-xl/);
    await expect(searchInput).toHaveClass(/bg-white\/80/);
    await expect(searchInput).toHaveClass(/dark:bg-black\/50/);
  });

  test('Tooltip has translucent glass styling', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/api/ui/api-docs.html');

    const tooltipTrigger = page.locator('.cursor-help').first();
    await expect(tooltipTrigger).toBeVisible();
    await tooltipTrigger.hover();

    // We get the tooltip from the portal
    const tooltip = page.locator('.fixed.z-\\[100\\]').first();
    await expect(tooltip).toBeVisible({ timeout: 10000 });
    // Validate classes
    await expect(tooltip).toHaveClass(/backdrop-blur-xl/);
    await expect(tooltip).toHaveClass(/bg-white\/80/);
    await expect(tooltip).toHaveClass(/dark:bg-black\/50/);
  });

  test('API Docs container has translucent glass styling', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/api/ui/api-docs.html');

    // The container for the swagger UI
    const container = page.locator('.max-w-6xl').nth(1); // Second max-w-6xl is the main container
    await expect(container).toBeVisible({ timeout: 10000 });
    // Validate classes
    await expect(container).toHaveClass(/backdrop-blur-xl/);
    await expect(container).toHaveClass(/bg-white\/80/);
    await expect(container).toHaveClass(/dark:bg-black\/50/);
  });

});
