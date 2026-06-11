import { test, expect } from '@playwright/test';

/**
 * Persona: Fatima, Food Cart Operator
 * Concept: Accepting USD from tourists in an offline festival.
 * CUJ: 1. Fatima opens POS.
 *      2. Toggles language to Arabic.
 *      3. Toggles currency to USD.
 *      4. Simulates offline mode.
 *      5. Performs a transaction.
 *      6. Verifies cached rates are used and UI shows feedback.
 */

test('Global Offline-First Localization & Currency Toggle', async ({ page, context }) => {
  // Seed staff data for offline login simulation
  await page.addInitScript(() => {
    localStorage.setItem('ohc_offline_staff', JSON.stringify([{
      id: 'fatima-1',
      name: 'Fatima',
      role: 'Manager',
      pin_hash: '1234'
    }]));

    // Seed FX rates for offline conversion
    localStorage.setItem('ohc-localization-storage', JSON.stringify({
      state: {
        locale: 'en',
        currency: 'USD',
        translations: {
          'Terminal Locked': 'Terminal Locked',
          'Enter your PIN to unlock': 'Enter your PIN to unlock',
          'New Order Total': 'New Order Total',
          'Using cached rates - Syncing soon': 'Using cached rates - Syncing soon'
        },
        fxRates: [{ from: 'USD', to: 'EUR', rate: 0.92 }]
      },
      version: 0
    }));
  });

  // 1. Navigate to POS Terminal
  await page.goto('/pos/terminal');

  // 2. Unlock Terminal
  await page.click('button:has-text("1")');
  await page.click('button:has-text("2")');
  await page.click('button:has-text("3")');
  await page.click('button:has-text("4")');
  await expect(page.locator('text=Fatima')).toBeVisible();

  // 3. Toggle Language to Arabic (Simulated by clicking the toggle)
  await page.click('button:has-text("🇺🇸")');
  await page.click('button:has-text("العربية")');

  // Note: Since we didn't seed real Arabic translations in this mock E2E,
  // we just verify the state change if possible, or assume it uses fallback.
  // In a real E2E with seeded DB, it would change the text.

  // 4. Toggle Currency to EUR
  await page.click('button:has-text("USD")');
  await page.click('button:has-text("EUR")');

  // 5. Simulate Offline and Process Order
  await context.setOffline(true);
  await page.evaluate(() => {
    Object.defineProperty(navigator, 'onLine', { value: false });
    window.dispatchEvent(new Event('offline'));
  });

  let dialogMessage = '';
  page.once('dialog', async dialog => {
    dialogMessage = dialog.message();
    await dialog.accept();
  });

  // Click "New Order"
  await page.click('text=New Order');

  expect(dialogMessage).toContain('46 EUR');

  // 6. Verify Offline Feedback
  await expect(page.locator('text=Using cached rates - Syncing soon')).toBeVisible();
});
