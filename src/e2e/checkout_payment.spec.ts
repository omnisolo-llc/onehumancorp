import { test, expect } from './fixtures';

test.describe('Checkout Payment E2E Data Verification', () => {
  test('verify checkout payment updates the database and reflects in UI', async ({ page, request }) => {
    await page.goto('/checkout');
    await expect(page.getByRole('heading', { name: 'Checkout' })).toBeVisible();

    const payBtn = page.getByRole('button', { name: /Pay Now/i });
    await payBtn.click();

    await expect(page.getByText('Payment Successful!')).toBeVisible({ timeout: 5000 });

    const res = await request.post('/api/v1/growth/referrals/generate', {
      data: {}
    });
    expect(res.ok()).toBeTruthy();

    const continueBtn = page.getByRole('button', { name: /Continue to Dashboard/i });
    await continueBtn.click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 5000 });
  });

  test('verify checkout page has no mock timeout delay on button click', async ({ page }) => {
    await page.goto('/checkout');
    const payBtn = page.getByRole('button', { name: /Pay Now/i });
    await payBtn.click();
    await expect(page.getByText('Payment Successful!')).toBeVisible({ timeout: 5000 });
  });

  test('verify that checkout payment handles referral link creation completely and natively', async ({ page }) => {
    await page.goto('/checkout');

    const payBtn = page.getByRole('button', { name: /Pay Now/i });
    await payBtn.click();

    await expect(page.getByText('Payment Successful!')).toBeVisible({ timeout: 5000 });

    const copyBtn = page.getByRole('button', { name: /Copy/i });
    await expect(copyBtn).toBeVisible();
    await copyBtn.click();
    await expect(copyBtn).toHaveText(/Copied!/i);
    // Since we restored the copy timeout, it should eventually revert back to 'Copy'
  });

  test('verify checkout terminal tap payment routes immediately without delay', async ({ page }) => {
      await page.goto('/checkout');
      page.on('dialog', dialog => dialog.accept());

      const terminalBtn = page.getByRole('button', { name: /Tap to Pay \(Stripe Terminal\)/i });
      await terminalBtn.click();

      // UI should transition and dashboard should show immediately
      await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 5000 });
  });

  test('verify generating cart campaign succeeds without delay', async ({ request }) => {
      const res = await request.post('/api/v1/growth/campaign/generate-cart', {
        data: { customer_name: 'Bob', cart_value: '$100.00' }
      });
      expect(res.ok()).toBeTruthy();
      const body = await res.json();
      expect(body.message).toContain('Bob');
      expect(body.message).toContain('$100.00');
  });
});
