import { test, expect } from './fixtures';

test.describe('Dashboard Cleanup Audit', () => {
  test('Verify absence of PRO badge in Advanced AI Automations card', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');
    const heading = page.locator('h2', { hasText: 'Advanced AI Automations' });
    if(await heading.count() > 0) {
      await expect(heading).toBeVisible();
      await expect(heading).not.toContainText('PRO');
    }
  });

  test('Verify absence of Failed to load time savings data error', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');
    await expect(page.locator('text="Failed to load time savings data."')).toHaveCount(0);
  });

  test('Verify walkthrough.js is not loaded', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    let walkthroughLoaded = false;
    page.on('request', request => {
      if (request.url().includes('walkthrough.js')) {
        walkthroughLoaded = true;
      }
    });
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');
    expect(walkthroughLoaded).toBe(false);
  });

  test('Verify help-chat.js is not loaded', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    let helpChatLoaded = false;
    page.on('request', request => {
      if (request.url().includes('help-chat.js')) {
        helpChatLoaded = true;
      }
    });
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');
    expect(helpChatLoaded).toBe(false);
  });

  test('Verify tooltip.js is not loaded', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    let tooltipLoaded = false;
    page.on('request', request => {
      if (request.url().includes('tooltip.js')) {
        tooltipLoaded = true;
      }
    });
    await page.goto('/setup');
    await page.waitForLoadState('networkidle');
    expect(tooltipLoaded).toBe(false);
  });
});
