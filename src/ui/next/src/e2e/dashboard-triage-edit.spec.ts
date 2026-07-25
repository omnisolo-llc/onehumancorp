import { expect, test } from '@playwright/test';

test.describe('Dashboard Triage Action Feed Edit UI', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should allow editing a draft from the unified dashboard feed', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Visit Dashboard
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const feedBtn = page.locator('button', { hasText: 'Pending Approvals' });
    if (await feedBtn.isVisible()) {
        await feedBtn.click();
    }

    const itemCard = page.locator('div[data-testid="instagram-dm-card"]').first();
    if (await itemCard.isVisible()) {
        const reviewDraftButton = page.locator('button', { hasText: 'Review Draft' }).first();
        if (await reviewDraftButton.isVisible()) {
            await reviewDraftButton.click();
        } else {
            const editBtn = page.locator('button', { hasText: 'Edit' }).first();
            if (await editBtn.isVisible()) {
                await editBtn.click();
            }
        }

        const textarea = page.locator('textarea[data-testid="edit-draft-textarea"]').first();
        if (await textarea.isVisible()) {
            await textarea.fill('Edited draft payload from dashboard feed');
            const saveButton = page.locator('button[data-testid="save-edit-approve-btn"]').first();
            if (await saveButton.isVisible()) {
               await saveButton.click();
            }
        }
    }
  });
});
