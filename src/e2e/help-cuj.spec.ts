import { test, expect } from './fixtures';

test.describe('Help Center CUJ', () => {

  test('CUJ 1: Help Center Navigation', async ({ page }) => {
    // Navigate via UI as a real user would from the index
    await page.goto('/');

    // Open Help Widget
    await page.locator('button[aria-label="Help"]').click();
    await expect(page.locator('#help-widget-container').last()).toBeVisible();

    // Click "Help" tab
    await page.locator('button:has-text("Help")').click();

    // Find and click "Getting Started"
    await page.locator('h4:has-text("Getting Started")').click();

    // Verify it navigates to the article and shows content
    await expect(page).toHaveURL(/\/help\/getting-started/);
    await expect(page.locator('h1:has-text("Getting Started with Your Store")')).toBeVisible();
    await expect(page.locator('text=Welcome to OneHumanCorp!')).toBeVisible();
  });

  test('CUJ 2: Contextual Tooltip', async ({ page }) => {
    await page.goto('/checkout');

    // Wait for the elements to load
    await expect(page.locator('button', { hasText: 'Pay Now' })).toBeVisible();

    // Let's use the first one since it's easier to find
    await page.locator('button', { hasText: 'Pay Now' }).hover();

    // Verify tooltip text appears for pay now
    await expect(page.locator('text=Click here to securely finish your purchase and process your payment.')).toBeVisible();
  });

  test('CUJ 3: Interactive Walkthrough', async ({ page }) => {
    await page.goto('/dashboard');

    // Open Help Widget
    await page.locator('button[aria-label="Help"]').click();
    await expect(page.locator('#help-widget-container').last()).toBeVisible();

    // Click "Tour: Accept your first payment"
    await page.locator('button:has-text("Tour: Accept your first payment")').click();

    // Verify walkthrough bubble appears
    await expect(page.locator('h3:has-text("Quick Guide")')).toBeVisible();
    await expect(page.locator('text=Click here to connect Stripe and start accepting payments.')).toBeVisible();

    // The walkthrough adds an overlay that should be visible
    await expect(page.locator('.fixed.z-\\[90\\].pointer-events-none.border-blue-500')).toBeVisible();
  });

  test('CUJ 4: AI Help Chat', async ({ page }) => {
    await page.goto('/dashboard');

    // Open Help Widget
    await page.locator('button[aria-label="Help"]').click();

    // Go to "Ask AI" tab
    await page.locator('button:has-text("Ask AI")').click();

    // Type a message and submit
    await page.locator('input[placeholder="Ask anything..."]').fill('How do I add a product?');
    await page.locator('button[aria-label="Send message"]').click();

    // Verify AI reply
    await expect(page.locator('text=I am your AI Help Agent!')).toBeVisible();
    await expect(page.locator('a:has-text("Read the full article →")')).toHaveAttribute('href', '/help');
  });

  test('CUJ 5: Video Tutorials', async ({ page }) => {
    await page.goto('/dashboard');

    // Open Help Widget
    await page.locator('button[aria-label="Help"]').click();

    // Click "Videos" tab
    await page.locator('button:has-text("Videos")').click();

    // Wait for the videos to fetch from API and render
    await expect(page.locator('text=How to set up your first store easily')).toBeVisible();

    // Click the first video to open the modal
    await page.locator('p:has-text("How to set up your first store easily")').click();

    // Verify modal is open (fake video player)
    await expect(page.locator('text=0:00 / 1:20')).toBeVisible();
  });
});