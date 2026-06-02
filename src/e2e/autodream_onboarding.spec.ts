import { test, expect } from '@playwright/test';

test.describe('AutoDream Conversational Onboarding', () => {
  test('should generate and scaffold a business from a conversational prompt', async ({ page }) => {
    // Navigate to the conversational onboarding page
    await page.goto('/autodream');

    // Verify initial state: Agent greeting
    await expect(page.locator('text=Operations Manager')).toBeVisible();
    await expect(page.locator("text=Hi! I'm your Operations Manager")).toBeVisible();

    // The user describes their business
    const inputField = page.locator('input[placeholder="Describe your business..."]');
    await inputField.fill('I bake vegan cakes in Austin');

    // Submit the prompt
    await page.locator('button[type="submit"]').click();

    // Verify user message appears in chat
    await expect(page.locator('text=I bake vegan cakes in Austin')).toBeVisible();

    // Verify simulated agent reasoning messages
    await expect(page.locator('text=Analyzing business type...')).toBeVisible();
    await expect(page.locator('text=Generating storefront design...')).toBeVisible();

    // Verify scaffold result preview card
    await expect(page.locator('text=Draft Ready for Review')).toBeVisible();
    await expect(page.locator('text=Maya\'s Cakes')).toBeVisible(); // Mocked response for "bake" keyword
    await expect(page.locator('text=Home Bakery')).toBeVisible();

    // Approve the scaffold
    const approveButton = page.locator('button:has-text("Approve & Launch")');
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // Should redirect to dashboard
    await expect(page).toHaveURL(/\/dashboard/);
  });
});
