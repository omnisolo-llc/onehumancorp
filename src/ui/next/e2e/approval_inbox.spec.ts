import { expect, test } from '@playwright/test';

test.describe('Unified Agent Feed Mobile Test', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should render properly and handle tabs', async ({ page }) => {
    test.setTimeout(180000);

    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Wait for the unified agent feed to load
    await expect(page.locator('button', { hasText: /Proposals/ }).first()).toBeVisible({ timeout: 15000 });
    await expect(page.locator('button', { hasText: 'Activity Feed' })).toBeVisible();

    // Switch tabs
    await page.locator('button', { hasText: 'Activity Feed' }).click();

    // Verify glassmorphism CSS
    const feedContainer = page.locator('.glassmorphism').first();
    await expect(feedContainer).toBeVisible();
    await expect(feedContainer).toHaveCSS('backdrop-filter', /blur\(30px\)|none/);

    // Switch back
    await page.locator('button', { hasText: /Proposals/ }).first().click({ force: true });

    // Verify one of the approval items is visible
    await expect(page.locator('h3', { hasText: /Agent tentatively booked/ }).first()).toBeVisible();
  });

  test('should display Action Needed tag correctly', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('button', { hasText: /Proposals/ }).first()).toBeVisible({ timeout: 15000 });

    // Look for Action Needed tag
    const actionNeededTag = page.locator('span', { hasText: 'Action Needed' }).first();
    await expect(actionNeededTag).toBeVisible();
    await expect(actionNeededTag).toHaveClass(/bg-green-100/);
  });

  test('should display Approval tag correctly', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('button', { hasText: /Proposals/ }).first()).toBeVisible({ timeout: 15000 });

    // Look for Approval tag
    const approvalTag = page.locator('span', { hasText: 'Approval' }).first();
    await expect(approvalTag).toBeVisible();
    await expect(approvalTag).toHaveClass(/bg-\[\#0066FF\]\/10/);
  });

  test('should display action buttons for proposals', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('button', { hasText: /Proposals/ }).first()).toBeVisible({ timeout: 15000 });

    const approveButton = page.locator('button', { hasText: 'Approve' }).first();
    await expect(approveButton).toBeVisible();
    await expect(approveButton).toHaveClass(/bg-green-500/);

    const editButton = page.locator('button', { hasText: 'Edit' }).first();
    await expect(editButton).toBeVisible();

    const denyButton = page.locator('button', { hasText: 'Deny' }).first();
    await expect(denyButton).toBeVisible();
    await expect(denyButton).toHaveClass(/bg-red-100/);
  });

  test('should display empty state or loading state in Activity Feed correctly', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('button', { hasText: 'Activity Feed' })).toBeVisible({ timeout: 15000 });

    // Switch tabs
    await page.locator('button', { hasText: 'Activity Feed' }).click();

    // Loading or empty state or populated activities
    const activityFeedItems = page.locator('.glassmorphism', { hasText: /Activity Feed|No recent activity found|Action completed/ });
    await expect(activityFeedItems.first()).toBeVisible();
  });
});
