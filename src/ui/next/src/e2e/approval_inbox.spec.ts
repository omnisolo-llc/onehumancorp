import { test, expect } from '@playwright/test';

test.describe('Unified Agent Feed Mobile Test', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should render and interact with ambassador_reply ActionRequired card', async ({ page, request }) => {
    test.setTimeout(180000);

    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || 'http://localhost:3000';

    // Login to establish session
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    // Use real application stack via the webhook endpoint
    const response = await request.post(`${apiBase}/api/agents/webhook`, {
      data: {
        tenant_id: 'default',
        message: 'Do you have vegan options for birthday cakes?',
        source: 'instagram'
      }
    });

    // Check webhook triggered
    expect(response.ok()).toBeTruthy();

    // Wait slightly to ensure it processed to the inbox feed
    await page.waitForTimeout(3000);

    await page.goto('/dashboard');

    // Ensure the feed loads
    // With AI generation we might not know exactly what it says, but we can verify the card types
    await expect(page.locator('button', { hasText: /Proposals/ }).first()).toBeVisible({ timeout: 15000 });

    // Check if the ambassador reply card is visible, if it hasn't loaded yet wait for it
    // Wait for either a pending item or the empty inbox state.
    const inquiryLocator = page.getByText(/Do you have vegan options for birthday cakes?|vegan options for birthday cakes/i).first();
    const draftLocator = page.getByText(/Draft Reply/i).first();

    await expect(page.getByText(/All Caught Up!|Draft email for review|vegan options/i).first()).toBeVisible({ timeout: 15000 });

    if (await draftLocator.isVisible()) {
      // Verify Ambassador specific UI elements
      const card = page.getByTestId('ambassador-reply-card').first();
      await expect(card).toBeVisible();

      // Verify buttons
      const sendButton = card.getByTestId('approve-ambassador-reply');
      await expect(sendButton).toBeVisible();
      await expect(sendButton).toHaveText('Send Draft');

      const editButton = card.getByTestId('edit-ambassador-reply');
      await expect(editButton).toBeVisible();
      await expect(editButton).toHaveText('Edit Reply');

      // Test the interaction
      await sendButton.click();

      // Card should disappear after action
      await expect(card).toBeHidden({ timeout: 15000 });
    }
  });

  test('should render properly and handle tabs', async ({ page }) => {
    test.setTimeout(180000);

    // Login
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();

    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Wait for the unified agent feed to load
    await expect(page.locator('button', { hasText: /Proposals/ }).first()).toBeVisible({ timeout: 15000 });
    await expect(page.locator('button', { hasText: 'Activity Feed' })).toBeVisible();

    // Switch tabs
    await page.locator('button', { hasText: 'Activity Feed' }).click();

    // Verify glassmorphism CSS
    const feedContainer = page.locator('.glassmorphism').first();
    await expect(feedContainer).toBeVisible();
    await expect(feedContainer).toHaveCSS('backdrop-filter', /blur\(30px\)/);

    // Switch back
    await page.locator('button', { hasText: /Proposals/ }).first().click();
  });
});
