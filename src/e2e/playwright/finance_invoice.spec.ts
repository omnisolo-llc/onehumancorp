import { test, expect } from '@playwright/test';

test.describe('Agentic Invoicing Flow', () => {
    test.beforeEach(async ({ page }) => {
        // Mock the dashboard login state by setting a cookie or local storage if needed,
        // but for now we'll just go straight to the finance page as an isolated mock UI flow.
        await page.goto('/finance');
    });

    test('should allow owner to review, edit and send AI-drafted invoice', async ({ page }) => {
        // 1. Verify page loads correctly
        await expect(page.locator('h1')).toHaveText('Finance & Invoicing');

        // 2. Look for the AI Triage Feed card
        const triageCard = page.locator('text=Draft Invoice ready for Nora\'s Design Project');
        await expect(triageCard).toBeVisible();

        // 3. Click Triage feed card to open Draft Modal
        await triageCard.click();

        // 4. Modal should appear with correct title
        const modalTitle = page.locator('h2', { hasText: 'Review Invoice Draft' });
        await expect(modalTitle).toBeVisible();

        // 5. Verify pre-filled data in the modal
        await expect(page.locator('input[value="Nora\'s Design Project"]')).toBeVisible();
        await expect(page.locator('input[value="Logo Design"]')).toBeVisible();
        await expect(page.locator('input[value="1500"]')).toBeVisible();

        // 6. Click Approve & Send
        const sendBtn = page.locator('button', { hasText: 'Approve & Send' });
        await expect(sendBtn).toBeVisible();

        // Setup dialog handler for the expected success alert
        page.on('dialog', dialog => dialog.accept());
        await sendBtn.click();

        // 7. Verify modal closes
        await expect(modalTitle).not.toBeVisible();
    });
});
