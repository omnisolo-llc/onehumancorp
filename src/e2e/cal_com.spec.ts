import { test, expect } from '@playwright/test';

test.describe('Cal.com Integration Flow', () => {
  test('User can open Cal.com Connect Modal and configure Availability and Notice', async ({ page }) => {
    // 1. Navigate to the Integrations page
    await page.goto('/integrations');

    // 2. Verify we are on the integrations page
    await expect(page.locator('h1')).toHaveText('Tool Integrations');

    // 3. Find the Cal.com card and click Connect
    const calComCard = page.locator('.bg-white', { hasText: 'Cal.com' });
    await expect(calComCard).toBeVisible();

    const connectButton = calComCard.locator('button', { hasText: 'Connect' });
    await connectButton.click();

    // 4. Verify the Modal appears with the correct elements
    const modal = page.locator('.fixed.inset-0', { hasText: 'Connect Cal.com' });
    await expect(modal).toBeVisible();

    // Verify Weekly Availability select
    await expect(modal.locator('label', { hasText: 'Weekly Availability' })).toBeVisible();
    await expect(modal.locator('select').nth(0)).toHaveValue('Mon-Fri, 9:00 AM - 5:00 PM');

    // Verify Minimum Notice Period select
    await expect(modal.locator('label', { hasText: 'Minimum Notice Period' })).toBeVisible();
    await expect(modal.locator('select').nth(1)).toHaveValue('2 hours');

    // 5. Click Continue and verify UI state updates to "Manage" / "Connected"
    const continueButton = modal.locator('button', { hasText: 'Continue to Cal.com' });
    await continueButton.click();

    // The modal should close after timeout and button turns to Manage
    await expect(modal).toBeHidden({ timeout: 2000 });

    // Verify status changed to "connected" and button to "Manage"
    await expect(calComCard.locator('span', { hasText: 'connected' })).toBeVisible();
    await expect(calComCard.locator('button', { hasText: 'Manage' })).toBeVisible();
  });
});
