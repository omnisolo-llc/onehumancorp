import { test, expect } from '@playwright/test';

test.describe('Unified Agent Dashboard', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('displays real cross-agent actions without mock data and correctly handles mobile viewport', async ({ page, request }) => {
    // Navigate to the dashboard
    await page.goto('/dashboard');

    // Wait for the page to load
    await page.waitForLoadState('networkidle');

    // The feed should be present and visible
    const feedSection = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feedSection).toBeVisible();

    // Ensure there is no horizontal scroll on the body
    const isScrollable = await page.evaluate(() => {
      return document.documentElement.scrollWidth > document.documentElement.clientWidth;
    });
    expect(isScrollable).toBeFalsy();

    // Trigger the real-time event by inserting it via backend API
    await request.post('/api/agents/approvals/simulate-quote-draft', {
      headers: {
        'x-tenant-id': 'default'
      }
    });

    // Wait for the real-time API SSE event to be processed and added to proposals naturally
    await expect(page.locator('text=Draft quote for Plumbing Fix')).toBeVisible({ timeout: 15000 });

    // Switch to Activity tab
    await page.click('button:has-text("Activity Feed")');

    // Check that we see the correct UI layout
    const feedContainer = page.locator('.flex.flex-col.gap-3.min-w-\\[320px\\]');
    await expect(feedContainer).toBeVisible();

    // Ensure there are no hardcoded [10:45 AM] Sandbox memory limit exceeded mocks in the audit view
    await page.goto('/agent-audit-dashboard');
    await expect(page.locator('text=Sandbox memory limit exceeded')).not.toBeVisible();
    await expect(page.locator('text=Cross-Agent Feed')).toBeVisible();
  });
});
