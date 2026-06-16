import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Field Service Quoting & Scheduling', () => {
  adminPage('Carlos persona: triage feed displays drafted quote and approves it with 1-tap', async ({ page }) => {
    // Navigate to the Dashboard which includes the agent feed (triage)
    await page.goto('/dashboard.html');

    // Wait for the agent feed to load
    await expect(page.getByTestId('quote-draft-card')).toBeVisible({ timeout: 15000 });

    const quoteCard = page.getByTestId('quote-draft-card').first();

    // Check if the card has the right controls
    await expect(quoteCard.getByTestId('approve-quote-draft')).toBeVisible();
    await expect(quoteCard.getByTestId('edit-quote-draft')).toBeVisible();
    await expect(quoteCard.getByTestId('reject-proposal')).toBeVisible();

    // Check if the card shows calculated total
    await expect(quoteCard).toContainText('Calculated Total');

    // Simulate 1-tap approval
    await quoteCard.getByTestId('approve-quote-draft').click();

    // Verify status message is shown
    await expect(page.locator('text="✨ Quote Sent & Time Slot Held!"').or(page.locator('text="Approved!"'))).toBeVisible();
  });
});
