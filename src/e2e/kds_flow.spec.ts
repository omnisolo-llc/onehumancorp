import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Multilingual Order Interceptor KDS Flow', () => {

  test.beforeEach(async ({ page }) => {
    let tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    if (!fs.existsSync(tauriUiDir)) {
        tauriUiDir = path.join(process.env.RUNFILES_DIR || process.cwd(), '_main/src/ui/tauri/src/ui');
    }
    await page.route('**/kds.html', async route => {
      const content = fs.readFileSync(path.join(tauriUiDir, 'kds.html'), 'utf-8');
      await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.setViewportSize({ width: 768, height: 1024 });
  });

  test('Persona: Fatima (Food Cart) views a translated order on KDS and marks it complete', async ({ page }) => {
    // We intercept the KDS API response to provide a mock order from a non-English customer
    await page.route('**/api/ui/kds?mobile_optimized=true', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify([{
            id: 'ord-12345678',
            original_message: 'Quiero tres tacos al pastor, por favor.',
            translated_items: '3x Al Pastor Tacos',
            created_at: new Date().toISOString()
        }])
      });
    });

    let completeCalled = false;
    await page.route('**/api/ui/kds/complete', async route => {
        completeCalled = true;
        const postData = JSON.parse(route.request().postData() || '{}');
        expect(postData.order_id).toBe('ord-12345678');
        await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ success: true }) });
    });

    await page.goto('http://localhost:18789/kds.html');

    // Verify UI components
    await expect(page.getByRole('heading', { name: 'Kitchen Display System' })).toBeVisible();
    await expect(page.getByText('Translated incoming orders.')).toBeVisible();

    // Verify translated order displays correctly
    await expect(page.getByText('Order #ord-12')).toBeVisible();
    await expect(page.getByText('3x Al Pastor Tacos')).toBeVisible();
    await expect(page.getByText('Original: "Quiero tres tacos al pastor, por favor."')).toBeVisible();

    // Verify button works
    await page.getByRole('button', { name: 'Mark Complete' }).click();

    // Order card should disappear (opacity or display none in real logic, just wait for network)
    await page.waitForTimeout(500);
    expect(completeCalled).toBe(true);
  });
});
