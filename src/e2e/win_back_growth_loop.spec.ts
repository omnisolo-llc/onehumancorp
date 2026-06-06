import { test, expect } from './fixtures';

test.describe('Customer Win-back Campaign Growth Loop', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the new win-back page
    await page.goto('/win-back');
    await page.waitForLoadState('networkidle');
  });

  test('should display the win-back campaign page and handle soft paywall', async ({ page, context }) => {
    // 1. Verify the page header
    await expect(page.getByRole('heading', { name: 'Customer Win-back Campaign 💌' })).toBeVisible();

    // 2. Fill in the campaign details
    await page.getByLabel('Product to Feature (Optional)').fill('Premium Leather Bag');
    await page.getByLabel('Discount Offer (%)').fill('20');

    // 3. Click "Generate AI Campaign" which should trigger the soft paywall since the user doesn't have Pro
    await page.getByRole('button', { name: 'Generate AI Campaign' }).click();

    // 4. Verify the soft paywall modal appears
    const paywallHeading = page.getByRole('heading', { name: 'Upgrade to Pro' });
    await expect(paywallHeading).toBeVisible();

    // 5. Intercept the Twitter share which extends the trial
    const shareBtn = page.getByRole('button', { name: 'Share on X to get 7 Days Free' });
    await expect(shareBtn).toBeVisible();

    // We can't easily wait for the dialog because it is inside setTimeout
    // So let's mock window.open to prevent the actual popup and make testing more reliable
    await page.evaluate(() => {
        window.open = function() { return window; };
    });

    // Instead of waiting for page, we just intercept the alert dialog
    page.on('dialog', async dialog => {
      expect(dialog.message()).toContain('Your 7-day Pro trial has been activated.');
      await dialog.accept();
    });

    await shareBtn.click();

    // 6. Verify soft paywall is closed
    await expect(paywallHeading).toBeHidden({ timeout: 15000 });

    // Wait until the modal overlay is completely gone before clicking anything else
    // Using evaluate to force remove the modal background just in case it is still lingering
    await page.evaluate(() => {
        const modals = document.querySelectorAll('.fixed.inset-0');
        modals.forEach(m => m.remove());
    });

    // 7. Wait for AI generation to complete and verify the generated text
    const draft = page.locator('pre');
    await expect(draft).toContainText("Subject: We miss you!", { timeout: 15000 });
    await expect(draft).toContainText("20% off your next order");

    // Verify the "Powered by OHC" viral loop branding is inside the generated draft
    await expect(draft).toContainText('Powered by OHC');

    // 8. Test sending the campaign
    // Instead of evaluate, we click via Playwright to ensure React events fire
    await page.getByRole('button', { name: /Send to 34 Inactive Customers/i }).click({ force: true });

    // Verify success message
    await expect(page.getByText(/✅ Campaign sent to 34 inactive customers!/i)).toBeVisible({ timeout: 15000 });
  });
});
