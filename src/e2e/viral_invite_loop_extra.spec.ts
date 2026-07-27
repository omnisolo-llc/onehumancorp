import { test, expect } from './fixtures';

test.describe('Viral Invite Loop on Dashboard Page', () => {
  test('should display Invite & Earn section', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    await expect(page.getByRole('heading', { name: 'Invite & Earn' })).toBeVisible();
    await expect(page.getByText('They get 1 month free, you get $50 credit.')).toBeVisible();
  });

  test('should show generate link button and it generates link', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    const inviteBtn = page.locator('#dashboard-invite-btn');
    await expect(inviteBtn).toBeVisible();

    await inviteBtn.click();

    const linkInput = page.locator('#dashboard-invite-link');
    await expect(linkInput).toBeVisible();
    await expect(linkInput).toHaveValue(/^https:\/\/ohc\.app\/invite\/.+/);
  });

  test('should copy generated link to clipboard', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    await page.locator('#dashboard-invite-btn').click();

    const copyBtn = page.locator('#dashboard-copy-btn');
    await expect(copyBtn).toBeVisible();

    // Test copy logic
    await copyBtn.click();
    await expect(copyBtn).toHaveText('Copied!');
  });

  test('should share generated link on X (Twitter)', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    await page.evaluate(() => {
        // test window.open to avoid actually opening twitter in tests
        window.open = function(url, target) {
            window.lastOpenedUrl = url;
            return window;
        };
    });

    await page.locator('#dashboard-invite-btn').click();

    const shareXBtn = page.locator('#dashboard-share-x-btn');
    await expect(shareXBtn).toBeVisible();
    await shareXBtn.click();

    // Verify window.open was called with twitter intent
    const lastOpenedUrl = await page.evaluate(() => window.lastOpenedUrl);
    expect(lastOpenedUrl).toContain('twitter.com/intent/tweet');
    expect(lastOpenedUrl).toContain('ohc.app/invite');
  });
});
