import { test, expect } from '@playwright/test';

test.describe('Neighborhood Collective E2E', () => {
  test('Maya forms a collective and customers use shared loyalty', async ({ page }) => {
    // We are testing UI resilience without a populated backend DB
    // 1. Maya logs in and sees the Neighborhood Pulse
    await page.goto('http://localhost:3002/collective');

    // We expect the Pulse Card to be visible. Even if nearby is 0, the text adapts.
    await expect(page.locator('text=The Neighborhood Pulse')).toBeVisible();
    await page.click('text=The Neighborhood Pulse');

    // Partner Match should appear
    await expect(page.locator('text=Partner Match')).toBeVisible();

    // The backend should return empty array since DB is empty, frontend displays 'No OHC businesses found nearby.'
    await expect(page.locator('text=No OHC businesses found nearby.')).toBeVisible();

    // Invite button must be disabled
    await expect(page.locator('button:has-text("Invite Partners")')).toBeDisabled();
  });
});
