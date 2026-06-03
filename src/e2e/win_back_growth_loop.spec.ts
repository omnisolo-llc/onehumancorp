import { test, expect } from './fixtures';

test.describe('Customer Win-back Campaign Growth Loop', () => {
  test('should display the win-back campaign page and handle soft paywall', async ({ page }) => {
    await page.goto('/win-back');

    await expect(page.getByRole('heading', { name: 'Win-back Campaign' })).toBeVisible();

    const sendBtn = page.getByRole('button', { name: 'Send Campaign' });
    await expect(sendBtn).toBeVisible();
    await sendBtn.click();

    await expect(page.getByText(/Campaign sent!/i)).toBeVisible({ timeout: 5000 });
  });
});
