import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Tauri Onboarding Wizard Flow', () => {
  test('Completes the onboarding flow, verifies 3-char minimum validation, enter-key progression, and backend state resume', async ({ page, browser }) => {
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

    await page.route('/assistant-setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'assistant-setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.route('/success.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'success.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    // Mock the Tauri invoke API.
    // We use sessionStorage to preserve state across page navigations in Playwright
    // This acts as our "Rust Backend / Postgres / Redis" local proxy for the Tauri frontend.
    const mockTauriBackend = () => {
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
            } else if (cmd === 'generate_cloud_invite') {
              return "https://cloud.ohc.network/invite/mock-test";
            }
            throw new Error(`Unhandled command: ${cmd}`);
          }
        }
      };
    };
    await page.addInitScript(mockTauriBackend);

    // Start local server or mock to circumvent cross-origin restrictions for session storage
    await page.route('http://mock/index.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'index.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });
    await page.route('http://mock/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });
    await page.route('http://mock/assistant-setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'assistant-setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });
    await page.route('http://mock/success.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'success.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    // Navigate to the mock index
    await page.goto('http://mock/index.html');

    await expect(page.getByRole('heading', { name: "Welcome to OHC" })).toBeVisible();
    await page.getByRole('button', { name: 'Start Onboarding' }).click();

    // Setup page
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    // Verify validation triggers
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#name-error')).toBeVisible();
    await expect(page.locator('#business-name')).toHaveCSS('border-color', 'rgb(255, 59, 48)'); // #FF3B30

    // Less than 3 chars validation
    await page.getByPlaceholder("e.g. Maya's Bakery").fill("Te");
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#name-error')).toBeVisible();

    // Valid business name
    await page.getByPlaceholder("e.g. Maya's Bakery").fill("Test Business");
    await expect(page.locator('#name-error')).toBeHidden();

    // Use Enter key to submit
    await page.getByPlaceholder("e.g. Maya's Bakery").press('Enter');

    // Assistant Setup page
    await expect(page.getByRole('heading', { name: "Set up your Assistant" })).toBeVisible();

    // Verify validation triggers
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#name-error')).toBeVisible();
    await expect(page.locator('#tone-error')).toBeVisible();
    await expect(page.locator('#assistant-name')).toHaveCSS('border-color', 'rgb(255, 59, 48)');

    // 2. Simulate Cross-Device Resume (Closing Page, Reopening, Checking State via Backend invoke mock)
    // Grab the stored backend state proxy
    const savedStateStr = await page.evaluate(() => {
        try { return sessionStorage.getItem('mockState'); } catch(e) { return null; }
    });

    const newContext = await browser.newContext();
    const newPage = await newContext.newPage();

    // Re-register our mock route handlers in the new context
    await newPage.route('http://mock/index.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'index.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await newPage.route('http://mock/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await newPage.route('http://mock/assistant-setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'assistant-setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await newPage.route('http://mock/success.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'success.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await newPage.addInitScript(mockTauriBackend);

    // Playwright needs to navigate to a valid origin first to use sessionStorage
    await newPage.goto('http://mock/index.html');

    // Rehydrate the 'backend' state
    await newPage.evaluate((stateStr) => {
        if (stateStr) {
            try { sessionStorage.setItem('mockState', stateStr); } catch(e) {}
        }
    }, savedStateStr);

    // Owner comes back to the app on another device
    await newPage.goto('http://mock/setup.html');

    // Business name should be restored
    await expect(newPage.getByPlaceholder("e.g. Maya's Bakery")).toHaveValue("Test Business");
    await newPage.getByRole('button', { name: 'Next' }).click();

    // Assistant Setup page
    await expect(newPage.getByRole('heading', { name: "Set up your Assistant" })).toBeVisible();

    await newPage.getByPlaceholder("e.g. Jarvis").fill("Jarvis");
    await newPage.locator('#assistant-tone').selectOption('Professional');
    await newPage.getByRole('button', { name: 'Next' }).click();

    // Success page
    await expect(newPage.getByRole('heading', { name: "You're all set!" })).toBeVisible();
    await expect(newPage.getByText('Workspace created for Test Business. Jarvis is ready to help.')).toBeVisible();

    await newContext.close();
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

    await page.route('/assistant-setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'assistant-setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.route('/success.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'success.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
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
    await expect(page).toHaveURL(/.*assistant-setup\.html/);

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
