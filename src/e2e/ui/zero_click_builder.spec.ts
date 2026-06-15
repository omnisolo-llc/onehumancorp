import { test, expect } from '@playwright/test';
import * as crypto from 'crypto';

test.describe('Zero-Click Business Generator CUJ', () => {

  test.beforeEach(async ({ page }) => {
    // Navigate using the real server routing instead of mocks
    const tenantId = `tenant-${crypto.randomBytes(4).toString('hex')}`;
    await page.addInitScript((id) => {
      localStorage.setItem('tenant_id', id);
      localStorage.setItem('user_id', id);
    }, tenantId);

    await page.setViewportSize({ width: 375, height: 812 });
  });

  test('User can generate a business with a single prompt', async ({ page }) => {

    // Navigate to the real setup page
    await page.goto('/api/ui/setup.html');

    // Verify Initial Screen
    await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' })).toBeVisible();

    // 1. Click "Instant Build"
    await page.getByRole('button', { name: 'Instant Build' }).click();

    // 2. Verify we are in the instant step
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();

    // 3. Fill in the description
    const instantInput = page.locator('#instant-bio');
    await expect(instantInput).toBeVisible();
    await instantInput.fill('I am a home baker in Austin selling custom vegan cakes and cupcakes.');

    const generateBtn = page.getByTestId('generate-storefront-btn');
    await expect(generateBtn).toBeEnabled();

    // 4. Click generate
    await generateBtn.click();

    // 5. Wait for generation to complete and the success message to appear
    await page.waitForURL('**/success.html', { timeout: 30000 });
  });
});
