import { test, expect } from '@playwright/test';
import { aiJudge } from './ai-judge';
import { E2E_ADMIN_USER } from './fixtures';

test.describe('Agentic Smart Pricing Engine', () => {
    test('detects stagnant stock and allows owner to approve discount', async ({ page }) => {
        // E2E test without using the failing adminPage fixture, we log in manually.
        await page.route('**/*', route => route.continue()); await page.goto('/login');
        await page.fill('input[type="email"]', E2E_ADMIN_USER.email);
        await page.fill('input[type="password"]', E2E_ADMIN_USER.password);
        await page.click('button[type="submit"]');

        await page.waitForURL('/dashboard');

        // Wait for the Agent Feed to load
        await page.waitForSelector('[aria-label="Unified Agent Feed"]');

        // Look for the action card with our "Smart Price Suggestion"
        await expect(page.locator('text=Smart Price Suggestion: Vintage Sweater')).toBeVisible({ timeout: 15000 });

        // Verify UI elements of the card
        await expect(page.locator('text=Current Price:')).toBeVisible();
        await expect(page.locator('text=$40')).toBeVisible();
        await expect(page.locator('text=Suggested: $34.00')).toBeVisible();

        // Click the approve button
        const approveButton = page.locator('button[aria-label="Approve proposal"]:has-text("Approve & Run Sale")');
        await expect(approveButton).toBeVisible();
        await approveButton.click();

        // The card should disappear after approval optimistic UI update
        await expect(page.locator('text=Smart Price Suggestion: Vintage Sweater')).not.toBeVisible();
    });
});
