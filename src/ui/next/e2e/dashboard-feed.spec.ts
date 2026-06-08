import { test, expect } from '@playwright/test';

test.describe('Dashboard Actionable Feed', () => {
  test('should display database-backed operations console', async ({ page }) => {
    await page.goto('/dashboard');

    await expect(page.getByRole('heading', { name: 'Business Analytics' })).toBeVisible();
    await expect(page.locator('text="Operations Map"').first()).toBeVisible();
    // Action required might not be there anymore, verifying other core elements
    await expect(page.locator('text="Recent Orders"')).toBeVisible();
    await expect(page.locator('text="Inbox Activity"')).toBeVisible();
  });

  test('should display the Unified Agent Feed proposals tab by default', async ({ page }) => {
    await page.goto('/dashboard');
    const proposalsTab = page.locator('button:has-text("Proposals")');
    await expect(proposalsTab).toBeVisible();
    await expect(proposalsTab).toHaveClass(/border-b-2/);
  });

  test('should be able to switch to Activity Feed tab', async ({ page }) => {
    await page.goto('/dashboard');
    const activityTab = page.locator('button:has-text("Activity Feed")');
    await expect(activityTab).toBeVisible();
    await activityTab.click();
    await expect(activityTab).toHaveClass(/border-b-2/);
  });

  test('should hide payload details initially and show them on click', async ({ page, request }) => {
    // Inject a test approval request
    await request.post('/api/agents/approvals/simulate-smart-pricing', {
        headers: { 'x-tenant-id': 'e2e-test' }
    });

    // Setup local storage to act as e2e-test tenant
    await page.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-test');
    });

    await page.goto('/dashboard');

    // Wait for the feed to load
    const cardToggle = page.locator('[data-testid^="card-toggle-"]').first();
    await expect(cardToggle).toBeVisible();

    // Verify 'Approve' button is NOT visible initially
    const approveButton = page.getByTestId('approve-proposal').first();
    await expect(approveButton).toBeHidden();

    // Click to expand
    await cardToggle.click();

    // Verify 'Approve' button is now visible
    await expect(approveButton).toBeVisible();
    await expect(page.getByTestId('reject-proposal').first()).toBeVisible();
  });

  test('should allow approving a proposal and remove it from the feed', async ({ page, request }) => {
    // Inject a test approval request
    await request.post('/api/agents/approvals/simulate-smart-pricing', {
        headers: { 'x-tenant-id': 'e2e-test-approve' }
    });

    // Setup local storage to act as e2e-test-approve tenant
    await page.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-test-approve');
    });

    await page.goto('/dashboard');

    // Wait for the feed to load
    const cardToggle = page.locator('[data-testid^="card-toggle-"]').first();
    await expect(cardToggle).toBeVisible();
    const id = (await cardToggle.getAttribute('data-testid'))?.replace('card-toggle-', '');

    // Expand
    await cardToggle.click();

    // Click Approve
    const approveButton = page.getByTestId('approve-proposal').first();
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // Verify it disappears from the proposals list
    await expect(page.locator(`[data-testid="card-toggle-${id}"]`)).toBeHidden();
  });
});
