import { test, expect } from './fixtures';

test.describe('Customer Referral Program Growth Loop', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the new customer referrals page
    await page.goto('/customer-referrals');
    await page.waitForLoadState('networkidle');
  });

  test('should display the customer referral program page and handle soft paywall and AI draft generation', async ({ page, context }) => {
    test.setTimeout(90000);

    // 1. Wait for page to fully load - wait for the header specifically, avoiding wait for navigation failure if possible
    await expect(page.locator('h1').filter({ hasText: 'Customer Referral Program 🚀' })).toBeVisible({ timeout: 20000 });

    // 2. Fill in the program details
    await page.getByLabel('Store Name (Optional)').fill('Maya Cakes');
    await page.getByLabel('Give Discount (%)').fill('20');
    await page.getByLabel('Get Reward ($)').fill('20');

    // 3. Click "Generate AI Campaign" which should trigger the soft paywall since the user doesn't have Pro
    await page.getByRole('button', { name: 'Generate AI Campaign' }).click();

    // 4. Verify the soft paywall modal appears
    const paywallHeading = page.getByRole('heading', { name: 'Upgrade to Pro' });
    await expect(paywallHeading).toBeVisible({ timeout: 15000 });

    // 5. Intercept the Twitter share which extends the trial
    const shareBtn = page.getByRole('button', { name: 'Share on X to get 7 Days Free' });
    await expect(shareBtn).toBeVisible({ timeout: 15000 });

    // Mock window.open to prevent the actual popup and make testing more reliable
    await page.evaluate(() => {
        window.open = function() { return window; };
    });

    // Accept the alert dialog gracefully
    page.on('dialog', async dialog => {
      expect(dialog.message()).toContain('Your 7-day Pro trial has been activated.');
      await dialog.accept();
    });

    await shareBtn.click();

    // 6. Verify soft paywall is automatically closed via the natural UI flow
    await expect(page.locator('[data-testid="soft-paywall-modal"]')).toBeHidden({ timeout: 15000 });

    // 7. Wait for AI generation to complete and verify the generated text
    // We expect the button to transition back from 'Drafting...' to the original text
    await expect(page.getByRole('button', { name: 'Generate AI Campaign' })).toBeEnabled({ timeout: 15000 });

    await expect(page.locator('pre')).toContainText("Maya Cakes", { timeout: 15000 });
    await expect(page.locator('pre')).toContainText("VIP Referral Program", { timeout: 15000 });

    // Verify the "Powered by OHC" viral loop branding is inside the generated draft
    await expect(page.locator('pre')).toContainText('⚡ Powered by OHC', { timeout: 15000 });

    // 8. Test sending the campaign
    const sendBtn = page.locator('button', { hasText: 'Launch Program to' });

    // Check if the button is enabled (customerCount > 0 from the simulated fetch)
    if (await sendBtn.isEnabled()) {
        await sendBtn.click();
        await expect(page.getByText(/✅ Referral program launched to/i)).toBeVisible({ timeout: 15000 });
    }
  });
});
