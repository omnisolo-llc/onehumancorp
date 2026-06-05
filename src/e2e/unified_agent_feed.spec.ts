import { test, expect } from './fixtures';

test.describe('Unified Agent Feed', () => {
  test('should display agent feed and allow interaction on mobile viewport', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');

    // Target 375px mobile viewport specifically as per OHC Mobile-First Non-Negotiables
    await page.setViewportSize({ width: 375, height: 812 });

    // Ensure we are using the seeded e2e tenant explicitly to fetch the seed data
    await page.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-tenant');
      localStorage.setItem('user_id', 'e2e-admin-user');
    });

    // Go to feed
    await page.goto('/feed');

    // Verify we are on the feed and the Unified Agent Feed is present
    await expect(page.getByRole('heading', { name: 'Agent Proposals' })).toBeVisible();

    // We expect seeded approvals to show up because of our seed data updates
    await expect(page.getByText('Draft email for review')).toBeVisible();
    await expect(page.getByText('Abandoned cart recovery: 10% discount for Sarah')).toBeVisible();

    // Check context data display
    await expect(page.getByText('Abandoned Carts:')).toBeVisible();
    await expect(page.getByText('3', { exact: true }).first()).toBeVisible();
    await expect(page.getByText('Potential Revenue:')).toBeVisible();
    await expect(page.getByText('$120.00')).toBeVisible();

    // Verify Edit button exists
    const editBtn = page.locator('button[aria-label="Edit proposal"]').first();
    await expect(editBtn).toBeVisible();

    // Click to decline the abandoned cart proposal
    const declineBtn = page.locator('button[aria-label="Reject proposal"]').last();
    await declineBtn.click();

    // Verify it was optimistically removed from the UI
    await expect(page.getByText('Abandoned cart recovery: 10% discount for Sarah')).not.toBeVisible();

    // Verify touch targets and layout constraints (glassmorphism/375px max width)
    const approveBtn = page.locator('button[aria-label="Approve proposal"]').first();
    const box = await approveBtn.boundingBox();
    expect(box?.width).toBeGreaterThanOrEqual(44);
    expect(box?.height).toBeGreaterThanOrEqual(44);

    const section = page.locator('section[aria-label="Unified Agent Feed"]');
    const sectionBox = await section.boundingBox();
    expect(sectionBox?.width).toBeLessThanOrEqual(375);
  });
});
