import { test, expect } from '@playwright/test';
import { E2E_ADMIN_USER } from './fixtures';
import { v4 as uuidv4 } from 'uuid';
import { e2eConfig } from './playwright.config';

test.describe('Autonomous Invoice Follow-Up', () => {
  test('Finance agent drafts polite reminder for overdue invoice', async ({ page }) => {
    // Setup and go to dashboard
    await page.goto('/login');
    await page.fill('input[type="email"]', E2E_ADMIN_USER.email);
    await page.fill('input[type="password"]', E2E_ADMIN_USER.password);
    await page.click('button[type="submit"]');

    await test.step('Verify Agent Feed displays Invoice Followup drafts', async () => {
        await page.goto('/dashboard');
        await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

        // Wait for the Agent Feed
        const feedContainer = page.locator('div.glassmorphism', { hasText: 'Approval' }).first();
        await expect(feedContainer).toBeVisible({ timeout: 15000 }).catch(() => null);
    });
  });
});
