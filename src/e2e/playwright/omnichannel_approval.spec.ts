import { test, expect } from '@playwright/test';

test.describe('Omnichannel Ambassador', () => {
  test('receives webhook and drafts message for 1-tap approval', async ({ page }) => {
    await page.goto('/');
  });
});
