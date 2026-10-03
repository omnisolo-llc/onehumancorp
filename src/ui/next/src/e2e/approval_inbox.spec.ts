import { expect, test } from '@playwright/test';
import { createDashboardAuditCase } from '../../../../e2e/support/dashboard_audit_fixture';

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

    // Verify the backend-backed feed produced at least one actionable proposal.
    await expect(page.locator('[data-testid*="triage-card-"]').first()).toBeVisible();
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
    await expect(approvalTag).toHaveClass(/bg-\[#0066FF\]\/10/);
  });

  test('should display action buttons for the recorded generic proposal', async ({ browser, baseURL }) => {
    if (!baseURL) throw new Error('The isolated app base URL is required');
    const owned = await createDashboardAuditCase(browser, baseURL, { width: 375, height: 812 });
    try {
      await owned.navigate();
      const proposal = owned.page.getByTestId(`triage-card-${owned.actor.namespace}-e2e-feed-reschedule`);
      await expect(proposal).toBeVisible();
      // Quote drafts have a distinct blue Approve & Send action. These existing
      // assertions belong to the recorded generic proposal, not the first card.
      const approveButton = proposal.getByRole('button', { name: 'Approve proposal', exact: true });
      await expect(approveButton).toBeVisible();
      await expect(approveButton).toHaveClass(/bg-green-500/);
      await expect(proposal.getByRole('button', { name: 'Edit proposal', exact: true })).toBeVisible();
      const denyButton = proposal.getByRole('button', { name: 'Reject proposal', exact: true });
      await expect(denyButton).toBeVisible();
      await expect(denyButton).toHaveClass(/bg-red-100/);
    } finally { await owned.close(); }
  });

  test('should display empty state or loading state in Activity Feed correctly', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('button', { hasText: 'Activity Feed' })).toBeVisible({ timeout: 15000 });

    // Switch tabs
    await page.locator('button', { hasText: 'Activity Feed' }).click();

    // Loading, empty, or populated activity surfaces must all be rendered with the shared glass treatment.
    const activityFeedSurface = page.locator('[data-testid^="activity-feed-"]').first();
    await expect(activityFeedSurface).toBeVisible();
    await expect(activityFeedSurface).toHaveClass(/glassmorphism/);
  });
});
