import { test, expect } from '@playwright/test';

test.describe('Viral Link-in-Bio Growth Loop', () => {
  test('should allow user to navigate to link-in-bio page and see powered by badge', async ({ page }) => {
    // 1. Go to home page which redirects to onboarding or dashboard
    await page.goto('/');

    // 2. We should be redirected to dashboard if we are seeded/logged in
    await page.waitForURL('**/dashboard');

    // 3. Find the Link-in-Bio Setup section and click the CTA
    await expect(page.getByRole('heading', { name: 'Link-in-Bio Setup' })).toBeVisible();
    await page.getByRole('link', { name: 'Create Link-in-Bio' }).click();

    // 4. Wait for the link-in-bio page to load
    await page.waitForURL('**/link-in-bio');

    // 5. Verify the Link-in-Bio page has loaded
    await expect(page.getByRole('heading', { name: 'Viral Link-in-Bio' })).toBeVisible();
    await expect(page.getByText('Share Your Business Everywhere')).toBeVisible();

    // 6. Verify the viral "Powered by OHC" footer is present in the preview
    const poweredByBadge = page.getByText(/Powered by\s*OHC/i);
    await expect(poweredByBadge).toBeVisible();

    // 7. Verify the copy link button is working
    await page.getByRole('button', { name: 'Copy Link' }).click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();
  });
});
