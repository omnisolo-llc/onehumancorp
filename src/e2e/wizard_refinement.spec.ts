import { test, expect } from '@playwright/test';

test.describe('Wizard Refinement E2E', () => {
  test.beforeEach(async ({ page }) => {
    const fs = require('fs');
    const path = require('path');
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const htmlContent = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: htmlContent   });
    });
    await page.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({})   });
    });
    await page.route('**/api/onboarding/draft', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({})   });
    });
  });

  test('keeps the setup flow plain-language', async ({ page }) => {
    await page.goto('http://mock/setup.html');
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();
    await expect(page.locator('text="Step-by-Step Setup"')).toBeVisible();
  });

  test.skip('exposes AI helper and prompt tuning areas', async ({ page }) => {
    // Requires backend
  });

  test.skip('settings remain accessible from dashboard quick actions', async ({ page }) => {
    // Requires backend
  });
});