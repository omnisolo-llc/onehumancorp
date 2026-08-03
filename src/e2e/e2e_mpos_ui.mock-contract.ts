import { test, expect } from './fixtures';

test.describe('mPOS Premium UI (Glassmorphism, Responsive)', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('mPOS UI shows glassmorphism styles, works offline, shows pulsing NFC and handles payment flow', async ({ memberPage, context, request }) => {
    // 1. Get token
    const authRes = await request.post('/api/v1/auth/login', {
        data: {
            email: 'test@example.com', // Using standard E2E_ADMIN_USER
            password: 'password123'
        }
    });
    const { token } = await authRes.json();

    const productId = `prod-mpos-ui-${Date.now()}`;

    // Navigate to mPOS URL
    await memberPage.goto('/pos/mpos?tenantId=tenant_mpos_ui_test');

    // Wait for the UI to load
    await expect(memberPage.locator('h1').filter({ hasText: 'mPOS' })).toBeVisible({ timeout: 15000 });

    // Test that the layout uses translucent glassmorphism tokens
    const header = memberPage.locator('header');
    await expect(header).toHaveClass(/backdrop-blur/);
    await expect(header).toHaveClass(/bg-\[rgba\(255,255,255,0\.65\)\]/);

    // Go offline
    await context.setOffline(true);

    // Check if offline badge shows
    await expect(memberPage.locator('text=Offline Mode')).toBeVisible({ timeout: 5000 });

    // Click the first product
    await memberPage.locator('text=Mock Croissant').first().click();

    // Verify the cart updates
    await expect(memberPage.locator('text=1 Items')).toBeVisible();

    // Click "Charge"
    const chargeBtn = memberPage.getByTestId('mpos-quick-charge');
    await expect(chargeBtn).not.toBeDisabled();
    await chargeBtn.click();

    // The "Tap to Pay" Payment sheet should slide up
    const tapToPayHeader = memberPage.locator('h2').filter({ hasText: 'Tap to Pay' });
    await expect(tapToPayHeader).toBeVisible();

    // Verify pulsing NFC UI elements exist
    const pulseElement = memberPage.locator('.animate-pulse');
    await expect(pulseElement).toBeVisible();

    // The bottom sheet itself has glassmorphism styles
    const bottomSheet = memberPage.locator('.animate-slide-up');
    await expect(bottomSheet).toHaveClass(/backdrop-blur-\[40px\]/);

    // Click "Cancel" to dismiss the payment sheet
    const cancelBtn = memberPage.locator('button').filter({ hasText: 'Cancel' }).first();
    await cancelBtn.click();

    // Sheet should close
    await expect(tapToPayHeader).not.toBeVisible();

    // Click charge again to simulate successful payment
    await chargeBtn.click();
    await expect(tapToPayHeader).toBeVisible();

    // Handle the payment process through StripeTerminalClient mock
    const cashMethodBtn = memberPage.locator('button').filter({ hasText: 'Cash' });
    await expect(cashMethodBtn).toBeVisible({ timeout: 10000 });
    await cashMethodBtn.click();

    const recordCashBtn = memberPage.locator('button').filter({ hasText: 'Record Offline Cash Sale' });
    await expect(recordCashBtn).toBeVisible();
    await recordCashBtn.click();

    // Verify Success Screen Appears
    await expect(memberPage.locator('text=Payment Successful')).toBeVisible({ timeout: 15000 });
    await expect(memberPage.locator('text=Thank you for your purchase.')).toBeVisible();
    await expect(memberPage.locator('text=Email Receipt')).toBeVisible();

    // Verify AI Assistant suggestion
    await expect(memberPage.locator('text=AI Assistant Suggestion')).toBeVisible();

    // End flow
    await memberPage.locator('button').filter({ hasText: 'No Receipt' }).click();

    // Sheet closed, Cart empty
    await expect(memberPage.locator('text=Payment Successful')).not.toBeVisible();
    await expect(memberPage.locator('text=0 Items')).toBeVisible();

    // Clean up
    await context.setOffline(false);
  });
});
