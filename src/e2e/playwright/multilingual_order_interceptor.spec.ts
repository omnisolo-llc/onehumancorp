import { test, expect } from '@playwright/test';

test.describe('Multilingual Order Interceptor CUJ', () => {
    test('Fatima receives a Spanish walk-up order and it appears as a structured English task', async ({ page }) => {
        // Mock the JWT token logic or use a test tenant
        await page.goto('/multilingual-order-interceptor?test_tenant=true');

        // Verify UI loads
        await expect(page.locator('h1', { hasText: 'Walk-up Order' })).toBeVisible();

        // Simulate Fatima tapping the mic and the customer saying "Quiero 3 tacos de pollo"
        // Since we mocked this to happen via the button or typing in the textarea:
        await page.fill('textarea', 'Quiero 3 tacos de pollo');
        await page.click('button:has-text("Process Text Order")');

        // Wait for the LLM to process and return structured data
        // For the e2e test without a mock, if it hits Minimax it might be slow, so give it some time
        await expect(page.locator('text=Order')).toBeVisible({ timeout: 15000 });

        // Match the translated intent depending on LLM output (usually "chicken tacos" or "Chicken Tacos")
        await expect(page.locator('text=chicken')).toBeVisible({ ignoreCase: true });
        await expect(page.locator('text=x3')).toBeVisible();

        // Confirm the order to send it to the agent feed
        // Handle the native dialog before clicking
        page.on('dialog', dialog => dialog.accept());
        await page.click('button:has-text("Confirm & Add to List")');

        // Wait for the form to reset
        await expect(page.locator('textarea')).toBeEmpty({ timeout: 5000 });
    });
});
