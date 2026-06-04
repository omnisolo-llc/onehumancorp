import { test, expect } from '@playwright/test';

test.describe('Dashboard Actionable Feed', () => {
  test('should display database-backed operations console', async ({ page }) => {
    await page.goto('http://localhost:3000/dashboard');

    await expect(page.locator('text="Business Analytics"')).toBeVisible();
    await expect(page.locator('text="Operations Map"')).toBeVisible();
    await expect(page.locator('text="Action Required"')).toBeVisible();
    await expect(page.locator('text="Recent Orders"')).toBeVisible();
    await expect(page.locator('text="Inbox Activity"')).toBeVisible();
  });

  test('should display and interact with the Unified Agent Feed', async ({ page }) => {
    await page.goto('http://localhost:3000/unified-agent-feed');

    // Verify header
    await expect(page.locator('text="Unified Agent Feed"')).toBeVisible();

    // Wait for the feed to load
    await page.waitForResponse(response => response.url().includes('/api/agents/feed') && response.status() === 200);

    // Verify operations urgent item
    await expect(page.locator('text="Operations Agent"')).toBeVisible();
    await expect(page.locator('text="3 new orders to fulfill."')).toBeVisible();

    const fulfillNowBtn = page.locator('button', { hasText: 'Fulfill Now' });
    await expect(fulfillNowBtn).toBeVisible();

    // Verify advisory proposal item
    await expect(page.locator('text="Advisory Agent"')).toBeVisible();
    await expect(page.locator('text="It\'s been 30 days since your last promo. Should I draft an email?"')).toBeVisible();

    // Interact with advisory proposal
    const draftEmailBtn = page.locator('button', { hasText: 'Yes, draft it' });
    await expect(draftEmailBtn).toBeVisible();
    await draftEmailBtn.click();

    // Verify that draft content expands and button changes to "Approve & Send"
    await expect(page.locator('text="Subject: We Miss You! Here\'s 20% Off"')).toBeVisible();
    const approveAndSendBtn = page.locator('button', { hasText: 'Approve & Send' });
    await expect(approveAndSendBtn).toBeVisible();

    // Click approve and send, verifying the card disappears from feed
    await approveAndSendBtn.click();
    await expect(page.locator('text="It\'s been 30 days since your last promo. Should I draft an email?"')).not.toBeVisible();
  });
});
