import { test, expect } from './fixtures';

test.describe('Unified AI Inbox CUJ', () => {

  test('Persona: Business Owner can swipe and approve AI Draft in Inbox', async ({ page }) => {
    // 1. Go to inbox directly
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: /Customer Inbox/i })).toBeVisible();

    // 2. Click the hidden generate draft button to seed data using JS because it's hidden
    await page.evaluate(() => {
        const btn = Array.from(document.querySelectorAll('button')).find(el => el.textContent?.includes('✨ AI Draft'));
        if (btn) btn.click();
    });

    // 3. Simulate generation of draft
    const aiDraftText = page.getByText(/AI Draft/i).first();
    await expect(aiDraftText).toBeVisible();

    // Verify draft UI
    await expect(page.getByText('Send').first()).toBeVisible();
    await expect(page.getByText('Edit').first()).toBeVisible();

    // Verify swipe right to send
    const draftCard = page.getByText(/AI Draft/i).first().locator('..');

    const draftBox = await draftCard.boundingBox();
    if (!draftBox) throw new Error("Draft box not found");

    await page.mouse.move(draftBox.x + 50, draftBox.y + 50);
    await page.mouse.down();
    await page.mouse.move(draftBox.x + 250, draftBox.y + 50, { steps: 10 });
    await page.mouse.up();

    // Wait for the new message to appear and the draft to disappear
    await expect(page.getByText('Yes, we have several vegan birthday cake options').first()).toBeVisible();
  });

  test('Persona: Business Owner can edit AI Draft manually without swiping in Inbox', async ({ page }) => {
    // 1. Go to inbox directly
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: /Customer Inbox/i })).toBeVisible();

    // 2. Click the hidden generate draft button to seed data using JS because it's hidden
    await page.evaluate(() => {
        const btn = Array.from(document.querySelectorAll('button')).find(el => el.textContent?.includes('✨ AI Draft'));
        if (btn) btn.click();
    });

    // 3. Simulate generation of draft
    const aiDraftText = page.getByText(/AI Draft/i).first();
    await expect(aiDraftText).toBeVisible();

    // The edit button should be visible to click
    const draftCard = page.getByText(/AI Draft/i).first().locator('..');

    await page.evaluate(() => {
        const btns = Array.from(document.querySelectorAll('button'));
        const editBtn = btns.reverse().find(el => el.textContent?.includes('Edit'));
        if (editBtn) editBtn.click();
    });

    // Verify edit mode is open
    await expect(page.locator('textarea[id^="reply-input-edit-"]').first()).toBeVisible();

    // Edit and send
    await page.locator('textarea[id^="reply-input-edit-"]').first().fill('Custom edited text!');

    // Click send
    await page.evaluate(() => {
        const textareas = Array.from(document.querySelectorAll('textarea'));
        const editTa = textareas.find(ta => ta.id && ta.id.startsWith('reply-input-edit-'));
        if (editTa && editTa.parentElement) {
            const btns = Array.from(editTa.parentElement.querySelectorAll('button'));
            const sendBtn = btns.find(b => b.textContent?.includes('Send'));
            if (sendBtn) sendBtn.click();
        }
    });

    // Wait for the new message to appear
    await expect(page.getByText('Custom edited text!').first()).toBeVisible();
  });
});
