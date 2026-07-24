import { test, expect } from '@playwright/test';

test.describe('Omnichannel Inbox', () => {
  test('should display the conversation list and thread', async ({ page }) => {
    // Navigate to the inbox page
    await page.goto('/inbox');

    // Verify inbox list is displayed
    await expect(page.locator('h1').filter({ hasText: 'Inbox' })).toBeVisible();
    await expect(page.getByText('Maya Baker')).toBeVisible();
    await expect(page.getByText('Requested vegan cake quote')).toBeVisible();

    // Click on the conversation to open the thread
    await page.getByText('Maya Baker').click();

    // Verify conversation thread is displayed
    await expect(page.locator('h1').filter({ hasText: 'Maya Baker' })).toBeVisible();
    await expect(page.getByText('Hi, do you do custom vegan cakes?')).toBeVisible();

    // Verify AI suggest button
    const aiSuggestBtn = page.getByRole('button', { name: '✨ AI Suggest' });
    await expect(aiSuggestBtn).toBeVisible();

    // Click AI suggest and verify input is filled
    await aiSuggestBtn.click();
    const input = page.getByPlaceholder('Type a message...');
    await expect(input).toHaveValue('Hi there! Yes, we do offer custom vegan cakes. What kind of flavors were you thinking of?');

    // Send the message
    await page.getByRole('button', { name: '↑' }).click();

    // Verify message is sent
    await expect(page.getByText('Hi there! Yes, we do offer custom vegan cakes. What kind of flavors were you thinking of?')).toBeVisible();
  });
});
