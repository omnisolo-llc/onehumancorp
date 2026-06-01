import { test, expect } from '@playwright/test';

test.describe('Help Center Widget', () => {
  test.beforeEach(async ({ page }) => {
    // Go to any page that includes the Help Widget (ClientLayout)
    await page.goto('/');
  });

  test('should open and navigate help widget tabs', async ({ page }) => {
    // Open the widget
    const helpButton = page.getByRole('button', { name: /help|open help/i });
    if (await helpButton.isVisible()) {
        await helpButton.click();
    } else {
        // Fallback if button is not easily found by role
        await page.click('button:has-text("?")');
    }

    // Verify widget is visible
    const widget = page.locator('#help-widget-container').last();
    await expect(widget).toBeVisible();

    // Verify default tab content (Help Articles)
    await expect(page.locator('#help-widget-container').last()).toContainText(/Quick Answers|How to|Articles|Help/i);

    // Navigate to Ask AI
    await page.getByRole('button', { name: 'Ask AI' }).click();
    await expect(page.getByPlaceholder('Ask anything...')).toBeVisible();

    // Navigate to Videos
    await page.getByRole('button', { name: 'Videos' }).click();
    await expect(page.locator('#help-widget-container').last()).toContainText(/Video|Tutorial/i);

    // Navigate to New (Changelog)
    await page.getByRole('button', { name: 'New' }).click();
    await expect(page.getByText('What\'s New')).toBeVisible();
  });
});
