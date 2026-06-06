import { test, expect } from './fixtures';

test.describe('Help Chat Flow', () => {
  test('should open help chat, type message, and see response', async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('/');

    // Check that the floating chat button exists
    const chatButton = page.getByRole('button', { name: 'Open help chat' });
    await expect(chatButton).toBeVisible();

    // Open chat
    await chatButton.click();

    // Verify chat UI appears
    const chatHeader = page.locator('#ai-chat-header');
    await expect(chatHeader).toBeVisible();
    await expect(page.getByText('Ask AI Help')).toBeVisible();
    await expect(page.getByText("Hi! I'm your AI Help Agent")).toBeVisible();

    // Type a message
    const input = page.getByPlaceholder('Ask me anything...');
    await input.fill('What is Operations?');

    // Submit
    const sendButton = page.getByRole('button', { name: 'Send message' });
    await sendButton.click();

    // Wait for the backend mocked response to appear
    await expect(page.locator('text=I have routed your request to the')).toBeVisible();

    // Verify link exists
    await expect(page.getByRole('link', { name: 'Check your inbox for updates →' })).toBeVisible();
  });
});

test.describe('Help Center Complete UI Flow', () => {
  test('should load Help Center, find videos, and click video to play', async ({ page }) => {
    await page.goto('/help');

    // Search for the video string
    const searchBox = page.getByPlaceholder('Search for help articles and videos...');
    await searchBox.fill('payment');

    // Wait for UI to filter
    await expect(page.getByText('Accept your first payment')).toBeVisible();

    // Click the video
    await page.getByText('Accept your first payment').click();

    // Expect the video player modal
    const videoModal = page.locator('video');
    await expect(videoModal).toBeVisible();

    // Close the modal
    const closeBtn = page.getByRole('button', { name: 'Close video' });
    await closeBtn.click();

    // Modal should be gone
    await expect(videoModal).not.toBeVisible();
  });
});

test.describe('Tooltip functionality', () => {
  test('should display tooltip on hover', async ({ page }) => {
    // Wait until tooltips load dynamically or are preloaded on Help page
    await page.goto('/api-docs');

    const tooltipTrigger = page.locator('span.cursor-help');
    await expect(tooltipTrigger).toBeVisible();

    await tooltipTrigger.hover();

    // Check if the tooltip wrapper gets rendered
    await expect(page.getByRole('tooltip')).toBeVisible();
    // Assuming the tooltip component role isn't 'tooltip', let's check text instead
    await expect(page.getByText('Direct API access is only for custom integrations.')).toBeVisible();
  });
});

test.describe('Changelog UX', () => {
  test('should ensure changelog renders beautiful design without placeholder text', async ({ page }) => {
    await page.goto('/changelog');

    await expect(page.getByRole('heading', { name: 'Version 1.0 (Latest)' })).toBeVisible();
    // Check that we removed the test line
    await expect(page.locator('text=This is a plain paragraph test line.')).not.toBeVisible();
  });
});
