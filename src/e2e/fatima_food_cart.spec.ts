import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Fatima Food Cart CUJ - Offline Menu Management', () => {
  test('Operator marks item sold out offline and verifies background sync when online', async ({ page, context }) => {
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR, process.env.TEST_WORKSPACE)
        : process.cwd();

    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    // Route setup for local HTML files
    await page.route('http://mock/dashboard.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'dashboard.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });
    await page.route('http://mock/menu.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'menu.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    // Mock Tauri Backend
    await page.addInitScript(() => {
      window.__TAURI__ = {
        core: {
          invoke: async (cmd: string, args: any) => {
            if (cmd === 'get_local_menu') {
              return [
                { id: 'prod_fatima_1', title: "Fatima's Special Biryani", inventoryCount: 20, isSoldOut: false }
              ];
            } else if (cmd === 'toggle_sold_out') {
              sessionStorage.setItem('offline_action', JSON.stringify(args));
              return null;
            } else if (cmd === 'sync_offline_actions') {
              // Simulate hit to backend
              const action = sessionStorage.getItem('offline_action');
              if (action) {
                 window.dispatchEvent(new CustomEvent('mock-backend-synced', { detail: JSON.parse(action) }));
              }
              return true;
            }
            return null;
          }
        }
      };
    });

    // 1. Navigate to Menu
    await page.goto('http://mock/dashboard.html');
    await page.getByRole('button', { name: 'Manage Daily Menu' }).click();
    await expect(page).toHaveURL(/.*menu\.html/);
    await expect(page.getByText("Fatima's Special Biryani")).toBeVisible();

    // 2. Go Offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));
    await expect(page.getByText('Working Offline')).toBeVisible();

    // 3. Toggle Sold Out
    await page.getByTitle('Toggle Sold Out').click();

    // 4. Go Online
    await context.setOffline(false);

    // Setup listener for sync proof
    const syncPromise = page.evaluate(() => {
        return new Promise(resolve => {
            window.addEventListener('mock-backend-synced', (e: any) => resolve(e.detail));
        });
    });

    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // 5. Verify Sync
    const syncedAction: any = await syncPromise;
    expect(syncedAction.id).toBe('prod_fatima_1');
    expect(syncedAction.isSoldOut).toBe(true);

    await expect(page.getByText('Connected Online')).toBeVisible();
  });
});
