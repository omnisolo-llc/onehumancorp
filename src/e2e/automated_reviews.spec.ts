import { test, expect } from './fixtures';

test.describe('Automated AI Review Requests', () => {
  test('exposes a growth loop for generating AI review requests from the dashboard', async ({ page }) => {
    // 1. Visit the Dashboard
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Welcome Back' })).toBeVisible();

    // 2. Find the AI Review Campaigns widget and click Generate
    const reviewCard = page.locator('text=AI Review Campaigns').locator('..');
    await expect(reviewCard).toBeVisible();

    const generateButton = reviewCard.getByRole('button', { name: 'Generate AI Review Request' });
    await expect(generateButton).toBeVisible();
    await generateButton.click();

    // 3. Verify the modal opens and displays generated content
    const modal = page.locator('text=AI Review Request').locator('..');
    await expect(modal).toBeVisible();

    // Wait for generation to complete (button text changes to Copy to Clipboard)
    const copyButton = modal.getByRole('button', { name: 'Copy to Clipboard' });
    await expect(copyButton).toBeEnabled({ timeout: 5000 });

    const textarea = modal.locator('textarea');
    await expect(textarea).toBeVisible();

    // Check that the generated message contains expected tokens
    const textValue = await textarea.inputValue();
    expect(textValue).toContain('Sarah'); // Contains the customer name
    expect(textValue).toContain('https://ohc.store/review/'); // Contains the review link

    // 4. Click copy button and verify state changes
    await copyButton.click();
    await expect(modal.getByRole('button', { name: 'Copied!' })).toBeVisible();
  });
});
