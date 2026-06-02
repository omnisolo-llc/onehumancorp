import { test, expect } from '@playwright/test';

test.describe('Offline-First Localization and Currency', () => {
  test('Fatima changes language and currency offline in POS', async ({ page }) => {
    // Navigate to POS terminal
    await page.goto('/pos/terminal');

    // Simulate setting local storage mock data as if she was online before
    await page.evaluate(() => {
      localStorage.setItem('ohc_i18n_ar', JSON.stringify({
        'clocked_in': 'مسجل الدخول',
        'not_clocked_in': 'غير مسجل الدخول',
        'clock_in': 'تسجيل الدخول',
        'lock': 'قفل',
        'new_order': 'طلب جديد',
        'quick_actions': 'إجراءات سريعة'
      }));
      localStorage.setItem('ohc_fx_rates', JSON.stringify({
        'AED': 3.67,
        'EUR': 0.92
      }));
      localStorage.setItem('ohc_offline_staff', JSON.stringify([
        { id: '1', name: 'Fatima', role: 'Manager', pin_hash: '1234' }
      ]));
    });

    await page.reload();

    // Unlock terminal
    await page.click('button:has-text("1")');
    await page.click('button:has-text("2")');
    await page.click('button:has-text("3")');
    await page.click('button:has-text("4")');

    // Wait for the UI to load
    await expect(page.locator('h1')).toHaveText('Fatima');

    // Go offline
    await page.context().setOffline(true);

    // Change language to Arabic
    await page.selectOption('select', { value: 'ar' });

    // Check if translated text appears
    await expect(page.locator('button:has-text("تسجيل الدخول")')).toBeVisible();
    await expect(page.locator('button:has-text("قفل")')).toBeVisible();

    // Change Currency to AED
    const selects = page.locator('select');
    await selects.nth(1).selectOption('AED');

    // Check if offline toast appears
    await expect(page.locator('text=Converted using yesterday\'s rate. Will finalize on sync.')).toBeVisible();

    // Restore online
    await page.context().setOffline(false);
  });
});
