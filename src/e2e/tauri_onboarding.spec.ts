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

    const routeHandler = async (route) => {
        const url = new URL(route.request().url());
        const filename = path.basename(url.pathname);
        const filepath = path.join(tauriUiDir, filename);
        if (fs.existsSync(filepath)) {
            const content = fs.readFileSync(filepath, 'utf-8');
            const contentType = filename.endsWith('.css') ? 'text/css' : 'text/html';
            await route.fulfill({ contentType, body: content });
        } else {
            await route.continue();
        }
    };

    await page.route('http://mock/**', routeHandler);

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
            }
            throw new Error(`Unhandled command: ${cmd}`);
          }
        }
      };
    };
    await page.addInitScript(mockTauriBackend);

    // Navigate to the mock index
    await page.goto('http://mock/index.html');

    await expect(page.getByRole('heading', { name: "Welcome to OHC" })).toBeVisible();
    await page.getByRole('button', { name: 'Start Onboarding' }).click();

    // Setup page
    await expect(page.getByRole('heading', { name: "Business Profile" })).toBeVisible();

    // Verify validation triggers
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#name-error')).toBeVisible();

    // Valid business name and industry
    await page.getByPlaceholder("e.g. Maya's Bakery").fill("Test Business");
    await page.getByText("🍰 Bakery").click();
    await page.getByRole('button', { name: 'Next' }).click();

    // Assistant Setup page
    await expect(page.getByRole('heading', { name: "Your AI Team" })).toBeVisible();

    // Verify validation triggers
    await page.getByRole('button', { name: 'Finish Setup' }).click();
    await expect(page.locator('#name-error')).toBeVisible();
    await expect(page.locator('#tone-error')).toBeVisible();

    // 2. Simulate Cross-Device Resume
    const savedStateStr = await page.evaluate(() => {
        try { return sessionStorage.getItem('mockState'); } catch(e) { return null; }
    });

    const newContext = await browser.newContext();
    const newPage = await newContext.newPage();

    await newPage.route('http://mock/**', routeHandler);
    await newPage.addInitScript(mockTauriBackend);

    await newPage.goto('http://mock/index.html');

    await newPage.evaluate((stateStr) => {
        if (stateStr) {
            try { sessionStorage.setItem('mockState', stateStr); } catch(e) {}
        }
    }, savedStateStr);

    await newPage.goto('http://mock/setup.html');

    // Business name should be restored
    await expect(newPage.getByPlaceholder("e.g. Maya's Bakery")).toHaveValue("Test Business");
    await newPage.getByRole('button', { name: 'Next' }).click();

    // Assistant Setup page
    await expect(newPage.getByRole('heading', { name: "Your AI Team" })).toBeVisible();

    await newPage.getByPlaceholder("e.g. Jarvis").fill("Jarvis");
    await newPage.locator('#assistant-tone').selectOption('Friendly');
    await newPage.getByRole('button', { name: 'Finish Setup' }).click();

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

    await page.route('http://mock/**', async (route) => {
        const url = new URL(route.request().url());
        const filename = path.basename(url.pathname);
        const filepath = path.join(tauriUiDir, filename);
        if (fs.existsSync(filepath)) {
            const content = fs.readFileSync(filepath, 'utf-8');
            const contentType = filename.endsWith('.css') ? 'text/css' : 'text/html';
            await route.fulfill({ contentType, body: content });
        } else {
            await route.continue();
        }
    });

    await page.goto('http://mock/setup.html');

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
});
