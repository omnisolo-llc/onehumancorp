import { test, expect } from '@playwright/test';

test.describe('Instant Quote Client Evaluation', () => {
  test('Customer selects options and sees instant price update', async ({ page }) => {
    // 1. Navigate to the instant quote page
    await page.goto('/instant-quote');

    // 2. Wait for loading to finish and rules to be populated
    await expect(page.getByTestId('service-select')).toBeVisible();

    // 3. Initially, no price should be shown
    await expect(page.getByTestId('estimated-price')).toHaveText('--');

    // 4. Select a service (from mock data: cake_delivery, base 5000 cents = $50.00)
    await page.getByTestId('service-select').selectOption('cake_delivery');

    // 5. Verify the base price updates instantly
    await expect(page.getByTestId('estimated-price')).toHaveText('$50.00');

    // 6. Check 'Rush Delivery' (mock adds 1500 cents = $15.00)
    await page.getByTestId('rush-checkbox').check();
    await expect(page.getByTestId('estimated-price')).toHaveText('$65.00');

    // 7. Check 'Weekend Service' (mock adds 20% to $65.00 = $13.00, total $78.00)
    await page.getByTestId('weekend-checkbox').check();
    await expect(page.getByTestId('estimated-price')).toHaveText('$78.00');

    // 8. Uncheck 'Rush Delivery' (base $50 + 20% = $60.00)
    await page.getByTestId('rush-checkbox').uncheck();
    await expect(page.getByTestId('estimated-price')).toHaveText('$60.00');

    // 9. Request the quote
    // Setup dialog listener to handle the alert
    page.on('dialog', dialog => dialog.accept());
    await page.getByTestId('request-quote-btn').click();

    // 10. Verify navigation back to home or appropriate screen
    await expect(page).toHaveURL('/');
  });
});
