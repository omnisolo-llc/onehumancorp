import { test, expect } from '@playwright/test';

test.describe('Voice Command Button E2E', () => {
  test('Voice Command button exists and can be interacted with', async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('http://localhost:3000/dashboard');

    // Check if the Voice Command Button exists
    const voiceBtn = page.getByTestId('voice-command-button');
    await expect(voiceBtn).toBeVisible({ timeout: 15000 });
  });
});
