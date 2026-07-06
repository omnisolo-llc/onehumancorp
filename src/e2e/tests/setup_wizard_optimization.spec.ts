import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Optimization', () => {
  test.beforeEach(async ({ page }) => {
    const fs = require('fs');
    const path = require('path');
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });
    await page.goto('http://mock/setup.html');
  });

  test('validates domain name correctly', async ({ page }) => {
    // Wait for the scripts to load
    await page.waitForFunction(() => window.goToStep !== undefined);

    await page.evaluate(() => { window.goToStep('step-domain'); });
    const domainInput = page.locator('#domain-name');

    await domainInput.fill('invalid_domain!');
    await page.locator('#step-domain .next-step-btn').click();

    const isValid = await page.evaluate(() => window.validateStep('step-domain'));
    expect(isValid).toBe(false);

    await expect(page.locator('#domain-error')).toBeVisible();
    await expect(page.locator('#domain-error')).toContainText('contain only lowercase letters');

    await domainInput.fill('valid-domain-123');
    await page.locator('#step-domain .next-step-btn').click();

    await expect(page.locator('#step-template')).toHaveClass(/step active/);
  });

  test('validates domain name visual structure properly', async ({ page }) => {
    await page.waitForFunction(() => window.goToStep !== undefined);
    await page.evaluate(() => { window.goToStep('step-domain'); });
    const domainInputContainer = page.locator('#step-domain .glass-control.glassmorphism').first();
    const spanSuffix = domainInputContainer.locator('span');

    await expect(spanSuffix).toBeVisible();
    await expect(spanSuffix).toHaveText('.ohc.app');
  });

  test('validates domain name min length correctly', async ({ page }) => {
    await page.waitForFunction(() => window.goToStep !== undefined);
    await page.evaluate(() => { window.goToStep('step-domain'); });
    const domainInput = page.locator('#domain-name');

    await domainInput.fill('ab');
    await page.locator('#step-domain .next-step-btn').click();

    await expect(page.locator('#domain-error')).toBeVisible();
  });

  test('validates domain error goes away', async ({ page }) => {
    await page.waitForFunction(() => window.goToStep !== undefined);
    await page.evaluate(() => { window.goToStep('step-domain'); });
    const domainInput = page.locator('#domain-name');

    await domainInput.fill('ab');
    await page.locator('#step-domain .next-step-btn').click();
    await expect(page.locator('#domain-error')).toBeVisible();

    await domainInput.fill('valid-domain');
    await page.locator('#step-domain .next-step-btn').click();
    await expect(page.locator('#domain-error')).toBeHidden();
  });

  test('validates domain name does not accept special chars', async ({ page }) => {
    await page.waitForFunction(() => window.goToStep !== undefined);
    await page.evaluate(() => { window.goToStep('step-domain'); });
    const domainInput = page.locator('#domain-name');

    await domainInput.fill('test domain');
    await page.locator('#step-domain .next-step-btn').click();

    await expect(page.locator('#domain-error')).toBeVisible();
  });
});
