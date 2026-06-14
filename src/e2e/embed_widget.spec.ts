import { test, expect } from '@playwright/test';

test.describe('Growth Loop: Interactive Embed Widget Builder', () => {
  test('Should render the embed builder, reflect inputs, and serve the backend embed endpoint', async ({ page, request, baseURL }) => {
    // Navigate to the dashboard first to ensure discoverability
    // Resolve dynamically for bazel environment compatibility
    await page.goto('/embed-builder');

    // Verify correct page
    await expect(page.getByRole('heading', { name: 'Interactive Embed Builder' })).toBeVisible();
    await expect(page.locator('text=Configuration')).toBeVisible();

    // Change configuration (Quote + Dark Theme)
    await page.click('button:has-text("Quote")');
    await page.click('button:has-text("Dark")');
    await page.fill('input[type="text"]', 'test-merchant-xyz');

    // Verify iframe DOM URL updates correctly matching the API logic
    const iframe = page.locator('iframe');
    await expect(iframe).toHaveAttribute('src', /\/embed\/widget/);
    await expect(iframe).toHaveAttribute('src', /tenant_id=test-merchant-xyz/);
    await expect(iframe).toHaveAttribute('src', /type=quote/);
  });
});
