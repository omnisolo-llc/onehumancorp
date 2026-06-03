import { test, expect } from '@playwright/test';

test.describe('Sales Acquisition Auto-Quote & Book', () => {
  test('should generate quote successfully', async ({ page }) => {
    // Navigate to the auto quote page
    await page.goto('/sales-acquisition');

    // Verify header is present
    await expect(page.getByRole('heading', { name: 'Auto-Quote & Book Dashboard' })).toBeVisible();

    // Verify simulate section is visible
    await expect(page.getByText('Customer Inquiry Simulator')).toBeVisible();

    // Fill the inquiry message
    await page.fill('textarea[placeholder="e.g. I need help fixing a broken pipe in my bathroom..."]', 'I need help installing a new sink');

    // Click generate button
    const generateBtn = page.getByRole('button', { name: 'Generate Quote' });
    await expect(generateBtn).toBeEnabled();
    await generateBtn.click();

    // Verify "Generating..." state
    await expect(page.getByRole('button', { name: 'Generating...' })).toBeVisible();

    // Wait for output to appear
    await expect(page.getByText('AI Salesperson Response')).toBeVisible({ timeout: 5000 });

    // Verify output text (approximate match on expected template content)
    await expect(page.getByText('Based on your description, we recommend the following service')).toBeVisible();
    await expect(page.getByText('Service: Handyman Repair')).toBeVisible();
  });
});
