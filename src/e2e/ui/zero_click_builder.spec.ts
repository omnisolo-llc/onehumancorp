import { test, expect } from '@playwright/test';
import * as crypto from 'crypto';

test.describe('Zero-Click Business Generator CUJ', () => {

  test.beforeEach(async ({ page }) => {
    // Generate a valid tenant identity via the standard authentication route
    const tenantId = `tenant-${crypto.randomBytes(4).toString('hex')}`;
    const agentId = `owner-${crypto.randomBytes(4).toString('hex')}@test.com`;

    // Go to login to obtain proper session/token auth context per repo guidelines
    await page.goto('/login');
    await page.getByPlaceholder('Email address').fill(agentId);
    await page.getByPlaceholder('Password').fill('Password123!');
    await page.getByRole('button', { name: 'Sign in' }).click();

    await expect(page).toHaveURL('/dashboard');
  });

  test('User can generate a business with a single prompt', async ({ page }) => {
    // Navigate to the real web app path
    await page.goto('/zero-click-builder');

    // Verify Initial Screen
    await expect(page.getByText('Zero-Click Business Generator')).toBeVisible();

    // 1. Fill in the prompt
    const promptInput = page.getByPlaceholder('e.g. I am a home baker in Austin selling custom vegan cakes.');
    await expect(promptInput).toBeVisible();
    await promptInput.fill('I am a home baker in Austin selling custom vegan cakes.');

    const generateBtn = page.getByRole('button');
    await expect(generateBtn).toBeEnabled();

    // 2. Click generate
    await generateBtn.click();

    // 3. Verify loading overlay
    await expect(page.getByText('Building Your Business...')).toBeVisible();

    // 4. Wait for generation to complete and the success state to appear
    await expect(page.getByText('Your business is live!')).toBeVisible({ timeout: 20000 });
    await expect(page.locator('iframe')).toBeVisible();
  });
});
