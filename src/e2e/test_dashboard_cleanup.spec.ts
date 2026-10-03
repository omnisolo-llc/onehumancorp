import { test, expect } from './fixtures';

test.describe('Dashboard Cleanup Audit', () => {
  test('Verify absence of PRO badge in Advanced AI Automations card', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    await page.waitForTimeout(3000);
    const heading = page.locator('h2', { hasText: 'Advanced AI Automations' });
    if(await heading.count() > 0) {
      await expect(heading).toBeVisible();
      await expect(heading).not.toContainText('PRO');
    }
  });

  test('Verify absence of Failed to load time savings data error', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    await page.waitForTimeout(3000);
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
    await page.waitForTimeout(3000);
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
    await page.waitForTimeout(3000);
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
    await page.goto('/dashboard');
    await page.waitForTimeout(3000);
    expect(tooltipLoaded).toBe(false);
  });

  for (const route of ['/dashboard.html', '/ui/dashboard.html']) {
    test(`${route} keeps navigation and Cloud Bridge visible when recorded savings is unavailable`, async ({ page, loginAs, unlimitedAdminUser }) => {
      await loginAs(page, unlimitedAdminUser);
      const savingsRead = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/growth/time-savings');
      await page.goto(route);
      const response = await savingsRead;
      expect(response.status()).toBe(501);
      expect(await response.json()).toMatchObject({ success: false, capability: 'measured_time_savings' });
      const widget = page.locator('#ai-savings-widget');
      await expect(widget).toBeVisible();
      await expect(widget.locator('..')).toBeVisible();
      await expect(page.locator('#ai-savings-title')).toHaveText('Recorded time savings');
      await expect(page.locator('#ai-savings-desc')).toHaveText('Recorded time-savings data is unavailable.');
      await expect(page.locator('#generate-cloud-bridge-btn')).toBeVisible();
      await expect(page.locator('#cloud-bridge-email')).toBeVisible();
      await expect(page.locator('a[href="booking-dashboard.html"]').first()).toBeVisible();
      await expect(widget.getByRole('link', { name: 'Check current plan and trial availability' })).toHaveAttribute('href', '/trial-extension');
      await expect(widget.getByText(/You saved 0|7 Days Pro|Trial Extended/)).not.toBeVisible();
    });
  }
});
