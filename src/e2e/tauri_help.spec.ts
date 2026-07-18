import { test, expect } from './fixtures';

test.describe('Help Center and Contextual Help (Tauri UI)', () => {
  test('Persona: Business Owner uses help center and chat', async ({ page }) => {
    // Navigate to local Tauri UI HTML
    await page.goto('/api/ui/dashboard.html?test_walkthrough=true');

    // Wait for page to load fully
    await page.waitForLoadState('networkidle');

    // Check if HelpChat component is accessible
    // Check if Videos tab works in dashboard widget
    const widgetBtn = page.locator('#ohc-floating-help-btn');
    await expect(widgetBtn).toBeVisible();
    await widgetBtn.dispatchEvent('click');

    const videosTab = page.locator('button[data-target="tab-videos"]');
    await expect(videosTab).toBeVisible();
    await videosTab.dispatchEvent('click');

    // Verify we fetch and render videos
    await expect(page.locator('#video-list')).not.toContainText('Loading videos...', { timeout: 10000 });
    await expect(page.locator('#video-list')).not.toContainText('Error loading videos.');

    await page.locator('#ohc-floating-help-close').dispatchEvent('click');

    const chatButton = page.locator('button[aria-label="Open help chat"]');
    await expect(chatButton).toBeVisible();
    await chatButton.dispatchEvent('click');
    await expect(page.locator('text=Ask AI Help').first()).toBeVisible();

    const input = page.locator('input[placeholder="Ask anything..."]');
    await input.fill('How do I accept credit cards?');
    await page.locator('button[aria-label="Send message"]').dispatchEvent('click');

    await expect(page.locator('text=How do I accept credit cards?').first()).toBeVisible();
    await expect(page.locator('text=I am your AI Help Agent!').first()).toBeVisible();
    await expect(page.locator('text=Read the full article').first()).toBeVisible();

    await page.locator('button[aria-label="Close help chat"]').dispatchEvent('click');

    // Go to /help
    await page.goto('/api/ui/help.html');
    await expect(page.locator('text=In-App Help Center').first()).toBeVisible();
    await expect(page.locator('text=Getting Started').first()).toBeVisible();

    await page.fill('input[placeholder="Search for help articles and videos..."]', 'paid');
    await expect(page.locator('text=Accepting Payments').first()).toBeVisible();
  });

  test('Persona: Business Owner views the Changelog', async ({ page }) => {
    await page.goto('/api/ui/changelog.html');
    await expect(page.locator('text=Release Notes & Changelog').first()).toBeVisible();
    await expect(page.locator('text=v0.4.48 (Cloud)').first()).toBeVisible();
    await expect(page.locator('text=Cloud Scaling Improvements').first()).toBeVisible();
  });

  test('Persona: Developer views the API documentation', async ({ page }) => {
    await page.goto('/api/ui/api-docs.html');
    await expect(page.locator('text=Advanced:').first()).toBeVisible();
    await expect(page.locator('text=OHC Advanced API Reference').first()).toBeVisible();
  });

  test('Persona: Business Owner interacts with a Tooltip', async ({ page }) => {
    await page.goto('/api/ui/dashboard.html?test_walkthrough=true');
    const shareLink = page.locator('#generate-link-btn');
    await expect(shareLink).toBeVisible();
    await shareLink.hover();
    await expect(page.locator('text=Click here to share access with a team member.').first()).toBeVisible();
  });
});
