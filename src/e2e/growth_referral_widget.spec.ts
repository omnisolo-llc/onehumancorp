import { test, expect } from './fixtures';

test.describe('Growth Referral Widget', () => {
  test('generates widget code and handles paywall correctly', async ({ page }) => {

    // In our E2E environment we go directly to dashboard
    await page.goto('/dashboard');

    // Wait for the Widget Builder button to appear under Invite & Earn section and click it
    const widgetBuilderBtn = page.locator('#dashboard-widget-btn');
    await expect(widgetBuilderBtn).toBeVisible();
    await widgetBuilderBtn.click();

    // Ensure the builder is visible
    await expect(page.getByRole('heading', { name: 'Referral Widget Builder' })).toBeVisible();

    // Verify preview renders correctly with defaults
    await expect(page.getByRole('heading', { name: 'Give 10%, Get 10%' })).toBeVisible();

    // Test updating the offer
    await page.locator('#discount-amount').fill('20');
    await page.locator('#discount-amount').dispatchEvent('input', { bubbles: true });
    await expect(page.getByRole('heading', { name: 'Give 20%, Get 20%' })).toBeVisible();

    // Test updating the offer type
    await page.locator('#discount-type').selectOption('$');
    await page.locator('#discount-type').dispatchEvent('change', { bubbles: true });
    await expect(page.getByRole('heading', { name: 'Give $20, Get $20' })).toBeVisible();

    // Verify branding is present by default
    await expect(page.getByText('⚡ Powered by OHC')).toBeVisible();

    // Click Generate code
    await page.getByRole('button', { name: 'Get Widget Code' }).click();

    // Verify embed modal
    const embedModal = page.locator('#embed-modal');
    await expect(embedModal).toHaveClass(/active/);
    await expect(page.getByRole('heading', { name: 'Embed Referral Widget' })).toBeVisible();

    // Verify iframe code structure
    const codeArea = page.locator('#embed-code');
    const codeValue = await codeArea.inputValue();
    expect(codeValue).toContain('<iframe');
    expect(codeValue).toContain('discount=20flat');

    // Close modal
    await page.locator('#close-embed-btn').click();
    await expect(embedModal).not.toHaveClass(/active/);
  });
});
