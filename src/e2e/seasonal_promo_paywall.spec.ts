import { test, expect } from './fixtures';

test.describe('Viral Trial Extension Soft Paywall Loop on Seasonal Promo', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to seasonal promo screen where the growth loop is implemented
    await page.goto('/seasonal-promo');
    await page.waitForLoadState('networkidle');
  });

  test('should display soft paywall when generating promo without Pro, and extend trial upon sharing', async ({ page, context }) => {
    // 1. Fill out the form
    await page.locator('#promo-occasion').fill('Winter Wonderland');
    await page.locator('#promo-discount').fill('25');

    // 2. Click to trigger the Pro feature action which should hit the soft paywall
    const generateBtn = page.getByRole('button', { name: 'Generate Campaign' });
    await expect(generateBtn).toBeVisible();
    await generateBtn.click();

    // 3. Verify the soft paywall modal appears
    const paywallHeading = page.getByRole('heading', { name: 'Upgrade to Pro' });
    await expect(paywallHeading).toBeVisible();
    await expect(page.getByText('Seasonal Promotion Generator is a Pro feature')).toBeVisible();

    // 4. Locate the "Share on X to get 7 Days Free" button
    const shareBtn = page.getByRole('button', { name: 'Share on X to get 7 Days Free' });
    await expect(shareBtn).toBeVisible();

    // 5. Mock the alert that is expected to show upon claiming trial extension
    const dialogPromise = page.waitForEvent('dialog');

    // 6. We also want to intercept the window.open call which shares to Twitter
    // Wait for the new page to be opened
    const pagePromise = context.waitForEvent('page');

    // Click the share button to claim trial
    await shareBtn.click();

    // Verify window.open opened a Twitter intent URL with referral link
    const newPage = await pagePromise;
    await newPage.waitForLoadState();
    expect(newPage.url()).toContain('twitter.com/intent/tweet');
    expect(newPage.url()).toContain('ohc://join?ref=');
    await newPage.close();

    // Verify the alert message
    const dialog = await dialogPromise;
    expect(dialog.message()).toContain('Your 7-day Pro trial has been activated.');
    await dialog.accept();

    // 7. Verify soft paywall is closed
    await expect(paywallHeading).toBeHidden();

    // 8. Verify the original action (generate campaign) resumed and completed
    const resultCard = page.locator('#promo-result');
    await expect(resultCard).toBeVisible({ timeout: 5000 });

    const resultText = await resultCard.textContent();
    expect(resultText).toContain('Winter Wonderland Special!');
    expect(resultText).toContain('25% OFF');
  });
});
