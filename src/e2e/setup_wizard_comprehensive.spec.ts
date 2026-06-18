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
    const tauriUiDir = require('path').join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const htmlContent = require('fs').readFileSync(require('path').join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: htmlContent });
    });
    await page.goto('http://mock/setup.html');
    await page.waitForLoadState('networkidle');


    await page.getByRole('button', { name: /Instant Build/ }).click();

    // Verify glassmorphism style is present
    await expect(page.locator('.glassmorphism').first()).toBeVisible({ timeout: 5000 });

    await page.getByPlaceholder('e.g. I run a local bakery').fill('I run a modern art shop online');

    await page.route('**/api/onboarding/intake', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });

    await page.route('**/api/onboarding/start', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({ organization_id: 'test' }) });
    });

    await page.getByRole('button', { name: /Next/ }).click();

    // Verify dashboard redirect
    await page.route('**/dashboard.html', async route => {
      await route.fulfill({ status: 200, body: 'Success' });
    });
    await page.waitForURL('**/dashboard.html', { timeout: 20000 }).catch(() => {});
    await expect(page.url()).toContain('dashboard.html');


  });

  test('validates empty input in Tell us about your business', async ({ page }) => {
    const tauriUiDir = require('path').join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const htmlContent = require('fs').readFileSync(require('path').join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: htmlContent });
    });
    await page.goto('http://mock/setup.html');
    await page.getByRole('button', { name: /Instant Build/ }).click();

    // The textarea starts empty
    const generateBtn = page.getByRole('button', { name: /Next/ }).last();
    await page.waitForFunction(() => document.querySelector('#generate-storefront-btn')?.disabled, { timeout: 10000 }).catch(() => {});
    await expect(page.locator('#generate-storefront-btn')).toBeDisabled();

    await page.getByPlaceholder('e.g. I run a local bakery').fill('A');
    await expect(generateBtn).toBeEnabled();
  });

  test('clears previous bio input when re-entering Instant Build', async ({ page }) => {
    const tauriUiDir = require('path').join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const htmlContent = require('fs').readFileSync(require('path').join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: htmlContent });
    });
    await page.goto('http://mock/setup.html');

    // Enter instant build, fill bio, then go back
    await page.getByRole('button', { name: /Instant Build/ }).click();
    await page.getByPlaceholder('e.g. I run a local bakery').fill('Some initial input');

    // Go back to step 0
    await page.locator('button', { hasText: 'Back' }).first().click();

    // Re-enter Instant Build
    await page.getByRole('button', { name: /Instant Build/ }).click();

    // Bio should be cleared and button disabled
    const generateBtn = page.getByRole('button', { name: /Next/ }).last();
    await page.waitForFunction(() => document.querySelector('#generate-storefront-btn')?.disabled, { timeout: 10000 }).catch(() => {});
    await expect(page.locator('#generate-storefront-btn')).toBeDisabled();
    await expect(page.getByPlaceholder('e.g. I run a local bakery')).toHaveValue('');
  });

  test('verifies Start My Business navigation is distinct from Instant Build', async ({ page }) => {
    const tauriUiDir = require('path').join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const htmlContent = require('fs').readFileSync(require('path').join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: htmlContent });
    });
    await page.goto('http://mock/setup.html');

    // We start at step-initial, so we don't need to click "Back" first.
    // Just click "Start My Business" directly.
    await page.getByRole('button', { name: /Start My Business/ }).click();

    await expect(page.getByRole('heading', { name: /How do you work\?/ })).toBeVisible();
    await expect(page.locator('text="Online Creator / Tutor"').first()).toBeVisible();
    await expect(page.locator('text="Storefront or Cafe"').first()).toBeVisible();
  });

  test('Instant Build gracefully handles whitespace-only bio input', async ({ page }) => {
    const tauriUiDir = require('path').join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const htmlContent = require('fs').readFileSync(require('path').join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: htmlContent });
    });
    await page.goto('http://mock/setup.html');
    await page.getByRole('button', { name: /Instant Build/ }).click();

    const generateBtn = page.getByRole('button', { name: /Next/ }).last();
    await page.waitForFunction(() => document.querySelector('#generate-storefront-btn')?.disabled, { timeout: 10000 }).catch(() => {});
    await expect(page.locator('#generate-storefront-btn')).toBeDisabled();

    await page.getByPlaceholder('e.g. I run a local bakery').fill('   \n  ');
    await expect(page.locator('#generate-storefront-btn')).toBeDisabled();

    await page.getByPlaceholder('e.g. I run a local bakery').fill(' Valid input ');
    await expect(generateBtn).toBeEnabled();
  });

  test('Powered by OHC link is visible on step 0', async ({ page }) => {
    const tauriUiDir = require('path').join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const htmlContent = require('fs').readFileSync(require('path').join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: htmlContent });
    });
    await page.goto('http://mock/setup.html');
    const poweredLink = page.getByRole('link', { name: /Powered by OHC/i });
    await expect(poweredLink).toBeVisible();
    await expect(poweredLink).toHaveAttribute('href', '/setup.html?ref=website-builder');
  });
});
