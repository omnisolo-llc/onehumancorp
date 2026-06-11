import { test, expect } from '@playwright/test';

test.describe('SONA Neural Patterns CUJ', () => {
  test('User can view and interact with SONA neural patterns', async ({ page }) => {
    await page.goto('/sona');
    await expect(page.getByRole('heading', { name: 'SONA Neural Patterns Dashboard' })).toBeVisible();
    const hasError = await page.getByText('Error loading patterns').isVisible();
    expect(hasError).toBe(false);
  });
});
