import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Tauri Onboarding Wizard Flow', () => {
  test('Completes the onboarding flow, verifies validation, multi-step progression, and backend state resume', async ({ page, browser }) => {
    // Serve the local files dynamically
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR, process.env.TEST_WORKSPACE)
        : require('path').join(__dirname, '../..');

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
            if (cmd === 'start_onboarding') return { success: true };
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
    await page.waitForTimeout(500);

    await page.locator('#business-categories').selectOption({ label: 'Handyman' });
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
    await newPage.route('**/*success.html*', async route => {
        const fs = require('fs');
        const path = require('path');
        const workspaceRoot = process.env.TEST_WORKSPACE ? path.join(process.env.TEST_SRCDIR, process.env.TEST_WORKSPACE) : path.join(__dirname, '../..');
        const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');
        const content = fs.readFileSync(path.join(tauriUiDir, 'success.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

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
    await newPage.waitForTimeout(500);
    await expect(newPage.locator('input[value="Local Service"]')).toBeChecked();
    await newPage.evaluate(() => { document.querySelector('#step-context .next-step-btn').click(); });

    await expect(newPage.locator('#business-categories')).toHaveValue('Handyman');
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
    newPage.on('console', msg => console.log('NEWPAGE CONSOLE:', msg.text()));
    await newPage.getByRole('button', { name: 'Finish Setup' }).click();
    await expect(newPage.locator('#template-error')).toBeVisible();

    await newPage.locator('#template-selection').selectOption('Modern');

    // Submit
    newPage.on('console', msg => console.log('NEWPAGE CONSOLE:', msg.text()));
    await newPage.getByRole('button', { name: 'Finish Setup' }).click();

    // Success page
    await newPage.waitForLoadState('networkidle');
    await expect(newPage.getByRole('heading', { name: "You're all set!" })).toBeVisible();
    await expect(newPage.getByText('Workspace created for Test Business. Jarvis is ready to help.')).toBeVisible();

    await newContext.close();

  });

  test('Validates 44px touch targets on mobile sizes and layout rules', async ({ page }) => {
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR, process.env.TEST_WORKSPACE)
        : require('path').join(__dirname, '../..');

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

    const option = page.locator('.radio-option').first();
    const optionBox = await option.boundingBox();
    const catInput = page.getByPlaceholder("e.g. Graphic Design");

    if (optionBox) {
        expect(optionBox.height).toBeGreaterThanOrEqual(44);
    }
  });

});

test.describe('Tauri Dashboard UI and UX Improvements', () => {


  test('Setup UI should have glassmorphism aesthetics applied', async ({ page }) => {
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR || require('path').join(__dirname, '../..'), process.env.TEST_WORKSPACE)
        : require('path').join(__dirname, '../..');

    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('http://mock/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.goto('http://mock/setup.html');

    // Check that the container class has the updated glassmorphism properties
    const container = page.locator('.container');
    await expect(container).toHaveCSS('backdrop-filter', 'blur(30px) saturate(2.1)');
    await expect(container).toHaveCSS('border-radius', '16px');
    await expect(container).toHaveCSS('background-color', 'rgba(255, 255, 255, 0.65)');
    await expect(container).toHaveCSS('border', '1px solid rgba(255, 255, 255, 0.4)');
  });

  test('Dashboard should have glassmorphism aesthetics applied', async ({ page }) => {
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR, process.env.TEST_WORKSPACE)
        : require('path').join(__dirname, '../..');

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
            if (cmd === 'start_onboarding') return { success: true };
            throw new Error(`Unhandled command: ${cmd}`);
          }
        }
      };
    });

        await page.route('http://mock/dashboard.html', async route => {
        const fs = require('fs');
        const path = require('path');
        const workspaceRoot = process.env.TEST_WORKSPACE ? path.join(process.env.TEST_SRCDIR, process.env.TEST_WORKSPACE) : path.join(__dirname, '../..');
        const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');
        const content = fs.readFileSync(path.join(tauriUiDir, 'dashboard.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.goto('http://mock/dashboard.html');

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

  test('Instant AI Build completes setup quickly', async ({ page }) => {
    // We navigate to /ui/tauri/src/ui/setup.html from the local server since that's what other tests use when they do page.goto('/ui/tauri/src/ui/setup.html')
    // Wait, earlier tests did `await page.goto('http://mock/setup.html');` but they also override `fs` with `process.cwd()`
    // We will just run the test via the mock server or local file protocol.

    // Instead of overriding everything, let's just use what works
    const path = require('path');
    const fs = require('fs');
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR, process.env.TEST_WORKSPACE)
        : path.join(__dirname, '../..'); // This makes it work
    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('http://mock/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    // Mock API intercept for intake
    await page.route('**/api/onboarding/intake', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_name: 'Chicago Emergency Handyman',
          business_type: 'Handyman',
          categories: ['services', 'physical'],
          location: 'Chicago, IL',
          target_audience: 'Homeowners',
          initial_products: [
            { name: 'Emergency Plumbing Repair', price: '150.00', description: 'Fast repair' }
          ]
        })
      });
    });

    // Mock state save/get
    await page.route('**/api/onboarding/state', async route => {
      if (route.request().method() === 'GET') {
          await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({}) });
      } else {
          await route.fulfill({ status: 204 });
      }
    });

    await page.goto('http://mock/setup.html');

    // Fill instant bio
    await page.fill('#instant-bio', 'I am a local handyman offering emergency plumbing and repair services in Chicago.');

    // Click Instant Build
    await page.click('#instant-build-btn');

    // Wait for the step update
    await page.waitForTimeout(1000);

    // Verify it jumped to step-template
    await expect(page.locator('#step-template')).toHaveClass(/active/);
    await expect(page.locator('#template-selection')).toHaveValue('Modern');
  });
});
