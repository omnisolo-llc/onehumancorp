import { test, expect } from '@playwright/test';

test.describe('Documentation Features', () => {

  test('Help Center page loads, displays topics, and search filtering works', async ({ page }) => {
    await page.goto('/help');

    // Check header
    await expect(page.locator('h1:has-text("Help Center")')).toBeVisible();

    // Check that some topics are visible
    await expect(page.locator('h2:has-text("Getting Started")')).toBeVisible();
    await expect(page.locator('h2:has-text("Getting Paid")')).toBeVisible();

    // Test search filtering
    const searchInput = page.getByPlaceholder('Search for help articles...');
    await searchInput.fill('Getting Paid');

    // Getting Paid should be visible, others like 'Account & Billing' should not
    await expect(page.locator('h2:has-text("Getting Paid")')).toBeVisible();
    await expect(page.locator('h2:has-text("Account & Billing")')).toBeHidden();
  });

  test('AI-Powered Help Chat floating button works', async ({ page }) => {
    await page.goto('/'); // It is loaded globally in layout.tsx

    // The chat button
    const askButton = page.getByRole('button', { name: 'Open help chat' });
    await expect(askButton).toBeVisible();
    await askButton.click();

    // The chat UI should open
    await expect(page.locator('h3:has-text("Help Agent")')).toBeVisible();

    // Send a message
    const chatInput = page.getByPlaceholder('Ask me anything...');
    await chatInput.fill('How do I add a product?');

    const sendButton = page.getByRole('button', { name: 'Send message' });
    await sendButton.click();

    // The user's message should appear
    await expect(page.locator('text=How do I add a product?')).toBeVisible();

    // The mock response should appear
    await expect(page.locator('text=I am your AI Help Agent!')).toBeVisible();
  });

  test('Tooltip functionality works', async ({ page }) => {
    await page.goto('/');

    // Hover over the help widget button which has id="help-btn-tooltip"
    // Wait for the help widget to load if needed
    const helpButton = page.locator('button[aria-label="Help"]');
    await expect(helpButton).toBeVisible();

    // Simulate hover
    await helpButton.hover();

    // The tooltip should appear
    // Tooltips are rendered as portals in fixed position with animation
    const tooltipText = page.locator('text=Need help? Click here for guides, videos, and to ask our AI.'); // From API tooltips, default was: "Need help? Click here to access our Help Center, Ask AI, Video Tutorials, and Release Notes."
    // We should wait for the tooltip text to be visible
    await expect(page.getByText('Need help? Click here for guides, videos, and to ask our AI.')).toBeVisible({ timeout: 5000 });
  });

  test('API Documentation page loads the Swagger UI successfully', async ({ page }) => {
    await page.goto('/api-docs');

    // Check advanced notice
    await expect(page.locator('text=Advanced:')).toBeVisible();

    // Swagger UI renders a swagger-ui class
    await expect(page.locator('.swagger-ui')).toBeVisible();
    await expect(page.locator('text=OHC Advanced API Reference')).toBeVisible();
  });

  test('Changelog page loads and displays release notes correctly', async ({ page }) => {
    await page.goto('/changelog');

    await expect(page.locator('h1:has-text("Release Notes & Changelog")')).toBeVisible();
    await expect(page.locator('text=Version 1.0 (Latest)')).toBeVisible();

    // The changelog has a specific feature listed
    await expect(page.locator('text=Interactive AI Store Builder')).toBeVisible();
  });

});
