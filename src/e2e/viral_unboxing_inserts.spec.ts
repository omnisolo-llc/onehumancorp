import { test, expect } from '@playwright/test';

test.describe('Viral Unboxing Inserts Growth Loop', () => {
  test('Owner can navigate to unboxing inserts and generate a printable graphic', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');
    await expect(page.locator('h1')).toContainText('Dashboard');

    // Find and click the Unboxing Inserts card in the Growth section
    const insertsCard = page.locator('a[href="/unboxing-inserts"]');
    await expect(insertsCard).toBeVisible();
    await insertsCard.click();

    // Verify Unboxing Inserts Page loads correctly
    await expect(page).toHaveURL(/.*\/unboxing-inserts/);
    await expect(page.locator('h1')).toContainText('Printable Inserts');

    // Verify default configuration fields
    const discountAmountInput = page.locator('input').nth(0);
    const discountCodeInput = page.locator('input').nth(1);

    await expect(discountAmountInput).toHaveValue('10%');
    await expect(discountCodeInput).toHaveValue('WELCOME10');

    // Verify preview renders default values
    const previewContainer = page.locator('#insert-preview');
    await expect(previewContainer).toBeVisible();
    await expect(previewContainer).toContainText('10% OFF');
    await expect(previewContainer).toContainText('WELCOME10');
    await expect(previewContainer).toContainText('OneHumanCorp');

    // Modify the configuration
    await discountAmountInput.fill('20%');
    await discountCodeInput.fill('VIP20');

    // Verify preview updates with new values
    await expect(previewContainer).toContainText('20% OFF');
    await expect(previewContainer).toContainText('VIP20');

    // Verify print button exists
    const printButton = page.locator('button', { hasText: 'Print Insert' });
    await expect(printButton).toBeVisible();
  });
});
