import { test, expect } from './fixtures';

test.describe('Cost Dashboard', () => {
  test('should display cost dashboard with correct elements and style', async ({ page }) => {
    await page.goto('/cost-dashboard');

    // Check main heading
    await expect(page.getByRole('heading', { name: 'Business Advisory Dashboard' })).toBeVisible();

    // Check main sections
    await expect(page.getByRole('heading', { name: 'Advisory Summary' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Cost Transparency' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Cost Breakdown' })).toBeVisible();

    // Check specific cost breakdown elements
    await expect(page.getByText('LLM Usage')).toBeVisible();
    await expect(page.locator('text=Storage').first()).toBeVisible();
    await expect(page.getByText('Payment Fees')).toBeVisible();

    // Ensure no jargon
    await expect(page.getByText('database storage')).not.toBeVisible();
    await expect(page.getByText('API error')).not.toBeVisible();

    // Check style/glassmorphism (basic check)
    const header = page.locator('header').first();
    const style = await header.evaluate((el) => {
        const computed = window.getComputedStyle(el);
        return {
            backdropFilter: computed.backdropFilter || computed.webkitBackdropFilter,
        };
    });
    if (style.backdropFilter && style.backdropFilter !== 'none') {
        expect(style.backdropFilter).toContain('blur(30px)');
    }
  });

  test('should redirect back to plan', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await page.getByRole('button', { name: 'Back to My Plan' }).click();
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();
  });
});
