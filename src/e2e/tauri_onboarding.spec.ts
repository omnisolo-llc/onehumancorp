import { test as baseTest, expect } from '@playwright/test';
import * as path from 'path';

export const test = baseTest.extend({});

test('Tauri HTML Onboarding flow', async ({ page, browser }) => {
  test.setTimeout(180000);
  const workspaceRoot = process.env.TEST_WORKSPACE
      ? path.join(process.env.TEST_SRCDIR as string, process.env.TEST_WORKSPACE)
      : process.cwd();

  const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');
  const baseUrl = `file://${tauriUiDir}`;

  await page.addInitScript(() => {
    (window as any).__TAURI__ = {
      core: {
        invoke: async (cmd: string, args: any) => {
          if (cmd === 'get_onboarding_state') {
            const state = localStorage.getItem('mockState');
            return state ? JSON.parse(state) : {};
          } else if (cmd === 'save_onboarding_state') {
            const state = localStorage.getItem('mockState');
            const currentState = state ? JSON.parse(state) : {};
            localStorage.setItem('mockState', JSON.stringify({ ...currentState, ...args.state }));
            return null;
          }
          throw new Error(`Unhandled command: ${cmd}`);
        }
      }
    };
  });

  await page.goto(`${baseUrl}/index.html`);
  await expect(page.locator('h1')).toHaveText('Welcome to OHC');
  await page.click('#start-btn');

  // URL should be setup.html
  await expect(page).toHaveURL(/.*setup\.html/);
  await expect(page.locator('h1')).toHaveText("What's the name of your business?");

  // Validation Check: Less than 3 characters
  await page.fill('#business-name', 'Ma');
  await page.click('#next-btn');
  await expect(page.locator('#name-error')).toBeVisible();

  // Cross device resume preparation:
  await page.fill('#business-name', 'My Bakery');
  await page.click('#next-btn');

  // Assistant page
  await expect(page).toHaveURL(/.*assistant\.html/);
  await expect(page.locator('h1')).toHaveText('Set up your Assistant');

  // Verify state saving by getting the localStorage manually and closing the page
  const savedStateStr = await page.evaluate(() => localStorage.getItem('mockState'));

  // Re-open simulating Cross Device
  const newContext = await browser.newContext();
  const newPage = await newContext.newPage();

  await newPage.addInitScript(() => {
    (window as any).__TAURI__ = {
      core: {
        invoke: async (cmd: string, args: any) => {
          if (cmd === 'get_onboarding_state') {
            const state = localStorage.getItem('mockState');
            return state ? JSON.parse(state) : {};
          } else if (cmd === 'save_onboarding_state') {
            const state = localStorage.getItem('mockState');
            const currentState = state ? JSON.parse(state) : {};
            localStorage.setItem('mockState', JSON.stringify({ ...currentState, ...args.state }));
            return null;
          }
          throw new Error(`Unhandled command: ${cmd}`);
        }
      }
    };
  });

  await newPage.goto(`${baseUrl}/index.html`); // Need to go to same origin to set storage

  await newPage.evaluate((stateStr: string) => {
      if (stateStr) {
          try { localStorage.setItem('mockState', stateStr); } catch(e) {}
      }
  }, savedStateStr);

  await newPage.goto(`${baseUrl}/setup.html`);
  await expect(newPage.locator('#business-name')).toHaveValue('My Bakery', { timeout: 15000 });
  // Submitting using Enter key to test Keyboard
  await newPage.locator('#business-name').press('Enter');

  // It should be assistant.html
  await expect(newPage).toHaveURL(/.*assistant\.html/);
  await expect(newPage.locator('h1')).toHaveText('Set up your Assistant');

  // Test Assistant form Enter key submission
  await newPage.fill('#assistant-name', 'Jarvis');
  await newPage.selectOption('#assistant-tone', 'Friendly');
  await newPage.locator('#assistant-tone').press('Enter');

  // It should be success.html
  await expect(newPage).toHaveURL(/.*success\.html/);
  await expect(newPage.locator('h1')).toHaveText("You're all set!");
  await expect(newPage.locator('#success-msg')).toContainText('Workspace created for My Bakery');
  await expect(newPage.locator('#success-msg')).toContainText('Jarvis is ready to help');

  // Verify CSS styles (Light mode)
  const container = newPage.locator('.container');
  await expect(container).toBeVisible();
  await expect(container).toHaveCSS('backdrop-filter', /blur\(30px\)/);
  await expect(container).toHaveCSS('border-radius', '16px');

  // Verify Dark mode via CSS emulation
  await newPage.emulateMedia({ colorScheme: 'dark' });
  await expect(container).toHaveCSS('background-color', 'rgba(22, 22, 26, 0.7)');

  const button = newPage.locator('#dashboard-btn');
  await expect(button).toBeVisible();
  await expect(button).toHaveCSS('border-radius', '8px');
});
