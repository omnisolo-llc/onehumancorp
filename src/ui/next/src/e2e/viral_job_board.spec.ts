import { test, expect } from '@playwright/test';

test.describe('Viral Job Board Generator', () => {
  test('should render the generator and preview correctly', async ({ page }) => {
    await page.goto('/viral-job-board-generator');

    // Check title
    await expect(page.locator('h1')).toContainText('Viral Job Board Generator');

    // Check default input values
    const titleInput = page.locator('input[placeholder="e.g. We are hiring!"]');
    await expect(titleInput).toHaveValue('We are hiring!');

    // Modify inputs and check preview updates
    await titleInput.fill('Join Our Startup');
    await expect(page.locator('.shadow-\[0_8px_30px_rgb\(0\,0\,0\,0\.1\)\] h2')).toContainText('Join Our Startup');

    // Check referral block in preview
    await expect(page.locator('.shadow-\[0_8px_30px_rgb\(0\,0\,0\,0\.1\)\] .bg-green-50')).toContainText('Refer a friend');
  });
});

test.describe('Viral Job Board Generator tests', () => {
  test('should clear the input fields properly', async ({ page }) => {
    await page.goto('/viral-job-board-generator');

    const titleInput = page.locator('input[placeholder="e.g. We are hiring!"]');
    await titleInput.fill('');
    await expect(page.locator('.shadow-\[0_8px_30px_rgb\(0\,0\,0\,0\.1\)\] h2')).toContainText('We are hiring!');
  });

  test('should handle description changes', async ({ page }) => {
    await page.goto('/viral-job-board-generator');

    const descInput = page.locator('textarea[placeholder="e.g. Join our team and help us build the future."]');
    await descInput.fill('Join our amazing company');
    await expect(page.locator('.shadow-\[0_8px_30px_rgb\(0\,0\,0\,0\.1\)\] p').first()).toContainText('Join our amazing company');
  });

  test('should handle empty description', async ({ page }) => {
    await page.goto('/viral-job-board-generator');

    const descInput = page.locator('textarea[placeholder="e.g. Join our team and help us build the future."]');
    await descInput.fill('');
    await expect(page.locator('.shadow-\[0_8px_30px_rgb\(0\,0\,0\,0\.1\)\] p').first()).toContainText('Join our team.');
  });

  test('should toggle theme correctly', async ({ page }) => {
    await page.goto('/viral-job-board-generator');

    await page.locator('button:has-text("Dark")').click();
    await expect(page.locator('.shadow-\[0_8px_30px_rgb\(0\,0\,0\,0\.1\)\]')).toHaveCSS('background-color', 'rgb(17, 24, 39)');

    await page.locator('button:has-text("Light")').click();
    await expect(page.locator('.shadow-\[0_8px_30px_rgb\(0\,0\,0\,0\.1\)\]')).toHaveCSS('background-color', 'rgb(255, 255, 255)');
  });

  test('should verify empty fields default to expected text in preview', async ({ page }) => {
    await page.goto('/viral-job-board-generator');

    const titleInput = page.locator('input[placeholder="e.g. We are hiring!"]');
    await titleInput.fill('');
    await expect(page.locator('.shadow-\\[0_8px_30px_rgb\\(0\\,0\\,0\\,0\\.1\\)\\] h2')).toContainText('We are hiring!');

    const descInput = page.locator('textarea[placeholder="e.g. Join our team and help us build the future."]');
    await descInput.fill('');
    await expect(page.locator('.shadow-\\[0_8px_30px_rgb\\(0\\,0\\,0\\,0\\.1\\)\\] p').first()).toContainText('Join our team.');
  });

  test('should show paywall when attempting to remove branding without pro', async ({ page }) => {
    // 1. Ensure non-pro for this run
    await page.evaluate(() => {
        localStorage.setItem('has_pro', 'false');
    });

    await page.goto('/viral-job-board-generator');

    const removeBrandingCheckbox = page.locator('#removeBranding');
    await removeBrandingCheckbox.check();

    // Soft paywall should appear
    const paywallModal = page.locator('h2', { hasText: 'Upgrade to Remove Branding' });
    await expect(paywallModal).toBeVisible();
  });

  test('should hide branding when PRO feature is used', async ({ page }) => {
    // 1. Ensure pro for this run
    await page.evaluate(() => {
        localStorage.setItem('has_pro', 'true');
    });

    await page.goto('/viral-job-board-generator');

    const removeBrandingCheckbox = page.locator('#removeBranding');
    await removeBrandingCheckbox.check();

    const paywallModal = page.locator('h2', { hasText: 'Upgrade to Remove Branding' });
    await expect(paywallModal).not.toBeVisible();

    // Branding should not be visible in preview
    const branding = page.locator('span', { hasText: '⚡ Powered by OHC' });
    await expect(branding).not.toBeVisible();
  });

});
