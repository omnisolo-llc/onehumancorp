import { test, expect } from './fixtures';

test.describe('Documentation & Help Interaction Features', () => {
  test('1. should navigate and view a specific Help Article', async ({ page }) => {
    await page.goto('/help');

    // Check main title
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();

    // Wait for articles to load and click the "Getting Started with Your Store" article
    const firstArticle = page.locator('text=Getting Started with Your Store').first();
    await expect(firstArticle).toBeVisible({ timeout: 10000 });

    await firstArticle.click();

    // Verify article page
    await expect(page.getByRole('heading', { name: 'Getting Started with Your Store' })).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Welcome to OneHumanCorp!')).toBeVisible();

    // Verify Back Link works
    await page.getByRole('link', { name: 'Back to Help Center' }).click();
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible({ timeout: 10000 });
  });

  test('2. should open and send a message in the AI Help Chat', async ({ page }) => {
    await page.goto('/dashboard');

    // Find the floating chat button and open it
    const chatButton = page.getByRole('button', { name: 'Open help chat' });
    await expect(chatButton).toBeVisible();
    await chatButton.click();

    // Verify header and initial message
    await expect(page.locator('#ai-chat-header')).toBeVisible();
    await expect(page.locator('text=Need help setting up your store')).toBeVisible();

    // Type a message
    const input = page.getByPlaceholder('Ask me anything...');
    await expect(input).toBeVisible();
    await input.fill('How do I set up my store?');

    // Send
    await page.getByRole('button', { name: 'Send message' }).click();

    // Verify we see our message
    await expect(page.locator('text=How do I set up my store?')).toBeVisible();
    // Verify the AI replies
    await expect(page.locator('text=To set up your storefront, go to the \'My Store\' tab')).toBeVisible({ timeout: 10000 });

    // Close the chat
    const closeButton = page.getByRole('button', { name: 'Close help chat' });
    await closeButton.click();
    await expect(page.locator('#ai-chat-header')).not.toBeVisible();
  });

  test('3. should trigger tooltip interaction successfully', async ({ page }) => {
    await page.goto('/api-docs');

    // Find the Advanced span that has the tooltip trigger
    const advancedLabel = page.locator('span.font-outfit.cursor-help.font-bold').filter({ hasText: 'Advanced:' });
    await expect(advancedLabel).toBeVisible({ timeout: 10000 });

    // Trigger mouse enter
    await advancedLabel.hover();

    // Verify tooltip text appears
    await expect(page.locator('text=Direct API access is only for custom integrations.')).toBeVisible();

    // Un-hover
    await page.mouse.move(0, 0);
    await expect(page.locator('text=Direct API access is only for custom integrations.')).not.toBeVisible();
  });

  test('4. should open Video Tutorials list and click a video', async ({ page }) => {
    await page.goto('/help/videos');

    await expect(page.getByRole('heading', { name: 'Video Guides' })).toBeVisible();

    // Wait for video list to load
    const videoItem = page.locator('text=How to set up your store').first();
    await expect(videoItem).toBeVisible({ timeout: 10000 });

    // Click the video item to open modal
    await videoItem.click();

    // Verify modal and close button exist
    const closeBtn = page.getByRole('button', { name: 'Close video' });
    await expect(closeBtn).toBeVisible();

    // Verify the native video tag is present
    await expect(page.locator('video')).toBeVisible();

    // Close modal
    await closeBtn.click();
    await expect(page.locator('video')).not.toBeVisible();
  });

  test('5. should navigate to the Changelog and verify layout', async ({ page }) => {
    await page.goto('/changelog');

    // Verify headers
    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Version 1.0 (Latest)' })).toBeVisible();

    // Verify content list exists
    await expect(page.locator('text=Interactive AI Store Builder:')).toBeVisible();

    // Verify external link exists
    await expect(page.getByRole('link', { name: 'Read the full technical changelog' })).toBeVisible();
  });
});

test.describe('Dashboard Navigation (Legacy checks)', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/dashboard');
  });

  test('should display dashboard with nav', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.getByRole('navigation', { name: 'Primary' })).toBeVisible();
  });

  test('should show dashboard link in nav', async ({ page }) => {
    const dashLink = page.getByRole('link', { name: 'Dashboard' });
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

test.describe('Login Page', () => {
  test('should display login form', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('input[type="password"]').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('button:has-text("Login")')).toBeVisible();
  });
});

test.describe('Agents Page', () => {
  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
  });

  test('should show hire agent button', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('button:has-text("Hire Agent")')).toBeVisible();
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
