import { test, expect } from '../fixtures';

test.describe('Unified Agent Feed CUJ', () => {
  test('should display agent feed and allow 1-tap approval', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');

    // Carlos the Handyman persona - using seeded e2e-tenant
    await page.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-tenant');
      localStorage.setItem('user_name', 'Carlos');
    });

    // 1. Go to dashboard
    await page.goto('/dashboard');

    // 2. Verify Agent Feed is present (Daily Feed header)
    await expect(page.getByText('Daily Feed')).toBeVisible();

    // 3. Verify seeded proposals are visible (from e2e-seed.sql)
    await expect(page.getByText('Draft email for review')).toBeVisible();
    await expect(page.getByText('Abandoned cart recovery: 10% discount for Sarah')).toBeVisible();

    // 4. Verify Context Data display in the card
    await expect(page.getByText('Abandoned Carts:')).toBeVisible();
    await expect(page.getByText('3', { exact: true }).first()).toBeVisible();

    // 5. Verify Touch Target Size (Premium Standard)
    const approveBtn = page.getByRole('button', { name: 'Approve' }).first();
    const box = await approveBtn.boundingBox();
    if (box) {
      expect(box.width).toBeGreaterThanOrEqual(44);
      expect(box.height).toBeGreaterThanOrEqual(44);
    }

    // 6. Execute Approval
    await approveBtn.click();

    // 7. Verify Optimistic Removal
    await expect(page.getByText('Draft email for review')).not.toBeVisible();
  });

  test('should navigate to dedicated agent feed and show all clear state', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');

    // Use a clean tenant with no proposals
    await page.addInitScript(() => {
      localStorage.setItem('tenant_id', 'new-clean-tenant');
    });

    await page.goto('/agent-feed');
    await expect(page.getByText('Agent Activity Feed')).toBeVisible();
    await expect(page.getByText('All Clear!')).toBeVisible();
  });
});
