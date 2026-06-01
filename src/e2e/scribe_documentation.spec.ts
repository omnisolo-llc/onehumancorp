import { test, expect } from './fixtures';

test.describe('Documentation Features', () => {
  test.beforeEach(async ({ memberPage: page }) => {
    await page.goto('/');
  });

  test('should display Help Widget and navigate tabs', async ({ memberPage: page }) => {
    // Wait for the help widget button to appear
    const helpBtn = page.getByRole('button', { name: 'Help' });
    await expect(helpBtn).toBeVisible();

    // Open Help Widget
    await helpBtn.click();
    await expect(page.locator('#help-widget-container')).toBeVisible();

    // Verify tabs
    await expect(page.getByRole('button', { name: 'Help' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Ask AI' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Videos' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'New' })).toBeVisible();
  });

  test('should use Help Center search', async ({ memberPage: page }) => {
    const helpBtn = page.getByRole('button', { name: 'Help' });
    await helpBtn.click();

    // Search for articles
    const searchInput = page.getByPlaceholder('Search for help...');
    await searchInput.fill('Getting Started');
    await expect(page.getByText('Getting Started')).toBeVisible();

    // Click an article link and verify navigation (mocked API or verify URL)
    const articleLink = page.getByRole('link', { name: 'Getting Started' });
    await expect(articleLink).toHaveAttribute('href', '/help/getting-started');
  });

  test('should interact with AI Help Chat', async ({ memberPage: page }) => {
    const helpBtn = page.getByRole('button', { name: 'Help' });
    await helpBtn.click();

    await page.getByRole('button', { name: 'Ask AI' }).click();
    await expect(page.getByText('Hi! I\'m your AI Support Agent.')).toBeVisible();

    const input = page.getByPlaceholder('Ask anything...');
    await input.fill('How do I add a product?');
    await page.getByRole('button', { name: 'Send message' }).click();

    await expect(page.getByText('How do I add a product?')).toBeVisible();
    await expect(page.getByText('I am your AI Help Agent! I specialize in answering questions about OHC features and helping you grow your small business.')).toBeVisible();
    await expect(page.getByRole('link', { name: 'Read the full article →' })).toBeVisible();
  });

  test('should display video tutorials', async ({ memberPage: page }) => {
    const helpBtn = page.getByRole('button', { name: 'Help' });
    await helpBtn.click();

    await page.getByRole('button', { name: 'Videos' }).click();

    // Check if a video is present and click it
    const firstVideo = page.getByText('How to set up your first store easily');
    await expect(firstVideo).toBeVisible();

    await firstVideo.click();
    // Check video modal
    await expect(page.locator('.animate-pop-in')).toBeVisible();
    await expect(page.locator('.animate-pop-in').getByText('How to set up your first store easily')).toBeVisible();
  });

  test('should view Release Notes', async ({ memberPage: page }) => {
    const helpBtn = page.getByRole('button', { name: 'Help' });
    await helpBtn.click();

    await page.getByRole('button', { name: 'New' }).click();
    await expect(page.getByText('New AI Store Builder')).toBeVisible();

    const changelogLink = page.getByRole('link', { name: 'Read full changelog →' });
    await expect(changelogLink).toBeVisible();
    await changelogLink.click();

    await expect(page.url()).toContain('/changelog');
    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();
  });

  test('should access API Docs correctly', async ({ memberPage: page }) => {
    await page.goto('/api-docs');
    await expect(page.getByText('Advanced: This section is for developers')).toBeVisible();
  });
});
