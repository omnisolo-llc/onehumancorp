import { expect, test } from "./fixtures";

test('Promoter Agent Flow navigates from dashboard and verifies promoter drafts in Agent Feed', async ({ page }) => {
    await page.goto('/dashboard');

    // Switch to Action Required tab where the feed lives
    const feedTab = page.locator('button', { hasText: 'Action Required' });
    if (await feedTab.isVisible()) {
        await feedTab.click();
    }

    // Check if the seeded promoter draft card is present
    const draftCard = page.locator('.bg-purple-50\\/50').or(page.locator('text=📣'));

    // Wait for it. This requires the DB to be seeded with e2e-approval-promoter
    await expect(draftCard.first()).toBeVisible({ timeout: 10000 });
});
