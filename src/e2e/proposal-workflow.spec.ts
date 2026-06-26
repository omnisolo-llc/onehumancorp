import { test, expect } from '@playwright/test';

test.describe('The Closer Proposal Workflow', () => {
    test('should draft and approve a proposal', async ({ page }) => {
        // Mock a scenario where a quote is requested via the API,
        // and we verify the UI for the proposal card.
        await page.goto('file:///app/src/ui/tauri/src/ui/proposal-card.html');

        const card = page.locator('#proposal-card');
        await expect(card).toBeVisible();

        await expect(page.locator('text=🤖 The Closer (Sales/Finance Agent)')).toBeVisible();

        // Check edit toggle
        const reviewBtn = page.locator('button:has-text("Review / Edit")');
        await reviewBtn.click();

        const editMode = page.locator('#edit-mode');
        await expect(editMode).toHaveClass(/active/);

        // Mock save
        page.on('dialog', dialog => dialog.accept());
        const saveBtn = page.locator('button:has-text("Save & Update")');
        await saveBtn.click();

        // Check approve toggle
        const approveBtn = page.locator('button:has-text("Approve & Send Link")');
        await approveBtn.click();

        // Since we mock alert, we can just check if opacity changed to 0.5
        await expect(card).toHaveCSS('opacity', '0.5');
    });
});
