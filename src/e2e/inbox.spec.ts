import { test, expect } from './fixtures';
import { judgeGeneratedOutput } from './ai-judge';

test.describe('Customer Inbox', () => {
  test('drafts and sends a reply', async ({ page }, testInfo) => {
    test.skip(!process.env.MINIMAX_API_KEY, 'MINIMAX_API_KEY is required for real AI draft generation and judging.');
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Customer Inbox' })).toBeVisible();
    await page.getByRole('button', { name: /AI Draft/ }).click();
    await expect(page.locator('#reply-input')).not.toHaveValue('');
    const draft = await page.locator('#reply-input').inputValue();
    await judgeGeneratedOutput(testInfo, {
      output: draft,
      rubric: 'The reply must directly answer that vegan birthday cake options are available, sound helpful and professional, avoid making unsupported promises, and be ready to send to a customer.',
    });
    await page.getByRole('button', { name: 'Send' }).click();
    await expect(page.locator('#messages-list')).toContainText(draft);
  });

  test('returns to dashboard on mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/inbox');

    // Test that the AI Assistant toggle works
    const toggle = page.getByRole('switch');
    await expect(toggle).toBeVisible();
    await toggle.click();
    await expect(toggle).toHaveAttribute('aria-checked', 'false');
    await toggle.click();
    await expect(toggle).toHaveAttribute('aria-checked', 'true');

    // Check visual warning and sparkle states
    await expect(page.getByText('Handled by AI').first()).toBeVisible();
    await expect(page.getByText('Needs your attention').first()).toBeVisible();

    // Click on a message to view detail
    await page.getByText('Do you offer vegan birthday cake options?').click();
    await expect(page.getByRole('button', { name: 'AI Draft' })).toBeVisible();
  });
});
