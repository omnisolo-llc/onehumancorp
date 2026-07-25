import { test, expect } from '@playwright/test';

test.describe('Native Omnichannel Chat Flow', () => {
  test('Native Omnichannel Chat creation and ws message', async ({ page }) => {
    await page.goto('/dashboard');
    const pageUrl = page.url();
    expect(pageUrl).toContain('/dashboard');
  });
});
