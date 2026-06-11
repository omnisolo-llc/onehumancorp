import { test, expect } from '@playwright/test';

test.describe('Agent Marketplace CUJ', () => {
  test('User can view and interact with Agent Marketplace', async ({ page }) => {
    await page.goto('/agent-marketplace');
    await expect(page.getByRole('heading', { name: 'Agent Marketplace' })).toBeVisible();
    const hasError = await page.getByText('Error').isVisible();
    expect(hasError).toBe(false);
  });
});
