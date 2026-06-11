import { test, expect } from '@playwright/test';

test.describe('Agent Protocol CUJ', () => {
  test('User can view and interact with Agent Protocol UI', async ({ page }) => {
    await page.goto('/agent-protocol');
    await expect(page.getByRole('heading', { name: 'Agent Protocol UI' })).toBeVisible();
    const hasError = await page.getByText('Error').isVisible();
    expect(hasError).toBe(false);
  });
});
