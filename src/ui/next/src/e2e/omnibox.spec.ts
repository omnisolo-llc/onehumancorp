import { test, expect } from '@playwright/test';

test.describe('Global Search / Omnibox', () => {
  test('should open omnibox and display search results', async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('/dashboard');

    // Wait for AppShell (specifically the body or some element to indicate we are loaded enough to accept keyboard events)
    await expect(page.locator('.app-page')).toBeVisible();

    // Press Cmd+K or Ctrl+K to open the omnibox
    await page.keyboard.press('Control+k');

    // The omnibox should be visible
    const searchInput = page.getByPlaceholder('Search customers, orders, or messages...');
    try {
        await expect(searchInput).toBeVisible({ timeout: 2000 });
    } catch {
        await page.keyboard.press('Meta+k');
        await expect(searchInput).toBeVisible();
    }

    // Type a query
    await searchInput.fill('John');

    // Should see the mocked search results
    await expect(page.getByText('John Doe')).toBeVisible();
    await expect(page.getByText('Order ord-123')).toBeVisible();

    // Click on a result to navigate
    await page.getByText('John Doe').click();

    // The omnibox should close
    await expect(searchInput).not.toBeVisible();

    // Assuming the URL updates based on our getHref logic in Omnibox
    await expect(page).toHaveURL(/.*\/customers\/cust-1/);
  });
});
