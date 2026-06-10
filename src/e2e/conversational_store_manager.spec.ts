import { test, expect } from './fixtures';

test.describe('Conversational Store Manager Growth Loop', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the store manager page
    await page.goto('/store-manager');
  });

  test('should render chat interface and process commands', async ({ page }) => {
    // Check page title
    await expect(page.locator('h1', { hasText: 'Store Manager AI' })).toBeVisible();

    // Verify initial greeting
    await expect(page.getByText('Good morning! You have 3 new orders. Should I schedule pickups?')).toBeVisible();

    // Type a command
    const textarea = page.locator('textarea[placeholder="Tell me what to do..."]');
    await textarea.fill('Yes, and create a 10% discount code for the weekend.');

    // Send command - the send button appears when there is text
    await page.locator('button').filter({ has: page.locator('svg') }).nth(1).click();

    // Verify user message appears
    await expect(page.getByText('Yes, and create a 10% discount code for the weekend.')).toBeVisible();

    // Verify agent response
    await expect(page.getByText(/Pickups scheduled\. Created discount code WEEKEND10\./)).toBeVisible({ timeout: 5000 });
  });

  test('should handle inventory intent with actionable buttons', async ({ page }) => {
    // Send inventory command
    const textarea = page.locator('textarea[placeholder="Tell me what to do..."]');
    await textarea.fill('Check my inventory');
    await page.locator('button').filter({ has: page.locator('svg') }).nth(1).click();

    // Wait for response and buttons
    await expect(page.getByText(/You are running low on Vanilla Extract/)).toBeVisible({ timeout: 5000 });

    // Click action button
    await page.getByRole('button', { name: 'Yes, order 2 bottles' }).click();

    // Verify mock action result
    await expect(page.getByText('Ordered 2 bottles of Vanilla Extract.')).toBeVisible();
  });
});
