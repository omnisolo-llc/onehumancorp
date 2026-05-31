import { test, expect } from './fixtures';

test.describe('CUJ: Billing Cost Tracking', () => {
  test.skip('Persona: Business Owner uses Cost Tracking test('Persona: Business Owner uses Cost Tracking & Plan upgrades successfully', async ({ page }) => { Plan upgrades successfully', async ({ page }) => {
    // 1. Owner opens the My Plan page
    await page.goto('/plan');

    // 2. Verify 'My Plan' heading is visible
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();

    // 3. Verify that 'Current Plan', 'Estimated Next Bill', 'AI Actions Used', and 'Storage Used' sections are visible.
    await expect(page.getByText('Current Plan')).toBeVisible();
    await expect(page.getByText('Estimated Next Bill')).toBeVisible();
    await expect(page.getByText('AI Actions Used')).toBeVisible();
    await expect(page.getByText('Storage Used')).toBeVisible();

    // 4. Click 'View Cost Details' button to navigate to /cost-dashboard
    await page.getByRole('button', { name: /View Cost Details/i }).click();

    // 5. Verify /cost-dashboard loads correctly
    await expect(page.getByRole('heading', { name: 'Cost Transparency' })).toBeVisible();

    // 6. Verify cost breakdown elements are visible
    await expect(page.getByText('Total Costs')).toBeVisible();
    await expect(page.getByText('LLM Usage')).toBeVisible();
    await expect(page.getByText('Storage', { exact: true })).toBeVisible();
    await expect(page.getByText('Payment Fees')).toBeVisible();

    // 7. Click 'Back to My Plan' to return to /plan
    await page.getByRole('button', { name: /Back to My Plan/i }).click();
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();

    // 8. Click 'View Upgrade Plans' to navigate to /pricing
    await page.getByRole('button', { name: /View Upgrade Plans/i }).click();

    // 9. Verify /pricing loads correctly
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();

    // 10. Verify pricing tiers are visible
    await expect(page.getByRole('heading', { name: 'Free' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Starter' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pro' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Business', exact: true })).toBeVisible();
  });
});
