import { test, expect } from '@playwright/test';

test.describe('Real-Time Multilingual KDS & Pre-Order Engine', () => {
  test('should render KDS, toggle Arabic language, and handle offline queuing', async ({ page, context }) => {
    // Navigate to KDS View
    await page.goto('/kds');

    // Default language is English
    // Use first() to avoid strict mode violations if multiple matches are found (e.g. Next.js development overlay)
    await expect(page.locator('h1').filter({ hasText: 'Kitchen Display System' }).first()).toBeVisible();
    await expect(page.locator('#lang-toggle-btn')).toContainText('عربي');

    // Toggle to Arabic
    await page.click('#lang-toggle-btn');
    await expect(page.locator('h1').filter({ hasText: 'نظام عرض المطبخ' }).first()).toBeVisible();
    await expect(page.locator('#lang-toggle-btn')).toContainText('English');

    // Toggle back to English for easier assertions
    await page.click('#lang-toggle-btn');
    await expect(page.locator('h1').filter({ hasText: 'Kitchen Display System' }).first()).toBeVisible();

    // Go Offline
    await context.setOffline(true);
    await page.evaluate(() => {
      window.dispatchEvent(new Event('offline'));
    });

    // We verify the indicator has the correct color rather than text sibling selectors which can be tricky
    await expect(page.locator('#network-status-indicator')).toHaveClass(/bg-red-500/);

    // Mark 'Chicken Over Rice' as sold out
    const soldOutBtn = page.locator('#sold-out-toggle-item-chicken');
    await soldOutBtn.click();

    // Verify Optimistic UI update
    await expect(soldOutBtn).toContainText('Sold Out');
    await expect(soldOutBtn).toHaveClass(/bg-red-100/);

    // Verify Offline Sync Banner appears
    const queueBanner = page.locator('#queue-dashboard');
    await expect(queueBanner).toBeVisible();
    await expect(queueBanner).toContainText('1 Sync Pending');

    // Mark order '101' as Preparing
    const prepBtn = page.locator('#btn-prep-101');
    await prepBtn.click();

    // Optimistic UI updates
    const readyBtn = page.locator('#btn-ready-101');
    await expect(readyBtn).toBeVisible();
    await expect(readyBtn).toContainText('Ready');

    // Verify Offline Sync Banner increments
    await expect(queueBanner).toContainText('2 Sync Pending');

    // Go Online
    await context.setOffline(false);
    await page.evaluate(() => {
      window.dispatchEvent(new Event('online'));
    });

    // Verify Network Status indicator
    await expect(page.locator('#network-status-indicator')).toHaveClass(/bg-green-500/);

    // Wait for sync to complete and banner to disappear
import { test, expect } from '@playwright/test';

test.describe('Real-Time Multilingual KDS & Pre-Order Engine', () => {
  test('should render KDS, toggle Arabic language, and handle offline queuing', async ({ page, context }) => {
    // Navigate to KDS View
    await page.goto('/kds');

    // Default language is English
    // Use first() to avoid strict mode violations if multiple matches are found (e.g. Next.js development overlay)
    await expect(page.locator('h1').filter({ hasText: 'Kitchen Display System' }).first()).toBeVisible();
    await expect(page.locator('#lang-toggle-btn')).toContainText('عربي');

    // Toggle to Arabic
    await page.click('#lang-toggle-btn');
    await expect(page.locator('h1').filter({ hasText: 'نظام عرض المطبخ' }).first()).toBeVisible();
    await expect(page.locator('#lang-toggle-btn')).toContainText('English');

    // Toggle back to English for easier assertions
    await page.click('#lang-toggle-btn');
    await expect(page.locator('h1').filter({ hasText: 'Kitchen Display System' }).first()).toBeVisible();

    // Go Offline
    await context.setOffline(true);
    await page.evaluate(() => {
      window.dispatchEvent(new Event('offline'));
    });

    // We verify the indicator has the correct color rather than text sibling selectors which can be tricky
    await expect(page.locator('#network-status-indicator')).toHaveClass(/bg-red-500/);

    // Mark 'Chicken Over Rice' as sold out
    const soldOutBtn = page.locator('#sold-out-toggle-item-chicken');
    await soldOutBtn.click();

    // Verify Optimistic UI update
    await expect(soldOutBtn).toContainText('Sold Out');
    await expect(soldOutBtn).toHaveClass(/bg-red-100/);

    // Verify Offline Sync Banner appears
    const queueBanner = page.locator('#queue-dashboard');
    await expect(queueBanner).toBeVisible();
    await expect(queueBanner).toContainText('1 Sync Pending');

    // Mark order '101' as Preparing
    const prepBtn = page.locator('#btn-prep-101');
    await prepBtn.click();

    // Optimistic UI updates
    const readyBtn = page.locator('#btn-ready-101');
    await expect(readyBtn).toBeVisible();
    await expect(readyBtn).toContainText('Ready');

    // Verify Offline Sync Banner increments
    await expect(queueBanner).toContainText('2 Sync Pending');

    // Go Online
    await context.setOffline(false);
    await page.evaluate(() => {
      window.dispatchEvent(new Event('online'));
    });

    // Verify Network Status indicator
    await expect(page.locator('#network-status-indicator')).toHaveClass(/bg-green-500/);

    // Wait for sync to complete and banner to disappear
    await expect(queueBanner).toBeHidden({ timeout: 5000 });
  });
});
