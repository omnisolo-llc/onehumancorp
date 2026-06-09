import { test, expect } from '@playwright/test';
import path from 'path';
import fs from 'fs';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test.beforeEach(async ({ page }) => {
    // For Tauri mock testing, since playwright config hits localhost:3000
    // we set up the route mock.
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR, process.env.TEST_WORKSPACE)
        : process.cwd();

    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('**/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.route('**/assistant-setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'assistant-setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.route('**/success.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'success.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.route('**/dashboard.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'dashboard.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.addInitScript(() => {
      window.__TAURI__ = {
        core: {
          invoke: async (cmd: string, args: any) => {
            if (cmd === 'get_onboarding_state') {
              const state = sessionStorage.getItem('mockState');
              return state ? JSON.parse(state) : {};
            } else if (cmd === 'save_onboarding_state') {
              const state = sessionStorage.getItem('mockState');
              const currentState = state ? JSON.parse(state) : {};
              sessionStorage.setItem('mockState', JSON.stringify({ ...currentState, ...args.state }));
              return null;
            }
            throw new Error(`Unhandled command: ${cmd}`);
          }
        }
      };
    });

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

    await page.goto('http://mock/setup.html');
    await page.waitForLoadState('networkidle');

    await expect(page.locator('.container').first()).toBeVisible({ timeout: 5000 });

    await page.getByPlaceholder("e.g. Maya's Bakery").fill('I run a modern art shop online');
    await page.getByRole('button', { name: /Next/ }).click();

    await expect(page.getByRole('heading', { name: /Set up your Assistant/ })).toBeVisible({ timeout: 5000 });
    await page.getByPlaceholder("e.g. Jarvis").fill("Artie");
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByRole('button', { name: /Next/ }).click();

    await expect(page.getByRole('heading', { name: /You're all set!/ })).toBeVisible({ timeout: 20000 });
  });

  test('validates empty input in Tell us about your business', async ({ page }) => {
    await page.goto('http://mock/setup.html');

    const generateBtn = page.getByRole('button', { name: /Next/ });

    await page.getByPlaceholder("e.g. Maya's Bakery").fill('');
    await generateBtn.click();
    await expect(page.locator('#name-error')).toBeVisible();

    await page.getByPlaceholder("e.g. Maya's Bakery").fill('Art');
    await expect(page.locator('#name-error')).toBeHidden();
  });
});
