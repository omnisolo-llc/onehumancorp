import { test, expect } from './fixtures';

test.describe('Lens Audit E2E Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('verify dashboard visual state and full UI lifecycle', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.locator('nav')).toBeVisible();
    await expect(page.getByText("Today's Sales")).toBeVisible();
    await expect(page.getByText("$114.99")).toBeVisible({ timeout: 10000 });
  });

  test('verify setup wizard starts and preserves real form state', async ({ page }) => {
    await page.goto('/business-setup');
    await page.getByRole('button', { name: /Start My Business/ }).click();

    await expect(page.getByRole('heading', { name: 'What kind of business are you building?' })).toBeVisible();
    await page.getByPlaceholder('Business type').fill('Online Store');
    await page.getByRole('button', { name: /Next/ }).click();

    await expect(page.getByRole('heading', { name: 'Give your business a name' })).toBeVisible();
    await expect(page.getByPlaceholder("What is your business called?")).toBeVisible();
  });

  test('verify responsive navigation compliance', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.locator('#mobile-bottom-nav')).toBeVisible();
  });

  test('verify unknown routes fall back without crashing', async ({ page }) => {
    await page.goto('/setup-screen');

    await expect(page.getByRole('heading').first()).toBeVisible();
  });

  test('verify user guide and help actions remain reachable', async ({ page }) => {
    await page.getByRole('button', { name: 'How to use this app' }).click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('verify visual truth mesh websocket updates and E2E navigation state lifecycle', async ({ page, request }) => {
      // Navigation naturally from home
      await page.goto('/');
      await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

      const dashboardLink = page.getByRole('link', { name: 'Dashboard' }).first();
      await dashboardLink.click();
      await expect(page).toHaveURL(/.*\/dashboard/);

      // Core logic DB validation
      // Ensure we see active snapshot
      await expect(page.getByText('Business Snapshot')).toBeVisible();
      await expect(page.getByText("Today\'s Sales")).toBeVisible();

      // Fire a websocket message and see if it populates the table via mesh connect endpoint instead of the mock system
      // Since we can't reliably drive mesh directly from playwright without deeper mocks which is anti-pattern,
      // we verify the endpoint handles real connections
      const res = await request.post('/api/mesh/v2/broadcast', {
         headers: {
           'Content-Type': 'application/json',
           'x-spiffe-id': 'spiffe://example.org/test'
         },
         data: {
           topic: 'system',
           message: {
             agent_id: "agent-test",
             action: "testing-mesh",
             status: "ok",
             payload: "e2e-payload",
             msg_id: "1234"
           }
         }
      });
      expect(res.status()).toBe(200);

      // Wait for the UI to update via websocket to reflect our task broadcast
      await page.waitForTimeout(500);
      await expect(page.getByText(/testing-mesh/)).toBeVisible({ timeout: 10000 });
  });

  test('verify grandmother criteria for main dashboard elements', async ({ page }) => {
      await page.goto('/');
      const dashboardLink = page.getByRole('link', { name: 'Dashboard' }).first();
      await dashboardLink.click();

      await expect(page.getByText('Business Snapshot')).toBeVisible();
      await expect(page.getByText('Active Customers')).toBeVisible();
      await expect(page.getByText('Pending Orders')).toBeVisible();
  });

  test('verify E2E database order insertion reflects in UI dashboard data', async ({ page }) => {
      await page.goto('/');
      const dashboardLink = page.getByRole('link', { name: 'Dashboard' }).first();
      await dashboardLink.click();

      // Wait for metrics to finish loading and fetching from API (which uses E2E DB state)
      await expect(page.getByText("Today\'s Sales")).toBeVisible();
      // Verify seeded database state (75.00 + 39.99 = 114.99) is displayed correctly without mock injection
      await expect(page.getByText("$114.99")).toBeVisible({ timeout: 10000 });

      await expect(page.getByText("Pending Orders")).toBeVisible();
  });

  test('verify E2E database agent approvals reflects in UI without mock injection', async ({ page }) => {
      await page.goto('/');
      const dashboardLink = page.getByRole('link', { name: 'Dashboard' }).first();
      await dashboardLink.click();

      // E2E seed data has pending approvals
      await expect(page.getByText("Action Required")).toBeVisible({ timeout: 10000 });
      await expect(page.getByText("Draft email for review")).toBeVisible();
      await expect(page.getByRole('button', { name: 'Approve' }).first()).toBeVisible();
  });

  test('verify approval rejection action updates DB and UI', async ({ page, request }) => {
      await page.goto('/');
      const dashboardLink = page.getByRole('link', { name: 'Dashboard' }).first();
      await dashboardLink.click();

      // Verify initial state
      await expect(page.getByText("Action Required")).toBeVisible({ timeout: 10000 });
      await expect(page.getByText("Draft email for review")).toBeVisible();

      // Click reject on the first approval
      await page.getByRole('button', { name: 'Reject' }).first().click();

      // Verify it disappeared from UI
      await expect(page.getByText("Draft email for review")).not.toBeVisible();

      // Refresh the page and assert it is still gone to verify backend actually saved it
      await page.reload();
      await expect(page.getByText("Draft email for review")).not.toBeVisible();
  });

  test('verify approval acceptance action updates DB and UI', async ({ page }) => {
      await page.goto('/');
      const dashboardLink = page.getByRole('link', { name: 'Dashboard' }).first();
      await dashboardLink.click();

      // Verify initial state
      await expect(page.getByText("Action Required")).toBeVisible({ timeout: 10000 });
      await expect(page.getByText("Generated 7-day social media plan for Vegan Celebration Cake")).toBeVisible();

      // Click approve on the approval
      // We will look for the button near the text
      const approvalCard = page.locator('div').filter({ hasText: 'Generated 7-day social media plan for Vegan Celebration Cake' }).first();
      await approvalCard.getByRole('button', { name: 'Approve' }).click();

      // Verify it disappeared from UI
      await expect(page.getByText("Generated 7-day social media plan for Vegan Celebration Cake")).not.toBeVisible();

      // Refresh the page and assert it is still gone to verify backend actually saved it
      await page.reload();
      await expect(page.getByText("Generated 7-day social media plan for Vegan Celebration Cake")).not.toBeVisible();
  });

  test('verify absence of hardcoded CustomerSuccess mockup data', async ({ page }) => {
      await page.goto('/');
      const dashboardLink = page.getByRole('link', { name: 'Dashboard' }).first();
      await dashboardLink.click();
      // The text from the old mock should NOT be there
      await expect(page.getByText("3 customers haven't reviewed their orders. Request reviews?")).not.toBeVisible();
  });

  test('verify Referral Program Snapshot uses dynamic data', async ({ page }) => {
      await page.goto('/');
      const dashboardLink = page.getByRole('link', { name: 'Dashboard' }).first();
      await dashboardLink.click();
      await expect(page.getByText("Referral Program")).toBeVisible();
      await expect(page.getByText("Team Invites Sent")).toBeVisible();
      // Ensure the old hardcoded mock data is absent
      await expect(page.getByText("Active Referrals")).not.toBeVisible();
      await expect(page.getByText("Revenue from Referrals")).not.toBeVisible();
      await expect(page.getByText("Pending Rewards")).not.toBeVisible();
  });

  test('verify visual grid token consistency and translucent card styling', async ({ page }) => {
    await page.goto('/');
    const dashboardLink = page.getByRole('link', { name: 'Dashboard' }).first();
    await dashboardLink.click();

    // Ensure one of the core hybrid panels adheres to the macOS-style Glass materials
    // background: rgba(255, 255, 255, 0.65)
    const panel = page.locator('.ohc-hybrid-panel').first();
    await expect(panel).toBeVisible();

    // Ensure it's rendered properly according to truth rules
    // Using evaluate for raw computed CSS style matching
    const hasToken = await panel.evaluate((el) => {
        const style = window.getComputedStyle(el);
        // Note standard browsers return `rgba(...)`
        return style.backgroundColor === 'rgba(255, 255, 255, 0.65)';
    });
    // This expects the element to correctly apply styling properties directly or via css overrides.
    expect(hasToken).toBe(true);
  });
});
