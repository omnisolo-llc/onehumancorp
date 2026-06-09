import { test, expect } from './fixtures';

test.describe('Global Search Omnibox', () => {
  test('should open via Cmd+K, search and show results from seeded DB', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Wait for the page to be ready (e.g. Dashboard title)
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // Trigger Cmd+K (Meta+K for Mac, Control+K for Windows/Linux)
    await page.keyboard.press('Meta+K');

    // Fallback: If Meta+K didn't open it (e.g. non-mac test environment), try Control+K
    const searchInput = page.locator('#omnibox-input');

    // Check if it's visible. If not, trigger Control+K
    const isVisible = await searchInput.isVisible();
    if (!isVisible) {
      await page.keyboard.press('Control+K');
    }

    // Expect the input to be focused and visible
    await expect(searchInput).toBeVisible();
    await expect(searchInput).toBeFocused();

    // Type the name of the seeded customer 'Ava'
    await searchInput.fill('Ava');

    // Wait for the debounced network request and UI update
    const resultTitle = page.locator('.omnibox-item-title', { hasText: 'Ava Customer' });
    await expect(resultTitle).toBeVisible({ timeout: 5000 });

    // Verify subtitle has the correct email
    const resultSubtitle = page.locator('.omnibox-item-subtitle', { hasText: 'ava@example.com' });
    await expect(resultSubtitle).toBeVisible();

    // Verify entity type badge is present
    const badge = page.locator('.omnibox-item-title span', { hasText: 'customer' });
    await expect(badge).toBeVisible();

    // Close via Escape
    await page.keyboard.press('Escape');
    await expect(searchInput).toBeHidden();
  });
});
