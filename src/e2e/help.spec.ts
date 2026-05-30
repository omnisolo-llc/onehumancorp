import { test, expect } from './fixtures';

test.describe('Help Center & Documentation Widget', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/dashboard');
  });

  test('should display Help Center Widget button', async ({ page }) => {
    const helpButton = page.locator('button[aria-label="Help"]');
    await expect(helpButton).toBeVisible();
  });

  test('should open Help Widget and show Help Center tab by default', async ({ page }) => {
    const helpButton = page.locator('button[aria-label="Help"]');
    await helpButton.click();

    // Help Center tab button should be active
    const helpCenterTab = page.locator('button', { hasText: 'Help' }).first();
    await expect(helpCenterTab).toBeVisible();

    // Help Center content should be visible
    const helpCenterHeading = page.locator('h3', { hasText: 'Help Center' });
    await expect(helpCenterHeading).toBeVisible();
  });

  test('should switch to Chat tab', async ({ page }) => {
    const helpButton = page.locator('button[aria-label="Help"]');
    await helpButton.click();

    const chatTab = page.locator('button', { hasText: 'Ask AI' });
    await chatTab.click();

    const chatInput = page.getByPlaceholder('Ask anything...');
    await expect(chatInput).toBeVisible();
  });

  test('should switch to Videos tab', async ({ page }) => {
    const helpButton = page.locator('button[aria-label="Help"]');
    await helpButton.click();

    const videosTab = page.locator('button', { hasText: 'Videos' });
    await videosTab.click();

    const tutorialsHeading = page.locator('h3', { hasText: 'Tutorials' });
    await expect(tutorialsHeading).toBeVisible();
  });

  test('should switch to What\'s New tab', async ({ page }) => {
    const helpButton = page.locator('button[aria-label="Help"]');
    await helpButton.click();

    const whatsNewTab = page.locator('button', { hasText: "New" }).first();
    await whatsNewTab.click();

    const whatsNewHeading = page.locator('h3', { hasText: "What's New" }).first();
    await expect(whatsNewHeading).toBeVisible();
  });
});
