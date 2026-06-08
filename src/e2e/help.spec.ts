import { test, expect } from './fixtures';
test.describe('Help Center', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/dashboard');
  });
  test('should display dashboard with nav', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.getByRole('navigation', { name: 'Primary' })).toBeVisible();
  });
  test('should show dashboard link in nav', async ({ page }) => {
    const dashLink = page.getByRole('link', { name: 'Dashboard', exact: true });
    await expect(dashLink).toBeVisible();
  });
  test('should show agents link in nav', async ({ page }) => {
    const agentsLink = page.getByRole('link', { name: 'Agents' });
    await expect(agentsLink).toBeVisible();
  });
  test('should show setup link in nav', async ({ page }) => {
    const setupLink = page.getByRole('link', { name: 'Setup' });
    await expect(setupLink).toBeVisible();
  });
  test('should display welcome message', async ({ page }) => {
    await expect(page.locator('text=Welcome back')).toBeVisible();
  });
  test('should display agents working message', async ({ page }) => {
    await expect(page.locator('text=Your agents are working on your behalf')).toBeVisible();
  });
});
test.describe('Agents Page', () => {
  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
  });
});
test.describe('Business Setup Page', () => {
  test('should display setup page', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' })).toBeVisible();
  });
  test('should show setup wizard text', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.locator('text=Your business, live in minutes')).toBeVisible();
  });
});
test.describe('Dashboard', () => {
  test('should have working nav links', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('link', { name: 'Agents' }).click();
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
  });
});

test.describe('Documentation Pages', () => {
  test('should display Help Center main page', async ({ page }) => {
    await page.goto('/help');
    await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();
  });

  test('should display Changelog page', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.locator('h1', { hasText: 'Release Notes & Changelog' })).toBeVisible();
  });

  test('should display API Docs page', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.locator('text=Advanced:')).toBeVisible();
  });

  test('should display Video Tutorials page', async ({ page }) => {
    await page.goto('/help/videos');
    await expect(page.locator('h1', { hasText: 'Video Guides' })).toBeVisible();
  });
});

test.describe('Help Center Interactions', () => {
    test('renders help center and navigates to an article', async ({ page }) => {
        await page.goto('/help');

        // Verify Help Center title
        await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

        // Verify that categories are rendered (Getting Started, My Store, Payments)
        await expect(page.locator('h2', { hasText: 'Getting Started' })).toBeVisible();
        await expect(page.locator('h2', { hasText: 'My Store' })).toBeVisible();
        await expect(page.locator('h2', { hasText: 'Payments' })).toBeVisible();

        // Search for an article
        const searchInput = page.getByPlaceholder('Search for help articles and videos...');
        await searchInput.fill('Getting Started');

        // Click on the article
        const articleLink = page.locator('a[href="/help/getting-started-1"]');
        await expect(articleLink).toBeVisible();
        await articleLink.click();

        // Wait for navigation and API load
        await page.waitForURL('/help/getting-started-1');

        // Verify article content
        await expect(page.locator('h1', { hasText: 'Getting Started' })).toBeVisible();
        await expect(page.locator('p', { hasText: 'Welcome to OneHumanCorp!' })).toBeVisible();

        await page.goto('/help');
        await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();
    });

    test('should use backend search for filtering articles', async ({ page }) => {
        await page.goto('/help');

        // Verify Help Center title
        await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

        // Search for an article that matches My Store
        const searchInput = page.getByPlaceholder('Search for help articles and videos...');

        // Use Promise.all to wait for the request to the search endpoint
        const [response] = await Promise.all([
            page.waitForResponse(response =>
                response.url().includes('/api/help/search') && (response.status() === 200 || response.status() === 304)
            ),
            searchInput.fill('My Store')
        ]);

        // Wait for UI to update
        const articleLink = page.locator('a[href="/help/my-store"]');
        await expect(articleLink).toBeVisible();
    });

    test('should open help chat and send a message', async ({ page }) => {
        await page.goto('/help');

        // Find and click the floating Ask anything button
        const chatButton = page.locator('button[aria-label="Open help chat"]');
        await expect(chatButton).toBeVisible();
        await chatButton.click();

        // Wait for the chat to open and be visible
        const chatHeader = page.locator('#ai-chat-header');
        await expect(chatHeader).toBeVisible();

        // Check if the chat input is present
        const chatInput = page.locator('input[placeholder="Ask me anything..."]');
        await expect(chatInput).toBeVisible();

        // Type a message and send it
        const testMessage = 'How do I add a product?';
        await chatInput.fill(testMessage);
        const sendButton = page.locator('button[aria-label="Send message"]');
        await expect(sendButton).toBeVisible();
        await sendButton.click();

        // Assert that the message appears in the chat
        const sentMessage = page.locator('div', { hasText: testMessage }).last();
        await expect(sentMessage).toBeVisible();

        // Close the chat
        const closeButton = page.locator('button[aria-label="Close help chat"]');
        await closeButton.click();
        await expect(chatHeader).not.toBeVisible();
    });
});
