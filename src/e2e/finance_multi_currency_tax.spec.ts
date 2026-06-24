import { test, expect } from './fixtures';

test.describe('Autonomous Multi-Currency Invoicing & Tax Ledger', () => {
    test('should allow owner to review a multi-currency invoice draft and view tax ledger summary', async ({ page }) => {
        // Navigate to the finance page which contains our newly implemented logic
        await page.goto('/finance');

        // 1. Verify page loads correctly
        await expect(page.locator('h1', { hasText: 'Finance & Invoicing' })).toBeVisible();

        // 2. Wait for potential network responses
        await page.waitForTimeout(2000);

        // 3. Verify Tax Liability Summary card
        const taxSummaryCard = page.locator('[data-testid="tax-summary-card"]');

        await expect(taxSummaryCard).toBeVisible({ timeout: 10000 });
        await expect(page.locator('text=Total Tax Set-Aside')).toBeVisible();
        await expect(page.locator('text=Base Currency')).toBeVisible();
        await expect(page.locator('text=USD').last()).toBeVisible();
    });
});
