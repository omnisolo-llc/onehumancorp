import { test, expect } from './fixtures';

test.describe('Instant Build Flow E2E', () => {
  test.beforeEach(async ({ page }) => {
    const id = `instant-build-${Date.now()}-${Math.random()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('ohc_wizard_state');
      localStorage.removeItem('onboarding-storage-v3');
    }, id);
  });

  test('completes the instant build path successfully', async ({ page }) => {
    await page.goto('/website-builder');
    await page.waitForLoadState('networkidle');
    await expect(page.locator('#setup-screen')).toBeVisible();

    // Start instant build
    await page.getByRole('button', { name: /Instant Build/ }).click();

    // Verify we are on the Instant Build step
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();

    // Fill bio
    await page.getByPlaceholder('e.g. I run a local bakery').fill('I run an amazing online custom cake shop for dogs.');

    // Generate
    await page.getByRole('button', { name: 'Generate Storefront' }).click();

    // Wait for the final success screen
    await expect(page.getByRole('heading', { name: 'Success! Your business is live!' })).toBeVisible({ timeout: 15000 });
  });
});
