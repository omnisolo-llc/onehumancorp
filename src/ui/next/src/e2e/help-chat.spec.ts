import { test, expect } from '@playwright/test';

test.describe('Help Chat Agent', () => {
  test('should return context-aware answers and links from help articles', async ({ page }) => {
    await page.goto('/dashboard?test_chat=true');

    // Click the "Ask anything" button
    const chatButton = page.locator('button', { hasText: 'Ask anything' });
    await expect(chatButton).toBeVisible();
    await chatButton.click();

    // Verify chat interface opened
    await expect(page.locator('h3', { hasText: 'Ask AI Help' })).toBeVisible();

    // Ask a question about getting started
    const input = page.getByPlaceholder('Ask me anything...');
    await input.fill('How do I get started?');
    await input.press('Enter');

    // Wait for the response
    await expect(page.locator('div', { hasText: 'Based on our help center: Welcome to One Human Corp!' }).last()).toBeVisible({ timeout: 10000 });

    // Verify the link is present
    const link = page.locator('a', { hasText: 'Read the full article →' }).last();
    await expect(link).toBeVisible();
    await expect(link).toHaveAttribute('href', '/help/getting-started');

    // Ask a question about billing
    await input.fill('Where is my billing info?');
    await input.press('Enter');

    // Wait for the response
    await expect(page.locator('div', { hasText: 'Based on our help center: Your monthly invoice shows exactly what you paid for.' }).last()).toBeVisible({ timeout: 10000 });

    // Verify the link is present
    const billingLink = page.locator('a', { hasText: 'Read the full article →' }).last();
    await expect(billingLink).toBeVisible();
    await expect(billingLink).toHaveAttribute('href', '/help/account-billing');
  });
});
