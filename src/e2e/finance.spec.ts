import { test, expect } from '@playwright/test';

// The server needs to be running. Since the sandbox uses mock data directly in the UI,
// we just test the frontend logic.
test('Finance dashboard shows offers and advances', async ({ page }) => {
  // Wait for login or navigate directly if mocked
  await page.goto('http://localhost:3000/finance');

  // Header
  await expect(page.locator('h1', { hasText: 'Finance' })).toBeVisible();

  // Ensure we see the Capital Advance Card
  await expect(page.locator('text=Need to stock up for the holidays?')).toBeVisible();

  // Verify Terms are shown
  await expect(page.locator('text=$1,500')).toBeVisible();
  await expect(page.locator('text=$150')).toBeVisible();
  await expect(page.locator('text=8% of daily sales')).toBeVisible();

  // Action Button
  const button = page.locator('button', { hasText: 'Get Funds Instantly' });
  await expect(button).toBeVisible();
});

test('Finance dashboard action flow', async ({ page }) => {
  await page.goto('http://localhost:3000/finance');

  const button = page.locator('button', { hasText: 'Get Funds Instantly' });
  await expect(button).toBeVisible();

  // Click the button to get funds
  await button.click();

  // Wait for some visual feedback (in a real app this would trigger an animation or state change)
  // Since our UI is simple, we just ensure it doesn't crash on click
  await expect(page.locator('h1', { hasText: 'Finance' })).toBeVisible();
});
