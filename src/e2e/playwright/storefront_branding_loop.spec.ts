import { test, expect } from '../fixtures';

test.describe('Storefront Branding Loop E2E', () => {
  test('verify storefront footer viral link contains correct referral', async ({ page }) => {
    // Navigate and login
    await page.goto('/login');
    await page.getByPlaceholder('Email').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Sign in' }).click();

    // Wait for auth to complete and land on dashboard
    await page.waitForURL('**/dashboard**');
    await page.waitForLoadState('networkidle');

    // Go to storefront preview page
    await page.goto('/storefront.html');
    await page.waitForLoadState('networkidle');

    // Check if the viral footer link exists
    const footerLink = page.getByTestId('storefront-footer-viral-link');
    await expect(footerLink).toBeVisible({ timeout: 15000 });

    // Check if the text matches
    await expect(footerLink).toContainText('⚡ Powered by OHC');

    // Check if it has the correct referral URL using the seeded tenant 'e2e-tenant'
    const href = await footerLink.getAttribute('href');
    expect(href).toContain('ref=e2e-tenant');
    expect(href).toContain('source=storefront_footer');
  });
});
