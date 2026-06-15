import { test, expect } from '@playwright/test';

test.describe('Offline Quote-to-Cash Engine', () => {
  test('Owner can draft a quote, see it locally, and initiate offline tap to pay', async ({ page, context }) => {
    // Navigate to the field ops quote-to-cash page
    await page.goto('/field-ops/quote-to-cash');

    // Verify online state
    await expect(page.locator('text=Quick Quote')).toBeVisible();

    // Simulate going offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Verify offline indicator
    await expect(page.locator('text=Saved Offline')).toBeVisible();

    // Fill in the job details
    const notesArea = page.locator('textarea').first();
    await notesArea.fill('Found a leak under the sink, requires immediate pipe replacement quote for $150.');

    // Click Generate Quote
    const generateBtn = page.getByRole('button', { name: '🎤 Generate Quote' });
    await generateBtn.click();

    // Verify Quote Card appears
    await expect(page.getByText('Draft Quote')).toBeVisible();
    await expect(page.getByText('Services & Materials')).toBeVisible();
    // Verify the parsed amount appears
    await expect(page.locator('text=$150.00')).toHaveCount(2);

    // Click the Charge button (part of StripeTerminalClient)
    const chargeBtn = page.getByRole('button', { name: 'Charge $150.00' });
    await chargeBtn.click();

    // Verify offline payment queue message
    await expect(page.getByText('Synced locally. Will push to cloud when network is restored.')).toBeVisible();

    // Simulate going back online
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Sync is handled by SyncManager in background - we verify the offline UX flow
    await expect(page.locator('text=Online')).toBeVisible();
  });
});
