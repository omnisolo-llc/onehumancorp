import { test, expect } from './fixtures';

test.describe('Business Analytics Share Loop E2E', () => {
  test('displays Share My Success button in Business Snapshot linking to Twitter with referral URL', async ({ page, context }) => {
    // 1. Navigate to the dashboard
    await page.goto('/dashboard');

    // 2. Wait for the page to load
    await page.waitForLoadState('networkidle');

    // 3. Look for the "Share My Success" button
    const shareBtn = page.getByRole('link', { name: 'Share My Success' });
    await expect(shareBtn).toBeVisible();

    // 4. Verify the link attributes
    await expect(shareBtn).toHaveAttribute('target', '_blank');
    await expect(shareBtn).toHaveAttribute('title', 'Share your success to earn $50 credit');

    // 5. Verify the link contains the Twitter intent URL with proper text and referral link
    const href = await shareBtn.getAttribute('href');
    expect(href).toContain('https://twitter.com/intent/tweet?text=');

    // The decoded text should contain the OHC pitch and the referral link.
    const decodedHref = decodeURIComponent(href || '');
    expect(decodedHref).toContain('I just made $0.00 today selling online with One Human Corp!');
    expect(decodedHref).toContain('ohc.store/join?ref=');
  });
});
