import { test, expect } from '@playwright/test';

test.describe('Quoting UI e2e', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('owner can navigate to quoting page, view a real quote from the backend, and approve it', async ({ page }) => {
    // Navigate to the quoting page
    await page.goto('/quoting?id=1');

    await expect(page.locator('text=Review Draft Quote')).toBeVisible({ timeout: 15000 });

    const approveBtn = page.getByRole('button', { name: 'Approve & Send' });
    await expect(approveBtn).toBeVisible();

    await approveBtn.click();
  });
});
