import { test, expect } from './fixtures';

test.describe('Help Center', () => {
  test('Persona: Business Owner uses help center and chat', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard?test_chat=true');

    // Wait for page to load fully
    await page.waitForLoadState('networkidle');

    // Check if HelpChat component is accessible
    const chatButton = page.locator('button[aria-label="Open help chat"]');
    await expect(chatButton).toBeVisible();
    await chatButton.click();
    await expect(page.locator('text=Ask AI Help').first()).toBeVisible();

    const input = page.locator('input[placeholder="Ask me anything..."]');
    await input.fill('How do I accept credit cards?');
    await page.locator('button[aria-label="Send message"]').click();

    await expect(page.locator('text=How do I accept credit cards?').first()).toBeVisible();
    await expect(page.locator('text=I am your AI Help Agent!').first()).toBeVisible();
    await expect(page.locator('text=Read the full article').first()).toBeVisible();

    await page.locator('button[aria-label="Close help chat"]').click();

    // Go to /help
    await page.goto('/help');
    await expect(page.locator('text=Help Center').first()).toBeVisible();
    await expect(page.locator('text=Getting Started').first()).toBeVisible();
    await expect(page.locator('text=My Store').first()).toBeVisible();

    await page.click('text=Getting Started');
    await expect(page).toHaveURL(/.*\/help\/getting-started/);
    await expect(page.locator('text=Getting Started with Your Store').first()).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Step 1: Tell us about your business' })).toBeVisible();

    await page.goto('/help');

    await page.fill('input[placeholder="Search for help articles and videos..."]', 'paid');
    await expect(page.locator('text=Getting Paid').first()).toBeVisible();
  });

  test('Persona: Business Owner views the Changelog', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.locator('text=Release Notes & Changelog').first()).toBeVisible();
    await expect(page.locator('text=Version 1.0 (Latest)').first()).toBeVisible();
    await expect(page.locator('text=New Features').first()).toBeVisible();
  });

  test('Persona: Developer views the API documentation', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.locator('text=Advanced:').first()).toBeVisible();
    await expect(page.locator('text=OHC Advanced API Reference').first()).toBeVisible();
  });

  test('Persona: Business Owner interacts with a Tooltip', async ({ page }) => {
    await page.goto('/dashboard');
    const kairosLink = page.locator('a[href="/kairos"]');
    await expect(kairosLink).toBeVisible();
    await kairosLink.hover();
    await expect(page.locator('text=Click here to see what your AI helpers are working on and how they plan.').first()).toBeVisible();
  });

  test('Persona: Business Owner navigates to KAIROS page', async ({ page }) => {
     await page.goto('/kairos');
     // Ensure page loaded
     await expect(page.getByRole('heading', { name: 'Kairos' })).toBeVisible();
  });
});
