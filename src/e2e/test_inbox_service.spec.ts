import { expect, test } from './fixtures';

test.describe('Omnichannel Inbox Mobile UI & Ambassador E2E', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('inbox UI renders correctly and handles actual E2E data', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.locator('text="Inbox"').first()).toBeVisible({ timeout: 25000 });
    const bodyWidth = await page.evaluate(() => document.body.scrollWidth);
    expect(bodyWidth).toBeLessThanOrEqual(375);

    const messageItems = page.locator('.app-list-item');
    if (await messageItems.count() > 0) {
        await messageItems.first().click();
        const draftReplyLabel = page.locator('text="Draft Reply"').first();
        if (await draftReplyLabel.isVisible()) {
            const draftReplyContainer = draftReplyLabel.locator('..').locator('div.mt-2').first();
            const classList = await draftReplyContainer.getAttribute('class');
            expect(classList).toContain('backdrop-blur-md');
            expect(classList).toContain('bg-[#FFD700]/10');
            const approveButton = page.locator('button', { hasText: 'Approve & Send Draft' }).first();
            if (await approveButton.isVisible()) {
              const box = await approveButton.boundingBox();
              expect(box?.height).toBeGreaterThanOrEqual(44);
              expect(box?.width).toBeGreaterThanOrEqual(44);
            }
        }
    }
  });
});
