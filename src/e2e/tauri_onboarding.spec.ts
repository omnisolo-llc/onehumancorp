import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Tauri Onboarding Wizard Flow', () => {
  test('Completes the onboarding flow with assistant setup', async ({ page }) => {

    // Serve the local files dynamically
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR, process.env.TEST_WORKSPACE)
        : process.cwd();

    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('/index.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'index.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.route('/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.route('/assistant.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'assistant.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.route('/success.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'success.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    // Mock the Tauri invoke API.
    // We use sessionStorage to preserve state across page navigations in Playwright
    await page.addInitScript(() => {
      window.__TAURI__ = {
        core: {
          invoke: async (cmd, args) => {
            if (cmd === 'get_onboarding_state') {
              const state = sessionStorage.getItem('mockState');
              return state ? JSON.parse(state) : {};
            } else if (cmd === 'save_onboarding_state') {
              const state = sessionStorage.getItem('mockState');
              const currentState = state ? JSON.parse(state) : {};
              sessionStorage.setItem('mockState', JSON.stringify({ ...currentState, ...args.state }));
              return null;
            } else if (cmd === 'generate_cloud_invite') {
              return "https://cloud.ohc.network/invite/mock-test";
            }
            throw new Error(`Unhandled command: ${cmd}`);
          }
        }
      };
    });

    // Navigate to the index.html page. Tauri loads it at /index.html.
    await page.goto('/index.html');

    await expect(page.getByRole('heading', { name: "Welcome to OHC" })).toBeVisible();
    await page.getByRole('button', { name: 'Start Onboarding' }).click();

    // Setup page
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    // Verify validation triggers
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#name-error')).toBeVisible();

    await page.getByPlaceholder("e.g. Maya's Bakery").fill("Test Business");
    await page.getByRole('button', { name: 'Next' }).click();

    // Assistant Setup page
    await expect(page.getByRole('heading', { name: "Set up your Assistant" })).toBeVisible();

    // Verify validation triggers
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#name-error')).toBeVisible();
    await expect(page.locator('#tone-error')).toBeVisible();

    await page.getByPlaceholder("e.g. Jarvis").fill("Jarvis");
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByRole('button', { name: 'Next' }).click();

    // Success page
    await expect(page.getByRole('heading', { name: "You're all set!" })).toBeVisible();
    await expect(page.getByText('Workspace created for Test Business. Jarvis is ready to help.')).toBeVisible();
  });

  test('Validates 44px touch targets on mobile sizes', async ({ page }) => {
    // Set a mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });

    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR, process.env.TEST_WORKSPACE)
        : process.cwd();
    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.goto('/setup.html');
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    const nameInput = page.getByPlaceholder("e.g. Maya's Bakery");
    await expect(nameInput).toBeVisible();
    const box = await nameInput.boundingBox();
    expect(box?.height).toBeGreaterThanOrEqual(44);

    const nextBtn = page.getByRole('button', { name: 'Next' });
    const btnBox = await nextBtn.boundingBox();
    expect(btnBox?.height).toBeGreaterThanOrEqual(44);
  });

  test('Enter key submits the inputs on setup and assistant screens', async ({ page }) => {
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR, process.env.TEST_WORKSPACE)
        : process.cwd();
    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.route('/assistant.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'assistant.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.addInitScript(() => {
      window.__TAURI__ = {
        core: {
          invoke: async (cmd, args) => {
            if (cmd === 'get_onboarding_state') return {};
            if (cmd === 'save_onboarding_state') return null;
            throw new Error(`Unhandled command: ${cmd}`);
          }
        }
      };
    });

    await page.goto('/setup.html');
    const nameInput = page.getByPlaceholder("e.g. Maya's Bakery");
    await nameInput.fill("Keyboard Business");
    await nameInput.press('Enter');

    // Should navigate to assistant.html
    await expect(page.getByRole('heading', { name: "Set up your Assistant" })).toBeVisible();

    const assistantInput = page.getByPlaceholder("e.g. Jarvis");
    await assistantInput.fill("Keyboard Assistant");
    await assistantInput.press('Enter');

    // Should focus the select
    const select = page.locator('#assistant-tone');
    await expect(select).toBeFocused();
  });

  test('Validation errors disappear immediately upon typing', async ({ page }) => {
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR, process.env.TEST_WORKSPACE)
        : process.cwd();
    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.goto('/setup.html');

    const nextBtn = page.getByRole('button', { name: 'Next' });
    await nextBtn.click();

    const errorMsg = page.locator('#name-error');
    await expect(errorMsg).toBeVisible();

    const nameInput = page.getByPlaceholder("e.g. Maya's Bakery");
    await nameInput.pressSequentially("a");

    await expect(errorMsg).toBeHidden();
  });

  test('Restores draft onboarding states from memory (sessionStorage)', async ({ page }) => {
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR, process.env.TEST_WORKSPACE)
        : process.cwd();
    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    // Mock Tauri invoke to return a saved state
    await page.addInitScript(() => {
      window.__TAURI__ = {
        core: {
          invoke: async (cmd, args) => {
            if (cmd === 'get_onboarding_state') {
              return { businessName: 'Restored Business Name' };
            }
            throw new Error(`Unhandled command: ${cmd}`);
          }
        }
      };
    });

    await page.goto('/setup.html');
    const nameInput = page.getByPlaceholder("e.g. Maya's Bakery");
    await expect(nameInput).toHaveValue('Restored Business Name');
  });
});
