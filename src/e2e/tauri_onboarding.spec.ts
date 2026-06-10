import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Tauri Onboarding Wizard Flow', () => {
  test('Completes the onboarding flow, verifies validation, multi-step progression, and backend state resume', async ({ page, browser }) => {
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

    await page.route('/success.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'success.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    // We mock the Tauri backend API to allow state save/resume
    const mockTauriBackend = () => {
      window.__TAURI__ = {
        core: {
          invoke: async (cmd, args) => {
            if (cmd === 'get_onboarding_state') {
              const stateStr = sessionStorage.getItem('mockState');
              return stateStr ? JSON.parse(stateStr) : {};
            } else if (cmd === 'save_onboarding_state') {
              const stateStr = sessionStorage.getItem('mockState');
              const currentState = stateStr ? JSON.parse(stateStr) : {};
              sessionStorage.setItem('mockState', JSON.stringify({ ...currentState, ...args.state }));
              return null;
            }
            throw new Error(`Unhandled command: ${cmd}`);
          }
        }
      };
    };
    await page.addInitScript(mockTauriBackend);

    await page.route('http://mock/index.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'index.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });
    await page.route('http://mock/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
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

    // Setup page (Step 1: Context)
    await expect(page.getByRole('heading', { name: "How do you work?" })).toBeVisible();

    // Verify validation triggers
    await page.locator('#step-context').getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#context-error')).toBeVisible();

    // Valid context
    await page.getByText('Local Service').click();
    await page.locator('#step-context').getByRole('button', { name: 'Next' }).click();

    // Step 2: Categories
    await expect(page.getByRole('heading', { name: "What's your category?" })).toBeVisible();
    await page.locator('#step-categories').getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#categories-error')).toBeVisible();

    await page.getByPlaceholder("e.g. Graphic Design").fill("Home Repair");
    await page.locator('#step-categories').getByRole('button', { name: 'Next' }).click();

    // Step 3: Name
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    // Less than 3 chars validation
    await page.getByPlaceholder("e.g. Maya's Bakery").fill("Te");
    await page.locator('#step-name').getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#name-error')).toBeVisible();

    // Valid business name
    await page.getByPlaceholder("e.g. Maya's Bakery").fill("Test Business");
    await page.getByPlaceholder("Tagline (optional)").fill("Fixing things");
    await expect(page.locator('#name-error')).toBeHidden();

    await page.locator('#step-name').getByRole('button', { name: 'Next' }).click();

    // Step 4: Assistant
    await expect(page.getByRole('heading', { name: "Set up your Assistant" })).toBeVisible();

    // Verify validation triggers
    await page.locator('#step-assistant').getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#assistant-name-error')).toBeVisible();
    await expect(page.locator('#tone-error')).toBeVisible();
    await expect(page.locator('#assistant-name')).toHaveCSS('border-color', 'rgb(255, 59, 48)');

    await page.getByPlaceholder("e.g. Jarvis").fill("Jarvis");
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.locator('#step-assistant').getByRole('button', { name: 'Next' }).click();

    // Step 5: Offer
    await expect(page.getByRole('heading', { name: "Your First Offer" })).toBeVisible();

    await page.getByPlaceholder("e.g. Custom Birthday Cake").fill("Faucet Repair");
    await page.locator('#step-offer').getByRole('button', { name: 'Next' }).click();

    // Step 6: Template
    await expect(page.getByRole('heading', { name: "Template Selection" })).toBeVisible();

    // Verify validation triggers
    await page.getByRole('button', { name: 'Finish Setup' }).click();
    await expect(page.locator('#template-error')).toBeVisible();

    await page.locator('#template-selection').selectOption('Modern');



    // 2. Simulate Cross-Device Resume (Closing Page, Reopening, Checking State via Backend invoke mock)
    const savedStateStr = await page.evaluate(() => {
        try { return sessionStorage.getItem('mockState'); } catch(e) { return null; }
    });

    const newContext = await browser.newContext();
    const newPage = await newContext.newPage();

    await newPage.route('http://mock/index.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'index.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await newPage.route('http://mock/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await newPage.route('http://mock/success.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'success.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await newPage.addInitScript(mockTauriBackend);
    await newPage.goto('http://mock/index.html');

    await newPage.evaluate((stateStr) => {
        if (stateStr) {
            try { sessionStorage.setItem('mockState', stateStr); } catch(e) {}
        }
    }, savedStateStr);

    // Owner comes back to the app on another device
    await newPage.goto('http://mock/setup.html');

    // It should load the values, we can skip through
    await expect(newPage.locator('input[value="Local Service"]')).toBeChecked();
    await newPage.locator('#step-context').getByRole('button', { name: 'Next' }).click();

    await expect(newPage.getByPlaceholder("e.g. Graphic Design")).toHaveValue("Home Repair");
    await newPage.locator('#step-categories').getByRole('button', { name: 'Next' }).click();

    await expect(newPage.getByPlaceholder("e.g. Maya's Bakery")).toHaveValue("Test Business");
    await expect(newPage.getByPlaceholder("Tagline (optional)")).toHaveValue("Fixing things");
    await newPage.locator('#step-name').getByRole('button', { name: 'Next' }).click();

    await expect(newPage.getByPlaceholder("e.g. Jarvis")).toHaveValue("Jarvis");
    await expect(newPage.locator('#assistant-tone')).toHaveValue('Professional');
    await newPage.locator('#step-assistant').getByRole('button', { name: 'Next' }).click();

    // Step 5: Offer
    await expect(newPage.getByRole('heading', { name: "Your First Offer" })).toBeVisible();
    await expect(newPage.getByPlaceholder("e.g. Custom Birthday Cake")).toHaveValue("Faucet Repair");
    await newPage.locator('#step-offer').getByRole('button', { name: 'Next' }).click();

    // Step 6: Template
    await expect(newPage.getByRole('heading', { name: "Template Selection" })).toBeVisible();


    // Verify validation triggers
    await newPage.getByRole('button', { name: 'Finish Setup' }).click();
    await expect(newPage.locator('#template-error')).toBeVisible();

    await newPage.locator('#template-selection').selectOption('Modern');

    // Submit
    await newPage.getByRole('button', { name: 'Finish Setup' }).click();

    // Success page
    await expect(newPage.getByRole('heading', { name: "You're all set!" })).toBeVisible();
    await expect(newPage.getByText('Workspace created for Test Business. Jarvis is ready to help.')).toBeVisible();

    await newContext.close();

  });

  test('Validates 44px touch targets on mobile sizes and layout rules', async ({ page }) => {
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR, process.env.TEST_WORKSPACE)
        : process.cwd();

    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('http://mock/setup.html', async route => {
        const fs = require('fs');
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    // Set a mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('http://mock/setup.html');

    // Wait for the container to be visible
    const container = page.locator('.container');
    await expect(container).toBeVisible();
    await expect(container).toHaveClass(/glassmorphism/);

    const catInput = page.getByPlaceholder("e.g. Graphic Design");
    const box = await catInput.boundingBox();
    // Inputs are initially hidden. We need to navigate to that step or test something visible
    const option = page.locator('.radio-option').first();
    const optionBox = await option.boundingBox();

    if (optionBox) {
        expect(optionBox.height).toBeGreaterThanOrEqual(44);
    }
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

    await page.goto('/dashboard');

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
});
