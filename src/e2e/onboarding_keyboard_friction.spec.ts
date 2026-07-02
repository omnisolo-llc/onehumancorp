import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Onboarding Keyboard Friction Mitigation', () => {

  test('Enter key should trigger Next button on setup steps', async ({ page }) => {
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

    // Step 0 -> Step Context
    await page.getByRole('button', { name: 'Step-by-Step Setup' }).click();
    await expect(page.locator('#step-context')).toHaveClass(/active/);

    // Step Context
    await page.getByTestId('context-local').click();
    await page.getByTestId('context-local').focus();
    // Press enter on next button to trigger navigation
    await page.locator('#step-context .next-step-btn').focus();
    await page.keyboard.press('Enter');
    await expect(page.locator('#step-categories')).toHaveClass(/active/);

    // Step Categories
    await page.locator('#business-categories').selectOption('Handyman');
    await page.locator('#business-categories').focus();
    // For select, we need to focus it or body to press Enter, let's just press Enter globally
    await page.keyboard.press('Enter');
    await expect(page.locator('#step-name')).toHaveClass(/active/);

    // Step Name
    await page.locator('#business-name').fill('Test Business');
    await page.locator('#business-name').press('Enter');
    await expect(page.locator('#step-assistant')).toHaveClass(/active/);

    // Step Assistant
    await page.getByTestId('team-operations').click();
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.locator('#assistant-tone').press('Enter');
    await expect(page.locator('#step-admin')).toHaveClass(/active/);

    // Step Admin
    await page.locator('#admin-name').fill('Admin');
    await page.locator('#admin-email').fill('admin@test.com');
    await page.locator('#admin-password').fill('password123');
    await page.locator('#admin-password').press('Enter');
    await expect(page.locator('#step-offer')).toHaveClass(/active/);
  });

  test('Enter key does not submit if validation fails', async ({ page }) => {
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR || process.cwd(), process.env.TEST_WORKSPACE)
        : process.cwd();

    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('http://mock/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });
    await page.goto('http://mock/setup.html');

    // Step 0 -> Step Context
    await page.getByRole('button', { name: 'Step-by-Step Setup' }).click();
    await expect(page.locator('#step-context')).toHaveClass(/active/);

    // Try to proceed without selecting context (fails validation)
    await page.locator('#step-context .next-step-btn').focus();
    await page.keyboard.press('Enter');

    // Should still be on step-context
    await expect(page.locator('#step-context')).toHaveClass(/active/);
    // Error should be visible
    await expect(page.locator('#context-error')).toBeVisible();
  });

  test('Enter key works on Instant Build page', async ({ page }) => {
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
             if (cmd === 'process_intake') {
                return {
                    business_name: 'Instant',
                    business_type: 'Other',
                    categories: [],
                    location: 'Local',
                    target_audience: 'Everyone',
                    initial_products: []
                };
             }
             return null;
          }
        }
      };
    });

    await page.goto('http://mock/setup.html');

    // Go to instant build
    // Now on step-initial
    await expect(page.locator('#step-initial')).toHaveClass(/active/);

    // Press Enter to submit instant build? No, textarea should not submit on enter.
    // Fill textarea
    await page.locator('#instant-bio').fill('Test bio');
    await page.locator('#instant-bio').press('Enter');

    // It should add a newline, not advance.
    await expect(page.locator('#step-initial')).toHaveClass(/active/);
    const val = await page.locator('#instant-bio').inputValue();
    expect(val).toBe('Test bio\n');
  });

  test('Enter key submits chat message in Conversational Setup', async ({ page }) => {
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR || process.cwd(), process.env.TEST_WORKSPACE)
        : process.cwd();

    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('http://mock/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    let chatRequestSent = false;
    await page.route('**/api/onboarding/chat', async route => {
        chatRequestSent = true;
        await route.fulfill({
            status: 200,
            contentType: 'application/json',
            body: JSON.stringify({
                reply: 'Hello there!',
                is_complete: false
            })
        });
    });

    await page.goto('http://mock/setup.html');

    // Go to chat setup
    await page.getByRole('button', { name: 'Conversational Setup' }).click();
    await expect(page.locator('#step-chat')).toHaveClass(/active/);

    // Press Enter to submit chat
    await page.locator('#chat-input').fill('Test chat message');
    await page.locator('#chat-input').press('Enter');

    // It should add a message bubble to the view and send the API request
    await expect(page.locator('#chat-messages').getByText('Test chat message')).toBeVisible();
    expect(chatRequestSent).toBe(true);
  });

});
