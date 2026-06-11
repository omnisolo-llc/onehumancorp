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

    // mock the tauri core invoke for 'generate_cloud_bridge_invite'
    await page.evaluate(() => {
        window.__TAURI__ = {
            core: {
                invoke: async (cmd) => {
                    if (cmd === 'generate_cloud_bridge_invite') {
                        return 'https://cloud.ohc.network/invite/mocked-link-123';
                    }
                    return null;
                }
            }
        };
    });

    await inviteBtn.click();

    const linkInput = page.locator('#dashboard-invite-link');
    await expect(linkInput).toBeVisible();
    await expect(linkInput).toHaveValue('https://cloud.ohc.network/invite/mocked-link-123');
  });

  test('should copy generated link to clipboard', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    await page.evaluate(() => {
        window.__TAURI__ = {
            core: {
                invoke: async (cmd) => {
                    if (cmd === 'generate_cloud_bridge_invite') {
                        return 'https://cloud.ohc.network/invite/mocked-link-123';
                    }
                    return null;
                }
            }
        };
    });

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
        window.__TAURI__ = {
            core: {
                invoke: async (cmd) => {
                    if (cmd === 'generate_cloud_bridge_invite') {
                        return 'https://cloud.ohc.network/invite/mocked-link-123';
                    }
                    return null;
                }
            }
        };
        // Mock window.open
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
    expect(lastOpenedUrl).toContain('mocked-link-123');
  });

  test('should fallback to default link if generate_cloud_bridge_invite fails', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    await page.evaluate(() => {
        window.__TAURI__ = {
            core: {
                invoke: async (cmd) => {
                    if (cmd === 'generate_cloud_bridge_invite') {
                        throw new Error("Failed");
                    }
                    return null;
                }
            }
        };
    });

    await page.locator('#dashboard-invite-btn').click();

    const linkInput = page.locator('#dashboard-invite-link');
    await expect(linkInput).toBeVisible();
    await expect(linkInput).toHaveValue('https://cloud.ohc.network/invite/fallback');
  });
});
