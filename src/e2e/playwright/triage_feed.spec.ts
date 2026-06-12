import { test, expect } from '@playwright/test';
import { randomUUID } from 'crypto';

test.describe('Dashboard Triage Feed CUJ', () => {
    test('should show agent-drafted action cards and allow approval', async ({ page }) => {
        // Go to dashboard. We might need a specific tenant_id to see the seeded data.
        await page.goto('/dashboard?dashboard=1');
        await page.evaluate(() => {
            localStorage.setItem('tenant_id', 'test-tenant');
        });
        await page.reload();

        // 1. Verify we're on the dashboard
        await expect(page.locator('text=Unified Agent Feed')).toBeVisible({ timeout: 10000 });

        // 2. Look for the seeded triage item ("Maya requested a custom cake")
        const triageCard = page.locator('[data-testid^="triage-card-"]', { hasText: 'Maya requested a custom cake' }).first();
        await expect(triageCard).toBeVisible();

        // 3. Verify it shows the proposed action
        await expect(triageCard).toContainText('Send deposit link to Maya');

        // 4. Find the Approve button
        const approveBtn = triageCard.locator('[data-testid="approve-btn"]');
        await expect(approveBtn).toBeVisible();

        // 5. Click Approve
        await approveBtn.click();

        // 6. Verify optimistic UI update (the card should disappear or a success message is shown)
        await expect(page.locator('text=Approving...')).toBeVisible();
        await expect(page.locator('text=Approved!')).toBeVisible();

        // Wait for it to disappear from the list
        await expect(triageCard).not.toBeVisible();
    });
});
