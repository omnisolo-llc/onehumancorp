import { test, expect } from '@playwright/test';

test.describe('Zero-Click Onboarding', () => {
  test('should generate business successfully and redirect to dashboard', async ({ page }) => {
    // Navigate to the onboarding zero-click page
    await page.goto('http://localhost:3000/onboarding/zero-click');

    // Make sure the title is visible
    await expect(page.getByRole('heading', { name: 'Zero-Click Setup' })).toBeVisible();

    // Type the prompt into the textarea
    const textarea = page.locator('textarea');
    await textarea.fill('I sell custom vegan cakes in Austin.');

    // Click the Generate button
    const generateBtn = page.getByRole('button', { name: 'Generate Business' });
    await generateBtn.click();

    // Verify loading state appears
    await expect(page.getByRole('heading', { name: 'Generating your business...' })).toBeVisible();

    // Wait for the redirect to dashboard (increased timeout to allow for LLM and DB processing)
    await page.waitForURL(/\/dashboard\?zero_click_success=true/, { timeout: 60000 });

    // Verify success banner appears on the dashboard
    await expect(page.getByText('Generation Complete!')).toBeVisible();
    await expect(page.getByText('The Operations Agent has set up your initial product catalog based on your description.')).toBeVisible();
  });
});
