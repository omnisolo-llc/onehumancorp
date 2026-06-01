import { test, expect } from '@playwright/test';

test.describe('Operations Department Performance Settings', () => {
  test('should display performance settings card in Operations department', async ({ page }) => {
    // Navigate to team page directly, simulating logged in state by default as it's an internal test
    await page.goto('/team');

    // Click on the Operations department card (The Manager)
    await page.getByRole('button', { name: /The Manager/i }).click();

    // Verify the Approval Inbox title for Operations
    await expect(page.getByRole('heading', { name: /The Manager/i })).toBeVisible();

    // Verify the Performance Settings card is visible
    await expect(page.getByRole('heading', { name: /Performance/i })).toBeVisible();
    await expect(page.getByText('Storefront optimized for speed: ON')).toBeVisible();
    await expect(page.getByText('Aggressive edge caching is enabled globally to handle traffic spikes.')).toBeVisible();
  });

  test('should NOT display performance settings card in other departments', async ({ page }) => {
    await page.goto('/team');

    // Click on Marketing department (The Promoter)
    await page.getByRole('button', { name: /The Promoter/i }).click();

    // Verify the Approval Inbox title for Marketing
    await expect(page.getByRole('heading', { name: /The Promoter/i })).toBeVisible();

    // Verify the Performance Settings card is NOT visible
    await expect(page.getByRole('heading', { name: /Performance/i })).not.toBeVisible();
    await expect(page.getByText('Storefront optimized for speed: ON')).not.toBeVisible();
  });
});
