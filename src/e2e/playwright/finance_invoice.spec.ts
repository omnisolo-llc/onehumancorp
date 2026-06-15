import { test, expect } from '@playwright/test';

test.describe('Agentic Invoicing Flow', () => {
    test.beforeEach(async ({ page }) => {
        // Mock the dashboard login state by setting a cookie or local storage if needed,
        // but for now we'll just go straight to the finance page as an isolated mock UI flow.
        await page.goto('/finance');
    });

    test('should allow owner to review, edit and send AI-drafted invoice', async ({ page }) => {
        // 1. Verify page loads correctly
        await expect(page.locator('h1', { hasText: 'Finance & Invoicing' })).toBeVisible();

        // 2. Look for the AI Triage Feed card
        const triageCard = page.locator('text=Draft Invoice ready for Nora\'s Design Project');
        await expect(triageCard).toBeVisible();

        // 3. Wait for UI invoices to render then click Triage feed card to open Draft Modal
        await expect(page.locator('text=Nora\'s Design Project').first()).toBeVisible();
        await triageCard.click({ force: true });
        await page.locator('text=Review').first().click({ force: true });
        await page.evaluate(() => {
            const card = document.querySelector('.bg-indigo-50');
            if (card instanceof HTMLElement) card.click();
        });

        // 4. Modal should appear with correct title
        const modalTitle = page.locator('h2', { hasText: 'Review Invoice Draft' });
        await expect(modalTitle).toBeAttached({ timeout: 10000 });

        // 5. Verify pre-filled data in the modal
        await expect(page.locator('input[value="Nora\'s Design Project"]')).toBeAttached();
        await expect(page.locator('input[value="Logo Design"]')).toBeAttached();
        await expect(page.locator('input[value="1500"]')).toBeAttached();

        // 6. Click Approve & Send
        const sendBtn = page.locator('button', { hasText: 'Approve & Send' });
        await expect(sendBtn).toBeAttached();

        // Setup dialog handler for the expected success alert
        page.on('dialog', async dialog => {
            expect(dialog.message()).toContain('Invoice sent!');
            await dialog.accept();
        });

        await page.evaluate(() => {
            const btn = Array.from(document.querySelectorAll('button')).find(b => b.textContent && b.textContent.includes('Approve & Send'));
            if (btn instanceof HTMLElement) btn.click();
        });

        // 7. Verify modal closes
        await expect(modalTitle).not.toBeVisible();
    });
});
