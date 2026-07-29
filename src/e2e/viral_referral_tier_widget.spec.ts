import { test as base, expect } from './fixtures';

const test = base.extend({
  page: async ({ page }, use) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await use(page);
  }
});

test.describe('Viral Referral Tier Widget', () => {
  test('displays correctly and handles copy/share', async ({ page, loginAs, adminUser }) => {
    // Note: Do not test the route, we must use real backend services! E2E rules explicitly forbid this.
    await loginAs(page, adminUser);

    const widget = page.getByTestId('referral-tier-widget');
    await expect(widget).toBeVisible({ timeout: 15000 });

    await expect(page.locator('#referral-tier-status')).toContainText('You are on the Bronze Tier.');
    await expect(page.locator('#referral-tier-progress')).toContainText('Total Conversions: 0');
    await expect(page.locator('#referral-tier-progress')).toContainText('Just 5 more referrals needed for Silver!');

    const linkInput = page.locator('#referral-link-input');
    await expect(linkInput).toHaveValue(/ohc\.app\/join\?ref=|ohc:\/\/join\?ref=/);

    const copyBtn = page.locator('#referral-tier-copy-btn');
    await expect(copyBtn).toHaveText('Copy Link');

    const shareXBtn = page.locator('#referral-tier-share-x-btn');
    await expect(shareXBtn).toHaveText('Share on X');
  });
});
