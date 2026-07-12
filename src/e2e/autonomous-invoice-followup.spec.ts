import { test, expect } from '@playwright/test';
import { E2E_ADMIN_USER } from './fixtures';
import { v4 as uuidv4 } from 'uuid';
import { e2eConfig } from './playwright.config';

test.describe('Autonomous Invoice Drafting and Follow-Up', () => {
  test('Finance agent drafts invoice for completed milestone and follows up when overdue', async ({ page }) => {
    // Setup and go to dashboard
    await page.goto('/login');
    await page.fill('input[type="email"]', E2E_ADMIN_USER.email);
    await page.fill('input[type="password"]', E2E_ADMIN_USER.password);
    await page.click('button[type="submit"]');

    await test.step('Simulate Invoice Draft', async () => {
        await page.goto('/feed');
        await expect(page.locator('h1', { hasText: 'Feed' }).first()).toBeVisible({ timeout: 25000 });

        await page.click('[data-testid="simulate-invoice-draft-btn"]');
        await expect(page.getByText('INVOICE DRAFT')).toBeVisible({ timeout: 15000 });
        await expect(page.getByText('Website Redesign')).toBeVisible();
        await expect(page.getByText('Phase 1 Complete')).toBeVisible();
        await expect(page.getByText('$2500.00')).toBeVisible();
    });

    await test.step('Approve Invoice Draft', async () => {
        await page.locator('[data-testid="feed-approve-btn"]').first().click();
        await expect(page.getByText('INVOICE DRAFT')).toBeHidden({ timeout: 15000 });
    });

    await test.step('Simulate Invoice Followup', async () => {
        await page.click('[data-testid="simulate-invoice-followup-btn"]');
        await expect(page.getByText('ACTION REQUIRED')).toBeVisible({ timeout: 15000 });
        await expect(page.getByText('Overdue Invoice Detected')).toBeVisible();
    });

    await test.step('Approve Invoice Followup', async () => {
        await page.locator('[data-testid="feed-approve-btn"]').first().click();
        await expect(page.getByText('ACTION REQUIRED')).toBeHidden({ timeout: 15000 });
    });
  });
});
