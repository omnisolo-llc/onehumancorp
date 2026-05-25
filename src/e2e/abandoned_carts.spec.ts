import { test, expect } from './fixtures';

test.describe('Viral Abandoned Carts Recovery E2E', () => {
  test('exposes AI cart recovery settings and verifies functionality', async ({ page }) => {
    // Navigate using the Next.js app route
    await page.goto('/abandoned-carts');

    // Check main heading
    await expect(page.getByRole('heading', { name: 'AI Abandoned Cart Recovery' })).toBeVisible();

    // Check toggle functionality
    await expect(page.getByRole('heading', { name: 'Enable AI Recovery' })).toBeVisible();

    // Enable the recovery sequence
    await page.getByRole('button', { name: /Enable AI Recovery/i }).click();

    // Assert success toast
    await expect(page.getByText('Success! AI Recovery Sequence is now active.')).toBeVisible();
  });
});
