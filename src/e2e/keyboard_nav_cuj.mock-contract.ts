import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Keyboard Navigation CUJ', () => {
  test('Non-technical owner can complete setup using only keyboard navigation', async ({ page }) => {
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR || process.cwd(), process.env.TEST_WORKSPACE)
        : process.cwd();

    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('http://mock/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.addInitScript(() => {
      window.__TAURI__ = {
        core: {
          invoke: async (cmd, args) => {
            return null;
          }
        }
      };
    });

    await page.goto('http://mock/setup.html');

    // Step Initial
    await expect(page.locator('#step-initial')).toHaveClass(/active/);

    // Start My Business
    await page.getByRole('button', { name: 'Step-by-Step Setup' }).focus();
    await page.keyboard.press('Enter');
    await expect(page.locator('#step-context')).toHaveClass(/active/);

    // Step Context
    // Focus the first context card
    await page.locator('label[data-testid="context-local"]').focus();
    // Press Enter to select it
    await page.keyboard.press('Enter');
    // Ensure it's selected
    await expect(page.locator('label[data-testid="context-local"]')).toHaveClass(/selected/);

    // Move to Next button and press Enter
    await page.waitForTimeout(100);
    await page.locator('#step-context .next-step-btn').focus();
    await page.keyboard.press('Enter');
    await expect(page.locator('#step-categories')).toHaveClass(/active/);

    // Step Categories
    await page.locator('#business-categories').focus();
    await page.locator('#business-categories').selectOption('Handyman');
    await page.locator('#business-categories').press('Enter'); // should advance because global enter catches it
    await expect(page.locator('#step-name')).toHaveClass(/active/);

    // Step Name
    await page.locator('#business-name').focus();
    await page.keyboard.type('My Cool Business');
    await page.keyboard.press('Enter'); // Global enter advances
    await expect(page.locator('#step-assistant')).toHaveClass(/active/);

    // Step Assistant
    await page.waitForTimeout(100);
    await page.locator('label[data-testid="team-operations"]').focus();
    await page.keyboard.press('Enter'); // Selects it
    await expect(page.locator('label[data-testid="team-operations"]')).toHaveClass(/selected/);

    await page.locator('#assistant-tone').selectOption('Professional');
    await page.locator('#assistant-tone').focus();
    await page.keyboard.press('Enter'); // Global enter advances
    await expect(page.locator('#step-admin')).toHaveClass(/active/);

    // Step Admin
    await page.locator('#admin-name').focus();
    await page.keyboard.type('Admin');
    await page.keyboard.press('Tab'); // move to email
    await page.keyboard.type('admin@example.com');
    await page.keyboard.press('Tab'); // move to password
    await page.keyboard.type('password123');
    await page.locator('#admin-password').press('Enter'); // Global enter advances
    await expect(page.locator('#step-offer')).toHaveClass(/active/);

  });
});
