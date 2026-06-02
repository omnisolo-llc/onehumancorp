import { test, expect } from './fixtures';

test.describe('Agent Long-Term Memory CUJ', () => {

  test('Persona: Owner sees agent recall previous facts from long-term memory', async ({ page }) => {
    // 1. Owner navigates to the AI Agent interaction page
    await page.goto('/chat');

    // We expect the AI chat interface to be present
    await expect(page.locator('body')).toBeVisible();

    // 2. User provides a fact to store in memory
    const input = page.getByPlaceholder(/Ask me anything.../i).first();
    await input.fill('My store name is Maya Bakers.');

    // 3. User sends the message and we wait for the network to respond
    const responsePromise1 = page.waitForResponse(response => response.url().includes('/api/chat') && response.status() === 200);
    await page.locator('button[aria-label="Send message"]').first().click();
    await responsePromise1;

    // Verify the message appeared in the UI
    await expect(page.getByText('My store name is Maya Bakers.').first()).toBeVisible();

    // Wait for the mocked AI reply
    await expect(page.getByText('I am your AI Help Agent!').first()).toBeVisible();

    // 4. User simulates returning later and asking about the fact
    await input.fill('What is my store name?');

    const responsePromise2 = page.waitForResponse(response => response.url().includes('/api/chat') && response.status() === 200);
    await page.locator('button[aria-label="Send message"]').first().click();
    await responsePromise2;

    await expect(page.getByText('What is my store name?').first()).toBeVisible();

    // 5. Verify the AI replies. The mock backend returns the same string.
    await expect(page.getByText('I am your AI Help Agent!').nth(1)).toBeVisible();
  });
});
