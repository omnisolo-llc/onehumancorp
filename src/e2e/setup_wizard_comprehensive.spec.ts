import { test, expect } from './fixtures';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => {
      localStorage.removeItem('website-builder-storage');
      localStorage.removeItem('ohc_builder_blocks');
      localStorage.removeItem('ohc_builder_status');
    });
  });

  test('traverses the new instant build flow', async ({ page }) => {
    const id = `setup-comprehensive-${Date.now()}-${Math.random()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('ohc_wizard_state');
      localStorage.removeItem('onboarding-storage-v3');
      localStorage.removeItem('website-builder-storage');
    }, id);

    // We only have the instant build flow now.
    await page.goto('/website-builder');
    await page.waitForLoadState('networkidle');


    await page.getByRole('button', { name: /Instant Build/ }).click();

    // Verify glassmorphism style is present
    await expect(page.locator('.glassmorphism').first()).toBeVisible({ timeout: 5000 });

    await page.getByPlaceholder('e.g. I run a local bakery').fill('I run a modern art shop online');

    await page.getByRole('button', { name: /Generate Storefront/ }).click();

    await expect(page.getByText('Agents are building your store...')).toBeVisible({ timeout: 10000 });

    // Verify glassmorphism style is present on loading screen
    await expect(page.locator('.glassmorphism', { hasText: 'Agents are building your store' }).first()).toBeVisible({ timeout: 5000 });

    await expect(page.getByRole('heading', { name: /Success! Your business is live!/ })).toBeVisible({ timeout: 20000 });
  });

  test('validates empty input in Tell us about your business', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Instant Build/ }).click();

    // The textarea starts empty
    const generateBtn = page.getByRole('button', { name: /Generate Storefront/ });
    await expect(generateBtn).toBeDisabled();

    await page.getByPlaceholder('e.g. I run a local bakery').fill('A');
    await expect(generateBtn).toBeEnabled();
  });

  test('clears previous bio input when re-entering Instant Build', async ({ page }) => {
    await page.goto('/website-builder');

    // Enter instant build, fill bio, then go back
    await page.getByRole('button', { name: /Instant Build/ }).click();
    await page.getByPlaceholder('e.g. I run a local bakery').fill('Some initial input');

    // Go back to step 0
    await page.getByRole('button', { name: /Back/ }).click();

    // Re-enter Instant Build
    await page.getByRole('button', { name: /Instant Build/ }).click();

    // Bio should be cleared and button disabled
    const generateBtn = page.getByRole('button', { name: /Generate Storefront/ });
    await expect(generateBtn).toBeDisabled();
    await expect(page.getByPlaceholder('e.g. I run a local bakery')).toHaveValue('');
  });

  test('verifies Start My Business navigation is distinct from Instant Build', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Start My Business/ }).click();
    await expect(page.getByRole('heading', { name: 'What kind of business are you building?' })).toBeVisible();
    await expect(page.getByRole('button', { name: /Online Store/ })).toBeVisible();
    await expect(page.getByRole('button', { name: /Restaurant/ })).toBeVisible();
  });

  test('Instant Build gracefully handles whitespace-only bio input', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Instant Build/ }).click();

    const generateBtn = page.getByRole('button', { name: /Generate Storefront/ });
    await expect(generateBtn).toBeDisabled();

    await page.getByPlaceholder('e.g. I run a local bakery').fill('   \n  ');
    await expect(generateBtn).toBeDisabled();

    await page.getByPlaceholder('e.g. I run a local bakery').fill(' Valid input ');
    await expect(generateBtn).toBeEnabled();
  });

  test('Powered by OHC link is visible on step 0', async ({ page }) => {
    await page.goto('/website-builder');
    const poweredLink = page.getByRole('link', { name: /Powered by OHC/i });
    await expect(poweredLink).toBeVisible();
    await expect(poweredLink).toHaveAttribute('href', '/onboarding?ref=website-builder');
  });
});
