import { test, expect } from './fixtures';

test.describe('Agent Audit Dashboard E2E Extra Tests', () => {
  test('should display Cost Tracker correctly', async ({ page }) => {
    await page.goto('/');
    await page.locator('nav a:has-text("Inbox")').click();
    await page.click('button[aria-label="Agent Audit Dashboard"], [title="Agent Audit Dashboard"]');
    await expect(page.locator('text=Cost Tracker')).toBeVisible();
    await expect(page.locator('text=Total organizational spend')).toBeVisible();
  });

  test('should display Operations correctly', async ({ page }) => {
    await page.goto('/');
    await page.locator('nav a:has-text("Inbox")').click();
    await page.click('button[aria-label="Agent Audit Dashboard"], [title="Agent Audit Dashboard"]');
    await expect(page.locator('text=Operations')).toBeVisible();
    await expect(page.locator('text=Agent Health: Optimal')).toBeVisible();
  });

  test('should display Marketing & Advertising correctly', async ({ page }) => {
    await page.goto('/');
    await page.locator('nav a:has-text("Inbox")').click();
    await page.click('button[aria-label="Agent Audit Dashboard"], [title="Agent Audit Dashboard"]');
    await expect(page.locator('text=Marketing & Advertising')).toBeVisible();
    await expect(page.locator('text=Campaigns Sync: Active')).toBeVisible();
  });

  test('should display Violation Feed correctly', async ({ page }) => {
    await page.goto('/');
    await page.locator('nav a:has-text("Inbox")').click();
    await page.click('button[aria-label="Agent Audit Dashboard"], [title="Agent Audit Dashboard"]');
    await expect(page.locator('text=Violation Feed')).toBeVisible();
    await expect(page.locator('text=Sandbox memory limit exceeded in Agent #452')).toBeVisible();
  });

  test('should go back to inbox', async ({ page }) => {
    await page.goto('/');
    await page.locator('nav a:has-text("Inbox")').click();
    await page.click('button[aria-label="Agent Audit Dashboard"], [title="Agent Audit Dashboard"]');
    await page.locator('text=Back to Inbox').click();
    await expect(page.locator('text=Unified Inbox')).toBeVisible();
  });
});
