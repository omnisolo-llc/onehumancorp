import { test, expect } from '@playwright/test';

test.describe('Operations Manager Flow', () => {
  test('User can use Operations Manager to add an item', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('http://127.0.0.1:3000/dashboard');

    // Check if the dashboard is loaded
    await expect(page.locator('text=Items').first()).toBeVisible();

    // Find the actual button using evaluate since it's nested
    await page.evaluate(() => {
        const spans = Array.from(document.querySelectorAll('span'));
        const target = spans.find(b => b.textContent && b.textContent.includes('Ask Operations Manager'));
        if(target && target.parentElement) {
            target.parentElement.click();
        }
    });

    // Operations Manager Modal should be visible
    const modalHeader = page.locator('h2', { hasText: 'Operations Manager' }).first();
    await expect(modalHeader).toBeVisible({ timeout: 10000 });

    // Type a prompt
    const textarea = page.locator('textarea');
    await expect(textarea).toBeVisible();
    await textarea.fill('Add dozen vanilla cupcakes for $24');

    // Click submit
    await page.evaluate(() => {
        const btn = Array.from(document.querySelectorAll('button')).find(b => b.textContent === 'Submit');
        if(btn) btn.click();
    });

    // Wait for the preview card to appear
    await expect(page.locator('text=Vanilla Cupcakes (Dozen)').first()).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=$24').first()).toBeVisible();

    // Approve the new product
    await page.evaluate(() => {
        const btn = Array.from(document.querySelectorAll('button')).find(b => b.textContent === 'Approve');
        if(btn) btn.click();
    });

    // Success view should appear
    await expect(page.locator('text=Item Added!').first()).toBeVisible({ timeout: 10000 });

    // Wait for modal to close automatically
    await expect(modalHeader).toBeHidden({ timeout: 10000 });
  });
});
