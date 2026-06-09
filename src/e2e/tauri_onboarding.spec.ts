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

    await page.route('/styles.css', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'styles.css'), 'utf-8');
        await route.fulfill({ contentType: 'text/css', body: content });
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
});

test.describe('Tauri Dashboard UI and UX Improvements', () => {
  test('Dashboard should have glassmorphism aesthetics applied', async ({ page }) => {
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR, process.env.TEST_WORKSPACE)
        : process.cwd();

    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');


    await page.route('/dashboard.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'dashboard.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.route('/styles.css', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'styles.css'), 'utf-8');
        await route.fulfill({ contentType: 'text/css', body: content });
    });


    await page.addInitScript(() => {
      window.__TAURI__ = {
        core: {
          invoke: async (cmd, args) => {
            if (cmd === 'generate_cloud_invite') {
              return "https://cloud.ohc.network/invite/mock-test";
            }
            throw new Error(`Unhandled command: ${cmd}`);
          }
        }
      };
    });

    await page.goto('/dashboard.html');

    // Check that the container class has the updated glassmorphism properties
    const container = page.locator('.container');
    await expect(container).toHaveCSS('backdrop-filter', 'blur(30px) saturate(2.1)');
    await expect(container).toHaveCSS('border-radius', '16px');
    await expect(container).toHaveCSS('background-color', 'rgba(255, 255, 255, 0.65)');
    await expect(container).toHaveCSS('border', '1px solid rgba(255, 255, 255, 0.4)');

    // Check dark mode
    await page.emulateMedia({ colorScheme: 'dark' });
    await expect(container).toHaveCSS('background-color', 'rgba(22, 22, 26, 0.7)');
    await expect(container).toHaveCSS('border', '1px solid rgba(255, 255, 255, 0.1)');
  });

  test('Setup and Assistant wizard forms can be submitted via Enter key', async ({ page }) => {
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


    await page.route('/success.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'success.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.route('/styles.css', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'styles.css'), 'utf-8');
        await route.fulfill({ contentType: 'text/css', body: content });
    });


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
            }
            throw new Error(`Unhandled command: ${cmd}`);
          }
        }
      };
    });

    // Test Setup Form
    await page.goto('/setup.html');
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Bakery").fill("My Cool Bakery");

    // Press Enter to submit
    await page.getByPlaceholder("e.g. Maya's Bakery").press('Enter');

    // It should navigate to assistant.html
    await expect(page).toHaveURL(/.*assistant\.html/);

    // Test Assistant Form
    await expect(page.getByRole('heading', { name: "Set up your Assistant" })).toBeVisible();
    await page.getByPlaceholder("e.g. Jarvis").fill("Jarvis");
    await page.locator('#assistant-tone').selectOption('Professional');

    // Press Enter to submit
    await page.getByPlaceholder("e.g. Jarvis").press('Enter');

    // It should navigate to success.html
    await expect(page).toHaveURL(/.*success\.html/);
    await expect(page.getByText('Workspace created for My Cool Bakery. Jarvis is ready to help.')).toBeVisible();
  });
});
