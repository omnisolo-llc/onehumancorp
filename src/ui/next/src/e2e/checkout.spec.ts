import { test, expect } from '@playwright/test';

test.describe('Checkout - Offline Multi-Currency', () => {
  test('should handle offline tap to pay with dynamic currency and i18n', async ({ page, context }) => {
    await page.goto('/');
    await page.evaluate(() => window.localStorage.clear());

    await page.goto('/checkout');

    await page.route('**/api/v1/i18n/cache', async (route) => {
        await route.fulfill({
            status: 200,
            json: {
                translations: {
                    en: { 'checkout.title': 'Checkout', 'checkout.subtitle': 'Please enter your payment details below.', 'checkout.pay_now': 'Pay Now', 'checkout.offline_warning': 'Converted using yesterday rate. Will finalize on sync.' },
                    es: { 'checkout.title': 'Pagar', 'checkout.subtitle': 'Ingrese sus detalles de pago a continuación.', 'checkout.pay_now': 'Pagar ahora', 'checkout.offline_warning': 'Estás desconectado.' },
                    ar: { 'checkout.title': 'الدفع', 'checkout.subtitle': 'الرجاء إدخال تفاصيل الدفع أدناه.', 'checkout.pay_now': 'ادفع الآن', 'checkout.offline_warning': 'أنت غير متصل بالإنترنت.' }
                },
                exchange_rates: {
                    USD: 1.0,
                    EUR: 0.92
                }
            }
        })
    });

    await page.reload();

    await expect(page.locator('h1')).toHaveText('Checkout');

    await page.waitForSelector('select');
    await page.locator('select').first().selectOption('es', { force: true });
    await expect(page.locator('h1')).toHaveText('Pagar');
    await expect(page.getByRole('button', { name: 'Pagar ahora' })).toBeVisible();

    await page.locator('select').nth(1).selectOption('EUR');

    await context.setOffline(true);

    let alertFired = false;
    page.on('dialog', async dialog => {
      if (dialog.type() === 'prompt') {
        await dialog.accept('50');
      } else {
        expect(dialog.message()).toContain('Estás desconectado');
        alertFired = true;
        await dialog.accept();
      }
    });

    await page.getByRole('button', { name: 'Tap to Pay' }).click();
    await page.waitForTimeout(500);
    expect(alertFired).toBeTruthy();

    await context.setOffline(false);

    await page.goto('/dashboard');
    const queue = await page.evaluate(() => window.localStorage.getItem('ohc_offline_queue'));
    expect(queue).toContain('"currency":"EUR"');
    expect(queue).toContain('"exchange_rate":0.92');
  });
});
