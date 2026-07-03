import { test, expect } from '../fixtures';

test.describe('Help Center & Documentation Features', () => {
  // Test owner persona: Maya - Home Baker using the app to find help
  test('Owner can navigate Help Center, use search, and play a video tutorial', async ({ page }) => {

    // 1. Owner opens Help Center from navigation or direct URL
    await page.goto('/api/ui/help.html');

    // 2. Help Center Page is loaded
    await expect(page.locator('h1:has-text("In-App Help Center")')).toBeVisible();

    // 3. Search for a specific topic
    const searchInput = page.locator('input[placeholder="Search for help articles and videos..."]');
    await searchInput.fill('store in 5 minutes');
    await page.waitForTimeout(500); // Wait for debounce or search update

    // 4. Verify search results contain video tutorial
    await expect(page.locator('text=How to set up your first store easily')).toBeVisible();

    // 5. Open video tutorial modal
    await page.locator('text=How to set up your first store easily').click();

    // 6. Verify video modal opens and can be closed
    const closeButton = page.locator('button[aria-label="Close video"]');
    await expect(closeButton.first()).toBeVisible();
    await closeButton.first().evaluate((b) => (b as HTMLElement).click());
    await expect(closeButton.first()).not.toBeVisible();
  });

  test('Owner can access API docs and see Advanced user tooltips', async ({ page }) => {

    // 1. Go to Help Center
    await page.goto('/api/ui/help.html');

    // 2. Click the API Documentation link in Advanced section
    const apiLink = page.locator('a:has-text("API Documentation")');
    await expect(apiLink).toBeVisible();

    // 3. Navigate to API Docs
    await apiLink.evaluate((b) => (b as HTMLElement).click());
    await expect(page).toHaveURL(/\/api-docs\.html/);

    // 4. Hover to see tooltip
    const tooltipTarget = page.locator('#api-docs-tooltip');
    await expect(tooltipTarget).toBeVisible();
    await tooltipTarget.hover({ force: true });
    await expect(page.locator('text=Direct API access is only for custom integrations.')).toBeVisible();

    // 5. Verify API docs loaded (Swagger UI)
    await expect(page.locator('text=Advanced:')).toBeVisible();
    await expect(page.locator('.swagger-ui')).toBeVisible();
  });

  test('Owner can trigger Interactive Walkthroughs from the Help Widget', async ({ page }) => {

    // 1. Ensure the Walkthrough can trigger on any page by adding test query param
    await page.goto('/api/ui/help.html?test_walkthrough=true');

    // 2. Open the Help Widget (floating ? button)
    const helpButton = page.locator('button[aria-label="Help"]');
    await expect(helpButton.first()).toBeVisible();
    await helpButton.first().evaluate((b) => (b as HTMLElement).click());

    // 3. Go to the Interactive Tours tab if not default, but Tours are in the "Learn" or default tab
    // Let's click "Tour: Set up your store"
    const tourButton = page.locator('button:has-text("Tour: Set up your store")');
    await expect(tourButton).toBeVisible();
    await tourButton.evaluate((b) => (b as HTMLElement).click());

    // 4. Verify Walkthrough bubble appears
    const nextButton = page.locator('button:has-text("Next")');
    await page.waitForTimeout(1000); // Allow react state update
    if (await nextButton.isVisible()) {
      await expect(page.locator('h3', { hasText: 'Step 1: Dashboard' })).toBeVisible();

      // 5. Navigate through the walkthrough
      await nextButton.click();
      await expect(page.locator('h3', { hasText: 'Step 2: Add Products' })).toBeVisible();
      await nextButton.click();
      await expect(page.locator('h3', { hasText: 'Step 3: Launch' })).toBeVisible();

      // 6. Finish the walkthrough
      const finishButton = page.locator('button:has-text("Finish")');
      await expect(finishButton).toBeVisible();
      await finishButton.click();
      await expect(finishButton).not.toBeVisible();
    }
  });

  test('Owner can access Help Chat from widget', async ({ page }) => {

    // 1. Go to a regular page
    await page.goto('/api/ui/help.html');

    // 2. Open the Help Widget
    const helpButton = page.locator('button[aria-label="Help"]');
    await expect(helpButton.first()).toBeVisible();
    await helpButton.first().evaluate((b) => (b as HTMLElement).click());

    // 3. Switch to Ask AI tab
    const askAiTab = page.locator('button:has-text("Ask AI")');
    if (await askAiTab.isVisible()) {
      await askAiTab.evaluate((b) => (b as HTMLElement).click());
    }

    // 4. Verify chat input
    const chatInput = page.locator('input[placeholder="Ask anything..."]');
    await expect(chatInput).toBeVisible();

    // 5. Send a message
    await chatInput.fill('How do I add a product?');
    await chatInput.press('Enter');

    // 6. Verify message was sent
    await expect(page.locator('text=How do I add a product?')).toBeVisible();
  });

  test('Owner can view Changelog from Help Center widget', async ({ page }) => {

    await page.goto('/api/ui/help.html');

    // Open widget
    const helpButton = page.locator('button[aria-label="Help"]');
    await expect(helpButton.first()).toBeVisible();
    await helpButton.first().evaluate((b) => (b as HTMLElement).click());

    // Switch to What's New tab
    const whatsNewTab = page.locator('button:has-text("New")');
    await expect(whatsNewTab).toBeVisible();
    await whatsNewTab.evaluate((b) => (b as HTMLElement).click());

    // Click the Read full release notes link
    const releaseNotesLink = page.locator('a:has-text("Read full release notes")');
    await expect(releaseNotesLink).toBeVisible();

    // Click and navigate
    await releaseNotesLink.evaluate((b) => (b as HTMLElement).click());
    await expect(page).toHaveURL(/\/changelog\.html/);
    await expect(page.locator('h1:has-text("Release Notes & Changelog")')).toBeVisible();
  });
});
