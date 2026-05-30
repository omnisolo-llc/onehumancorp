import { test, expect } from './fixtures';

test.describe('Viral Trial Extension Soft Paywall Loop', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to dashboard where the growth loop is implemented
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');
  });

  test('should display soft paywall when sending campaign without Pro, and extend trial upon sharing', async ({ page, context }) => {
    // 1. Locate the "Send AI Review Requests" button in the Growth loops section
    const sendCampaignBtn = page.getByRole('button', { name: /Send AI Review Requests/i });
    await expect(sendCampaignBtn).toBeVisible();

    // 2. Click to trigger the Pro feature action which should hit the soft paywall
    await sendCampaignBtn.click();

    // 3. Verify the soft paywall modal appears
    const paywallHeading = page.getByRole('heading', { name: 'Upgrade to Pro' });
    await expect(paywallHeading).toBeVisible();
    await expect(page.getByText('Unlock AI Business Insights')).toBeVisible();

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
    expect(newPage.url()).toContain('https://ohc.store/join?ref=');
    await newPage.close();

    // Verify the alert message
    const dialog = await dialogPromise;
    expect(dialog.message()).toContain('Your 7-day Pro trial has been activated.');
    await dialog.accept();

    // 7. Verify soft paywall is closed
    await expect(paywallHeading).toBeHidden();

    // 8. Verify the original action (send campaign) resumed and completed
    // After claiming trial, it automatically calls handleSendCampaign() again
    // which transitions through "Generating drafts..." and then displays success
    const successMessage = page.getByText('Campaign sent to 12 customers!');
    await expect(successMessage).toBeVisible({ timeout: 5000 });
  });
});
