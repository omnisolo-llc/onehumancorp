import { test, expect } from './fixtures';

test.describe('In-Person Payment (POS) Flow', () => {
  test('should complete a tap-to-pay transaction', async ({ page }) => {
    // Start from pricing/plans and navigate to checkout
    await page.goto('/pricing');

    const getStartedBtn = page.getByRole('button', { name: /Get Started/i }).first();
    await getStartedBtn.click();

    // Fill delivery address if present (forces the checkout flow to load)
    const addressInput = page.getByRole('textbox', { name: /Delivery/i });
    if (await addressInput.isVisible()) {
        await addressInput.fill("123 Fake St");
        await page.getByRole('button', { name: /Check Delivery/i }).click();
    }

    const tapToPayBtn = page.getByRole('button', { name: /Tap to Pay/i });
    await tapToPayBtn.click();

    // Find Stripe terminal connect button (which indicates terminal client loaded and discovered readers)
    const connectBtn = page.getByRole('button', { name: /Connect/i }).first();
    await connectBtn.click();

    const chargeBtn = page.getByRole('button', { name: /Charge/i });
    await chargeBtn.click();

    // Look for success confirmation
    await expect(page.getByText(/Payment Successful/i)).toBeVisible({ timeout: 15000 });

    // Dismiss the success modal to navigate back to dashboard
    const continueBtn = page.getByRole('button', { name: /Continue to Dashboard/i });
    if (await continueBtn.isVisible()) {
      await continueBtn.click();
    } else {
      await page.goto('/dashboard');
    }

    // Navigate to Finance
    await page.getByRole('link', { name: /Finance/i }).click();

    // Verify the transaction appears. We just verify that we landed on Finance.
    await expect(page.getByRole('heading', { name: /Finance/i })).toBeVisible();
  });
});
