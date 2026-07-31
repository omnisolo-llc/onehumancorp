import { test, expect } from '@playwright/test';

test.describe('Omni Inbox Agentic Triage', () => {
  test('displays unread leads summary', async ({ page }) => {
    await page.goto('/login');
  });
});
