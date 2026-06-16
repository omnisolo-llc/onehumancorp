import { test, expect } from '@playwright/test';

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
    await page.goto('/onboarding');
    await page.waitForLoadState('networkidle');




    // Verify glassmorphism style is present
    await expect(page.locator('.glassmorphism').first()).toBeVisible({ timeout: 5000 });

    await page.getByPlaceholder('e.g. I run a local bakery').fill('I run a modern art shop online');
    await page.getByTestId('admin-email').fill('maya@example.com');
    await page.getByTestId('admin-password').fill('mypassword123');

    await page.getByRole('button', { name: /Next/ }).click();

    await expect(page.getByText('Building Your Business...')).toBeVisible({ timeout: 10000 });

    // Verify glassmorphism style is present on loading screen
    await expect(page.locator('.glassmorphism', { hasText: 'Building Your Business' }).first()).toBeVisible({ timeout: 5000 });

    await expect(page.getByRole('heading', { name: /You're Live!/ })).toBeVisible({ timeout: 20000 });
  });

  test('validates empty input in Tell us about your business', async ({ page }) => {
    await page.goto('/onboarding');


    // The textarea starts empty
    const generateBtn = page.getByRole('button', { name: /Next/ });
    await expect(generateBtn).toBeDisabled();

    await page.getByTestId('admin-email').fill('maya@example.com');
    await page.getByTestId('admin-password').fill('mypassword123');
    await page.getByPlaceholder('e.g. I run a local bakery').fill('A');
    await expect(generateBtn).toBeEnabled();
  });


  test('Instant Build gracefully handles whitespace-only bio input', async ({ page }) => {
    await page.goto('/onboarding');


    const generateBtn = page.getByRole('button', { name: /Next/ });
    await expect(generateBtn).toBeDisabled();

    await page.getByTestId('admin-email').fill('maya@example.com');
    await page.getByTestId('admin-password').fill('mypassword123');
    await page.getByPlaceholder('e.g. I run a local bakery').fill('   \n  ');
    await expect(generateBtn).toBeDisabled();

    await page.getByPlaceholder('e.g. I run a local bakery').fill(' Valid input ');
    await expect(generateBtn).toBeEnabled();
  });

  test('Powered by OHC link is visible on step 0', async ({ page }) => {
    await page.goto('/onboarding');
    const poweredLink = page.getByRole('link', { name: /Powered by OHC/i });
    await expect(poweredLink).toBeVisible();
    await expect(poweredLink).toHaveAttribute('href', '/onboarding?ref=website-builder');
  });
});
