import { test, expect } from './fixtures';

test.describe('Videos Page', () => {
  test('should display videos', async ({ page }) => {
    await page.goto('/videos');
    await expect(page.getByRole('heading', { name: 'Video Tutorials' })).toBeVisible();
    await expect(page.getByText('How to set up your first store easily')).toBeVisible();
    await expect(page.getByText('1:20')).toBeVisible();
    await expect(page.getByText('Changing colors and logos')).toBeVisible();
  });
});
