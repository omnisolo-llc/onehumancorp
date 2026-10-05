import { test, expect } from '../fixtures';

test.describe('Help Center & Documentation Features', () => {
  // Test owner persona: Maya - Home Baker using the app to find help
  test('Owner can navigate Help Center, use search, and play a video tutorial', async ({ page }) => {

    // 1. Owner opens Help Center from navigation or direct URL
    await page.goto('/api/v1/ui/help.html');

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
    await closeButton.first().click();
    await expect(closeButton.first()).not.toBeVisible();
  });

  test('Owner can access API docs and see Advanced user tooltips', async ({ page }) => {

    // 1. Go to Help Center
    await page.goto('/api/v1/ui/help.html');

    // 2. Click the API Documentation link in Advanced section
    const apiLink = page.locator('a:has-text("API Documentation")').first();
    await expect(apiLink).toBeVisible();

    // 3. Follow the maintained reference and wait for its real specification.
    await expect(apiLink).toHaveAttribute('href', '/api-docs');
    const loaded = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/api-docs-spec' && response.request().method() === 'GET');
    await apiLink.click();
    await page.waitForLoadState('domcontentloaded');
    await expect(page).toHaveURL(/\/api-docs$/);
    const response = await loaded;
    expect(response.status()).toBe(200);
    const spec = await response.json();
    expect(spec.info.title).toBe('API Documentation (for Advanced Users)');

    // 4. Hover to see tooltip
    const tooltipTarget = page.locator('#api-docs-tooltip');
    await expect(tooltipTarget).toBeVisible();
    await tooltipTarget.hover();
    const tooltip = page.getByRole('tooltip');
    await expect(tooltip).toHaveText('Direct API access is only for custom integrations.');
    await expect(tooltip).toBeVisible();
    await page.mouse.move(0, 0);
    await expect(tooltip).toBeHidden();
    await tooltipTarget.focus();
    await expect(tooltip).toBeVisible();
    await expect(tooltipTarget).toHaveAttribute('aria-describedby', 'api-docs-tooltip-description');
    await tooltipTarget.press('Escape');
    await expect(tooltip).toBeHidden();
    await expect(tooltipTarget).toBeFocused();

    // 5. Verify API docs loaded (Swagger UI)
    await expect(page.locator('text=Advanced:')).toBeVisible();
    await expect(page.locator('.swagger-ui').first()).toBeVisible();
    await expect(page.locator('.swagger-ui .info .title')).toContainText(spec.info.title);
  });

  test('Owner can trigger Interactive Walkthroughs from the Help Widget', async ({ page }) => {

    // 1. Ensure the Walkthrough can trigger on any page by adding test query param
    await page.goto('/api/v1/ui/help.html?test_walkthrough=true');

    // 2. Open the Help Widget (floating ? button)
    const helpButton = page.locator('button[aria-label="Help"]');
    await expect(helpButton.first()).toBeVisible();
    await helpButton.first().click();

    const widget = page.locator('#ohc-floating-help-widget');
    await widget.getByRole('button', { name: 'Interactive Tours', exact: true }).click();
    await widget.locator('.omnisolo-tour-card').filter({ hasText: 'Set up your store' }).click();
    const bubble = page.locator('#walkthrough-bubble');
    await expect(bubble).toBeVisible();
    await expect(bubble).toContainText('Set up your store');
    await expect(bubble).toContainText('Click here to access your storefront and add your first products.');
    await bubble.getByRole('button', { name: 'Finish', exact: true }).click();
    await expect(bubble).toBeHidden();
  });

  test('Owner can access Help Chat from widget', async ({ page }) => {

    // 1. Go to a regular page
    await page.goto('/api/v1/ui/help.html');

    // 2. Open the Help Widget
    const helpButton = page.locator('button[aria-label="Help"]');
    await expect(helpButton.first()).toBeVisible();
    await helpButton.first().click();

    // 3. Switch to Ask AI tab
    const askAiTab = page.locator('button:has-text("Ask AI")');
    await expect(askAiTab).toBeVisible();
    await askAiTab.click();

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

    await page.goto('/api/v1/ui/help.html');

    // Open widget
    const helpButton = page.locator('button[aria-label="Help"]');
    await expect(helpButton.first()).toBeVisible();
    await helpButton.first().click();

    // Switch to What's New tab
    const widget = page.locator('#ohc-floating-help-widget');
    const whatsNewTab = widget.getByRole('button', { name: 'New', exact: true });
    await expect(whatsNewTab).toBeVisible();
    await whatsNewTab.click();

    // Click the Read full release notes link
    const releaseNotesLink = page.locator('a:has-text("Read full release notes")');
    await expect(releaseNotesLink).toBeVisible();
    await expect(releaseNotesLink).toHaveAttribute('href', '/changelog');

    // Click and navigate
    await releaseNotesLink.click();
    await expect(page).toHaveURL((url) => url.pathname === '/changelog');
    await expect(page.locator('h1:has-text("Release Notes & Changelog")')).toBeVisible();
  });
});
