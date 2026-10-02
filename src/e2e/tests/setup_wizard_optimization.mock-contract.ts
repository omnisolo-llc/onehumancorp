import * as nativeFsModule from 'node:fs';
import * as nativePathModule from 'node:path';
import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Optimization', () => {
  test.beforeEach(async ({ page }) => {
    const fs = nativeFsModule;
    const path = nativePathModule;
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });
    await page.goto('http://mock/setup.html');
  });

  test('validates domain name correctly', async ({ page }) => {
    // Wait for the scripts to load
    await page.waitForFunction(() => 'goToStep' in window && typeof window.goToStep === 'function');

    await page.evaluate(() => { if (!('goToStep' in window) || typeof window.goToStep !== 'function') throw new Error('Wizard navigation is unavailable'); window.goToStep('step-domain'); });
    const domainInput = page.locator('#domain-name');

    await domainInput.fill('invalid_domain!');
    await page.locator('#step-domain .next-step-btn').click();

    const isValid = await page.evaluate(() => { if (!('validateStep' in window) || typeof window.validateStep !== 'function') throw new Error('Wizard validation is unavailable'); const result: unknown = window.validateStep('step-domain'); if (typeof result !== 'boolean') throw new Error('Wizard validation returned an invalid result'); return result; });
    expect(isValid).toBe(false);

    await expect(page.locator('#domain-error')).toBeVisible();
    await expect(page.locator('#domain-error')).toContainText('contain only lowercase letters');

    await domainInput.fill('-invalid-leading-hyphen');
    await page.locator('#step-domain .next-step-btn').click();
    await expect(page.locator('#domain-error')).toBeVisible();
    await expect(page.locator('#domain-error')).toContainText('cannot start or end with a hyphen');

    await domainInput.fill('invalid-trailing-hyphen-');
    await page.locator('#step-domain .next-step-btn').click();
    await expect(page.locator('#domain-error')).toBeVisible();
    await expect(page.locator('#domain-error')).toContainText('cannot start or end with a hyphen');

    await domainInput.fill('valid-domain-123');
    await page.locator('#step-domain .next-step-btn').click();

    await expect(page.locator('#step-template')).toHaveClass(/step active/);
  });

  test('validates domain name visual structure properly', async ({ page }) => {
    await page.waitForFunction(() => 'goToStep' in window && typeof window.goToStep === 'function');
    await page.evaluate(() => { if (!('goToStep' in window) || typeof window.goToStep !== 'function') throw new Error('Wizard navigation is unavailable'); window.goToStep('step-domain'); });
    const domainInputContainer = page.locator('#step-domain .glass-control.glassmorphism').first();
    const spanSuffix = domainInputContainer.locator('span');

    await expect(spanSuffix).toBeVisible();
    await expect(spanSuffix).toHaveText('.cloud.omnisolo.co');
  });

  test('validates domain name min length correctly', async ({ page }) => {
    await page.waitForFunction(() => 'goToStep' in window && typeof window.goToStep === 'function');
    await page.evaluate(() => { if (!('goToStep' in window) || typeof window.goToStep !== 'function') throw new Error('Wizard navigation is unavailable'); window.goToStep('step-domain'); });
    const domainInput = page.locator('#domain-name');

    await domainInput.fill('ab');
    await page.locator('#step-domain .next-step-btn').click();

    await expect(page.locator('#domain-error')).toBeVisible();
  });

  test('validates domain error goes away', async ({ page }) => {
    await page.waitForFunction(() => 'goToStep' in window && typeof window.goToStep === 'function');
    await page.evaluate(() => { if (!('goToStep' in window) || typeof window.goToStep !== 'function') throw new Error('Wizard navigation is unavailable'); window.goToStep('step-domain'); });
    const domainInput = page.locator('#domain-name');

    await domainInput.fill('ab');
    await page.locator('#step-domain .next-step-btn').click();
    await expect(page.locator('#domain-error')).toBeVisible();

    await domainInput.fill('valid-domain');
    await page.locator('#step-domain .next-step-btn').click();
    await expect(page.locator('#domain-error')).toBeHidden();
  });

  test('validates domain name does not accept special chars', async ({ page }) => {
    await page.waitForFunction(() => 'goToStep' in window && typeof window.goToStep === 'function');
    await page.evaluate(() => { if (!('goToStep' in window) || typeof window.goToStep !== 'function') throw new Error('Wizard navigation is unavailable'); window.goToStep('step-domain'); });
    const domainInput = page.locator('#domain-name');

    await domainInput.fill('test domain');
    await page.locator('#step-domain .next-step-btn').click();

    await expect(page.locator('#domain-error')).toBeVisible();
  });

  test('validates domain error goes away dynamically', async ({ page }) => {
    await page.waitForFunction(() => 'goToStep' in window && typeof window.goToStep === 'function');
    await page.evaluate(() => { if (!('goToStep' in window) || typeof window.goToStep !== 'function') throw new Error('Wizard navigation is unavailable'); window.goToStep('step-domain'); });
    const domainInput = page.locator('#domain-name');

    await domainInput.fill('ab');
    await page.locator('#step-domain .next-step-btn').click();
    await expect(page.locator('#domain-error')).toBeVisible();

    await domainInput.fill('valid-domain');
    // We should not need to click next, the error should be hidden
    await expect(page.locator('#domain-error')).toBeHidden();
  });
});
