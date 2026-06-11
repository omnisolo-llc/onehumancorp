import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Proactive Triage Feed', () => {
  adminPage('should see proactive insights and approve an action', async ({ page, serverUrl, tenantId }) => {
    // The test must go through the main dashboard feed where the triage insights appear.
    // Go to the unified home feed as a business owner would
    await page.goto(`${serverUrl}/`);

    // Wait for triage to load
    await page.waitForTimeout(2000);

    // Ensure the triage section is visible and loaded
    const triageSection = page.locator('#triage-section');
    await expect(triageSection).toBeVisible({ timeout: 15000 });

    // We expect some insights from the database seed
    const triageCards = triageSection.locator('.triage-item');
    await expect(triageCards.first()).toBeVisible({ timeout: 10000 });

    const cardText = await triageSection.textContent();
    expect(cardText).toContain('Maya requested a custom cake');

    const initialCount = await triageCards.count();

    const approveButton = triageCards.first().getByTestId('approve-btn');
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // Wait for the action to complete and verify the count goes down
    await expect(triageSection.locator('.triage-item')).toHaveCount(initialCount - 1);
  });
});
