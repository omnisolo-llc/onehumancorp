import { test, expect } from './fixtures';
import { judgeGeneratedOutput } from './ai-judge';

test.describe('Customer Inbox', () => {
  test('drafts and sends a reply', async ({ page }, testInfo) => {
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Customer Inbox' })).toBeVisible();

    // Open conversation
    await page.locator('text=Maya (Instagram)').click();

    // Trigger AI draft
    await page.getByRole('button', { name: /AI Draft Reply/ }).click();

    // Ensure shimmer loading is visible (briefly) or draft panel appears
    await expect(page.locator('#ai-draft-panel')).toBeVisible({ timeout: 10000 });

    await expect(page.locator('#reply-input')).not.toHaveValue('');
    const draft = await page.locator('#reply-input').inputValue();

    // Skip real AI judging if no API key
    if (process.env.MINIMAX_API_KEY) {
      await judgeGeneratedOutput(testInfo, {
        output: draft,
        rubric: 'The reply must directly answer that vegan birthday cake options are available, sound helpful and professional, avoid making unsupported promises, and be ready to send to a customer.',
      });
    } else {
      console.log('Skipped AI judging due to missing MINIMAX_API_KEY');
    }

    // Click send draft
    await page.getByRole('button', { name: 'Send Draft' }).click();
    await expect(page.locator('#messages-list')).toContainText(draft);
  });

  test('returns to dashboard on mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/inbox');
    await page.getByRole('button', { name: '< Back' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });
});
