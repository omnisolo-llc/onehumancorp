import { test, expect } from './fixtures';

test.describe('Dashboard to Assistant Integration', () => {
  test('should link to WorkBuddy Assistant and navigate correctly', async ({ page }) => {
    // 1. Navigate to the dashboard
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // 2. Verify the Dashboard heading exists
    await expect(page.getByRole('heading', { name: 'Welcome back' })).toBeVisible();

    // 3. Locate the Open WorkBuddy Assistant link
    const assistantLink = page.getByRole('link', { name: 'Open WorkBuddy Assistant' });

    // 4. Verify it's visible and correctly styled
    await expect(assistantLink).toBeVisible();

    // 5. Click the link to navigate to the assistant page
    await assistantLink.click();
    await page.waitForLoadState('networkidle');

    // 6. Verify we reached the Agent Assistant page
    await expect(page.getByRole('heading', { name: 'Agent Assistant' })).toBeVisible();
  });
});
