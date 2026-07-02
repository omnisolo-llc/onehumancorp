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

        // 3. Click Triage feed card to open Draft Modal
        await triageCard.click();

        // 4. Modal should appear with correct title
        const modalTitle = page.locator('h2', { hasText: 'Review Invoice Draft' });
        await expect(modalTitle).toBeVisible();

        // 5. Verify pre-filled data in the modal
        await expect(page.locator('input[value="New Client"]')).toBeVisible();
        await expect(page.locator('input[value="Consulting Services"]')).toBeVisible();
        await expect(page.locator('input[value="100"]')).toBeVisible();

        // 6. Click Approve & Send
        const sendBtn = page.locator('button', { hasText: 'Approve & Send' });
        await expect(sendBtn).toBeVisible();

        // Setup dialog handler for the expected success alert
        page.on('dialog', dialog => dialog.accept());
        await sendBtn.click();

        // 7. Verify modal closes
        await expect(modalTitle).not.toBeVisible();
    });
    test('should display multi-currency localization toggle in settings', async ({ page }) => {
        await page.goto('/settings');
        const settingsHeader = page.locator('h1', { hasText: 'Settings' });
        await expect(settingsHeader).toBeVisible();

        const globalSalesToggle = page.getByRole('checkbox', { name: 'Enable Global Sales' });
        await expect(globalSalesToggle).toBeVisible();
        await globalSalesToggle.check();
        await expect(globalSalesToggle).toBeChecked();
    });

    test('should allow creating multi-currency invoices', async ({ page }) => {
        await page.goto('/invoice-generator');
        const header = page.locator('h2', { hasText: 'Create Professional Invoice' });
        await expect(header).toBeVisible();

        const amountInput = page.locator('input[placeholder="e.g. 1500.00"]');
        await amountInput.fill('250.00');

        const currencySelect = page.locator('select');
        await currencySelect.selectOption('EUR');

        const generateBtn = page.locator('button', { hasText: 'Generate Shareable Invoice' });
        await expect(generateBtn).toBeVisible();
    });

});
