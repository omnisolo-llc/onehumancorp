import { test, expect } from './fixtures';

test.describe('Viral Milestones Loop', () => {
  test('should display Milestones Page and copy share message', async ({ page }) => {
    await page.goto('/milestones');

    // Check if the growth component is visible
    await expect(page.getByRole('heading', { name: 'Success Milestones 🏆' })).toBeVisible();
    await expect(page.getByText('First Order! 🎉')).toBeVisible();

    // Click on a milestone
    await page.getByText('First Order! 🎉').click();

    // Verify modal content
    await expect(page.getByRole('heading', { name: 'Share Your Success' })).toBeVisible();

    // Test copy button interaction
    await page.getByRole('button', { name: 'Copy Share Message' }).click();
    await expect(page.getByRole('button', { name: 'Copied Message!' })).toBeVisible();
  });
});
