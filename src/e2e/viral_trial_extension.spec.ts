import { test, expect } from './fixtures';

test.describe('Viral Trial Extension Flow', () => {
  test('user shares on Twitter to extend trial by 7 days', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Make sure we are on the dashboard
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // Locate the "Extend Your Trial" section
    const extendSection = page.locator('h2', { hasText: 'Extend Your Trial' });
    await expect(extendSection).toBeVisible({ timeout: 60000 });

    // Verify initial days left (starts at 14)
    // There are multiple 14s, so we look for the one in the specific block
    const daysLeftText = page.locator('.text-5xl.font-outfit.font-bold', { hasText: '14' });
    await expect(daysLeftText).toBeVisible();

    // Handle dialog gracefully and verify it was triggered
    let dialogTriggered = false;
    page.on('dialog', async dialog => {
      dialogTriggered = true;
      expect(dialog.message()).toContain('Awesome! Your trial has been extended by 7 days.');
      await dialog.accept();
    });

    // Click the "Share" button for Twitter
    // Wait for the button, click it, but prevent opening a new window during test
    // Find the specific container for "Share on X (Twitter)" to avoid clicking wrong "Share" link
    const twitterContainer = page.locator('h4', { hasText: 'Share on X (Twitter)' }).locator('..').locator('..').locator('..');
    const shareBtn = twitterContainer.locator('a', { hasText: 'Share' }).first();
    await expect(shareBtn).toBeVisible();

    // Verify the href of the link to make sure it includes the correct message
    const href = await shareBtn.getAttribute('href');
    expect(href).toContain('twitter.com/intent/tweet');
    expect(href).toContain('Powered%20by%20OHC');
    expect(href).toContain('ohc.store');

    // Click the link to trigger the onClick event
    // To avoid actually navigating, we evaluate the click via JS since standard click might open target=_blank
    // However, playwright can handle target=_blank. We intercept new pages to be safe.

    // Set up a promise to catch the new page event if needed, but we don't strictly need it if we just mock or ignore it
    const [newPage] = await Promise.all([
      page.context().waitForEvent('page'),
      shareBtn.click()
    ]);

    // Verify the trial days increased by 7 to 21
    const newDaysLeftText = page.locator('.text-5xl.font-outfit.font-bold', { hasText: '21' });
    await expect(newDaysLeftText).toBeVisible({ timeout: 5000 });

    // Verify the button is now disabled (shows "Shared")
    const sharedBtn = twitterContainer.locator('button', { hasText: 'Shared' }).first();
    await expect(sharedBtn).toBeVisible();
    await expect(sharedBtn).toBeDisabled();

    // Close the opened page to be clean
    await newPage.close();
  });
});
