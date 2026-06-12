import { test, expect } from '@playwright/test';
import { adminUser } from './fixtures';

test.describe('Omnichannel Returns', () => {
    test.use({ storageState: 'test-results/admin.json' });

    test('should allow owner to approve an omnichannel return from triage feed', async ({ page }) => {
        await page.goto('/ui/dashboard.html');

        // Check if the card is visible in the triage feed
        const triageCard = page.locator('.triage-item:has-text("Return requested")');
        await expect(triageCard).toBeVisible({ timeout: 15000 });

        await expect(triageCard).toContainText('Return requested by Sarah for Order #1042');
        await expect(triageCard).toContainText('Operations Agent has generated a return label');

        const approveBtn = triageCard.locator('button:has-text("Approve")');
        await expect(approveBtn).toBeVisible();

        await approveBtn.click();

        // Check if the item vanishes or moves to the activity feed
        await expect(triageCard).not.toBeVisible();
    });
});
