import { test, expect } from './playwright/fixtures';

test.describe('Multilingual Order Interceptor CUJ', () => {
    test.beforeEach(async ({ page, context }) => {
        await context.clearCookies();
        await page.goto('/multilingual-order-interceptor');
        await page.evaluate(() => localStorage.clear());
        await page.reload();
    });

    test('Fatima receives a Spanish walk-up order and it appears as a structured English task', async ({ page }) => {
        // The E2E tests will hit the real backend which will either hit minimax, gemini, or the local mock LLM
        // depending on the environment variables defined in global setup.

        await expect(page.locator('h1', { hasText: 'Walk-up Order' })).toBeVisible();

        await page.fill('textarea', 'Quiero 3 tacos de pollo');
        await page.click('button:has-text("Process Text Order")');

        await expect(page.locator('text=Order')).toBeVisible({ timeout: 15000 });
        await expect(page.locator('text=chicken')).toBeVisible({ ignoreCase: true });
        await expect(page.locator('text=x3')).toBeVisible();

        page.on('dialog', dialog => dialog.accept());
        await page.click('button:has-text("Confirm & Add to List")');

        await expect(page.locator('textarea')).toBeEmpty({ timeout: 5000 });
    });

    test('Shows validation error when processing empty order', async ({ page }) => {
        await expect(page.locator('h1', { hasText: 'Walk-up Order' })).toBeVisible();
        await expect(page.locator('button:has-text("Process Text Order")')).not.toBeVisible(); // Button should not be visible if textarea is empty
    });

    test('Allows cancellation of structured order', async ({ page }) => {
        await expect(page.locator('h1', { hasText: 'Walk-up Order' })).toBeVisible();

        await page.fill('textarea', 'Quiero 3 tacos de pollo');
        await page.click('button:has-text("Process Text Order")');

        await expect(page.locator('text=Order')).toBeVisible({ timeout: 15000 });

        await page.click('button:has-text("Cancel")');
        await expect(page.locator('textarea')).toBeEmpty();
    });

    test('Voice transcription disabled button acts correctly', async ({ page }) => {
        await expect(page.locator('button[aria-label="Voice transcription unavailable"]')).toBeDisabled();
        await expect(page.locator('text=Voice transcription is unavailable')).toBeVisible();
    });
});
