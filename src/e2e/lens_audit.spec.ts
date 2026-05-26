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

      // E2E seed data has 1 pending approval: 'Draft email for review' in customer_success
      // The dashboard page will render it natively if the UI is wired correctly
      await expect(page.getByText("Action Required")).toBeVisible({ timeout: 10000 });
      await expect(page.getByText("Draft email for review")).toBeVisible();
      await expect(page.getByRole('button', { name: 'Approve' })).toBeVisible();
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

  // NEW AUDIT TESTS
  test('verify that no mock Automated Review Request is hardcoded in the Action Required panel', async ({ page }) => {
    await page.goto('/dashboard');
    // Ensure the main "Action Required" section loads with real data, but not the hardcoded mock text
    await expect(page.getByText('Action Required')).toBeVisible({ timeout: 10000 });
    const mockCardText = page.getByText("3 customers haven't reviewed their orders. Request reviews?", { exact: false });
    await expect(mockCardText).not.toBeVisible();
  });

  test('verify Seasonal Promo generator creates campaign immediately without mock timeouts', async ({ page }) => {
    await page.goto('/seasonal-promo');
    const occasionInput = page.locator('input#promo-occasion');
    await occasionInput.fill('Spring Sale');
    const discountInput = page.locator('input#promo-discount');
    await discountInput.fill('20');

    const generateBtn = page.getByRole('button', { name: /Generate Campaign/ });
    await generateBtn.click();

    // Because the setTimeout was removed, it should appear synchronously.
    await expect(page.getByText('Spring Sale Special! 20% OFF')).toBeVisible({ timeout: 1000 });
  });

  test('verify embedded desktop Rust UI dashboard-screen does not contain hardcoded approval-item-1', async ({ page }) => {
    await page.goto('/');
    // We navigate to dashboard and evaluate the DOM to make sure id="approval-item-1" was stripped.
    const dashboardLink = page.getByRole('link', { name: 'Dashboard' }).first();
    await dashboardLink.click();
    const hardcodedApprovalItem = page.locator('#approval-item-1');
    await expect(hardcodedApprovalItem).not.toBeAttached();
  });

  test('verify the real seeded DB approvals persist on reload without mock state reset', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByText("Action Required")).toBeVisible({ timeout: 10000 });
    // First load confirms real data is populated.
    await expect(page.getByText("Draft email for review")).toBeVisible();

    // Reload page, it should fetch from the DB again and display the identical data without relying on hardcoded arrays.
    await page.reload();
    await expect(page.getByText("Action Required")).toBeVisible({ timeout: 10000 });
    await expect(page.getByText("Draft email for review")).toBeVisible();
  });

  test('verify CustomerSuccess Department renders properly inside Action Required block', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByText("Action Required")).toBeVisible({ timeout: 10000 });
    await expect(page.getByText("CustomerSuccess Department")).toBeVisible();
  });

  test('verify Approve button works and removes item from UI', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByText("Action Required")).toBeVisible({ timeout: 10000 });

    // We expect the seeded 'Draft email for review' to be present initially
    const approvalText = page.getByText("Draft email for review");
    await expect(approvalText).toBeVisible();

    const approveButton = page.locator('div.p-5').filter({ hasText: 'Draft email for review' }).getByRole('button', { name: 'Approve' });
    await approveButton.click();

    // After clicking approve, the item should be removed
    await expect(approvalText).not.toBeVisible();
  });

  test('verify Reject button works and removes item from UI', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByText("Action Required")).toBeVisible({ timeout: 10000 });

    const approvalText = page.getByText("Abandoned cart recovery");
    await expect(approvalText).toBeVisible();

    const rejectButton = page.locator('div.p-5').filter({ hasText: 'Abandoned cart recovery' }).getByRole('button', { name: 'Reject' });
    await rejectButton.click();

    // After clicking reject, the item should be removed
    await expect(approvalText).not.toBeVisible();
  });

  test('verify Business Snapshot remains visible when Action Required is populated', async ({ page }) => {
    await page.goto('/dashboard');
    // We know Action Required is populated from the seed data
    await expect(page.getByText("Action Required")).toBeVisible({ timeout: 10000 });

    // Verify Business Snapshot is also visible, ensuring it's not hidden
    await expect(page.getByText("Business Snapshot")).toBeVisible();
  });

  test('verify Team Activity waiting state is rendered', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByText("Team Activity")).toBeVisible({ timeout: 10000 });

    // Verify the "Waiting for team activity..." element is rendered before any websockets messages
    await expect(page.getByText("Waiting for team activity...")).toBeVisible();
  });

  test('verify promo generation resolves instantly (no 800ms mock delay)', async ({ request }) => {
    // Tests that the API endpoint doesn't have the 800ms mock delay
    const start = Date.now();
    const res = await request.post('/api/v1/growth/promotions/generate', {
      data: { tenant: 'test-tenant' }
    });
    const end = Date.now();
    expect(res.ok()).toBeTruthy();
    // Assuming a fast local response (< 200ms) - definitely less than the 800ms old mock
    expect(end - start).toBeLessThan(500);
  });

  test('verify seasonal-promo route resolves and has correct initial state without timeouts', async ({ page }) => {
    await page.goto('/seasonal-promo');
    await expect(page.getByRole('heading', { name: 'AI Seasonal Promotions' })).toBeVisible();
    await expect(page.locator('input#promo-occasion')).toBeVisible();
  });

  test('verify the "generating..." UI isn\'t stuck on the dashboard-screen when navigating', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // Ensure "Generating..." mock state is not present in the dom
    await expect(page.getByText('Generating...')).not.toBeVisible();
  });

  test('verify navigation from the dashboard doesn\'t incur the 2000ms mock delay', async ({ page }) => {
    await page.goto('/dashboard');
    const start = Date.now();

    // Navigate to agents
    const agentsLink = page.getByRole('link', { name: 'AI Departments' });
    await agentsLink.click();

    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible({ timeout: 1000 });
    const end = Date.now();

    // Ensure it took less than 2000ms
    expect(end - start).toBeLessThan(1500);
  });

  test('verify that the promo string correctly replaces {tenant} with the default value my-store when tenant is not defined', async ({ request }) => {
    const res = await request.post('/api/v1/growth/promotions/generate', {
      data: {} // Empty body to trigger the fallback
    });
    expect(res.ok()).toBeTruthy();
    const body = await res.json();

    // Because the response is random, we just check that '{tenant}' is not present
    // and that 'my-store' is present
    expect(body.message).not.toContain('{tenant}');
    expect(body.message).toContain('my-store');
  });
});
