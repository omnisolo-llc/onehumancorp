import { test, expect } from './fixtures';
import * as fs from 'fs';
import * as path from 'path';

test.describe('Zero-Click Onboarding Agent E2E', () => {

  test.beforeEach(async ({ page }) => {
    // Intercept zero-click-builder.html load to serve from filesystem for tests
    let tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    if (!fs.existsSync(tauriUiDir)) {
        tauriUiDir = path.join(process.env.RUNFILES_DIR || process.cwd(), '_main/src/ui/tauri/src/ui');
    }
    await page.route('**/zero-click-builder.html', async route => {
      const content = fs.readFileSync(path.join(tauriUiDir, 'zero-click-builder.html'), 'utf-8');
      await route.fulfill({ contentType: 'text/html', body: content });
    });

    // No mocking of internal APIs allowed per repo standards.
    // We expect the backend to be running.
    await page.setViewportSize({ width: 375, height: 812 });
  });

  test('Persona: Maya (Home Baker) uses Zero-Click Builder', async ({ page }) => {
    await page.goto('http://localhost:18789/zero-click-builder.html');

    // Verify Initial State
    await expect(page.getByRole('heading', { name: 'Zero-Click Generator' })).toBeVisible();

    const promptArea = page.locator('#prompt');
    await expect(promptArea).toBeVisible();
    await promptArea.fill('I am a home baker in Austin selling custom vegan cakes.');

    const generateBtn = page.locator('#generate-btn');
    await generateBtn.click();

    // Verify Loading State
    await expect(page.locator('#loading-step')).toBeVisible();
    await expect(page.locator('#loading-text')).toBeVisible();

    // Verify Result State - use more generic matchers since we don't mock the response
    await expect(page.getByText('Your business is live!')).toBeVisible({ timeout: 45000 });
    await expect(page.locator('#res-name')).not.toBeEmpty();
    await expect(page.locator('#res-count')).toContainText('items');
    await expect(page.locator('#res-url')).toContainText('.ohc.app');

    // Verify Action Buttons
    await expect(page.locator('#dashboard-btn')).toBeVisible();
    await expect(page.locator('#share-btn')).toBeVisible();
  });
});
