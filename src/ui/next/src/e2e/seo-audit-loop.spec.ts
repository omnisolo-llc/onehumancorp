import { test, expect } from '@playwright/test';

test.describe('Storefront SEO Health Report Growth Loop', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('http://localhost:3000/dashboard');
    await page.waitForLoadState('networkidle');
  });

  test('should display the SEO health report and handle soft paywall', async ({ page }) => {
    // 1. Verify the SEO Health Report section is visible
    const seoHeading = page.getByRole('heading', { name: 'Storefront SEO Health Report' });
    await expect(seoHeading).toBeVisible();

    // Verify the "Growth Loop" badge is present
    await expect(page.locator('text=Growth Loop').filter({ hasText: 'Growth Loop' }).first()).toBeVisible();

    // 2. Click "Generate AI SEO Fixes"
    await page.getByRole('button', { name: 'Generate AI SEO Fixes' }).click();

    // 3. Verify the modal opens and shows generating state
    const modalHeading = page.getByRole('heading', { name: '📈 AI SEO Audit' });
    await expect(modalHeading).toBeVisible();
    await expect(page.getByText('Analyzing your storefront...')).toBeVisible();

    // 4. Wait for generation to complete and verify report content
    await expect(page.getByText('Analyzing your storefront...')).toBeHidden({ timeout: 10000 });

    const reportPre = page.locator('pre');
    await expect(reportPre).toBeVisible();
    await expect(reportPre).toContainText('SEO Audit Report');
    await expect(reportPre).toContainText('Powered by OHC');

    // 5. Verify the Soft Paywall Upsell CTA is present
    await expect(page.getByRole('heading', { name: 'Automate with Pro' })).toBeVisible();

    // 6. Click "Upgrade to Pro"
    await page.getByRole('button', { name: 'Upgrade to Pro' }).click();

    // 7. Verify the upgrade modal is triggered
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).toBeVisible();
  });
});
