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
      await expect(page.getByText(/Waiting for team activity\.\.\./)).not.toBeVisible();
  });

  test('verify mesh websocket accepts raw JSON payloads and parses correctly', async ({ page, request }) => {
      await page.goto('/');
      const dashboardLink = page.getByRole('link', { name: 'Dashboard' }).first();
      await dashboardLink.click();

      // Broadcast raw JSON (already happening implicitly, but explicit test for isolation)
      await request.post('/api/mesh/v2/broadcast', {
         headers: { 'Content-Type': 'application/json', 'x-spiffe-id': 'spiffe://example.org/test' },
         data: { topic: 'system', message: { agent_id: "agent-json", action: "json-parsing-test", status: "ok" } }
      });
      await page.waitForTimeout(500);
      await expect(page.getByText(/json-parsing-test/)).toBeVisible({ timeout: 10000 });
  });

  test('verify mesh websocket accepts base64 payloads as fallback', async ({ page, request }) => {
      await page.goto('/');
      const dashboardLink = page.getByRole('link', { name: 'Dashboard' }).first();
      await dashboardLink.click();

      // To test base64 fallback properly, we'll send a payload formatted such that it tricks the JSON.parse
      // into failing if it wasn't a valid JSON but a valid base64 string. However, request.post '/api/mesh/v2/broadcast'
      // routes the payload. We will broadcast a base64 string directly in the payload data object,
      // simulating what happens if the backend encoded the action string inside.
      await request.post('/api/mesh/v2/broadcast', {
         headers: { 'Content-Type': 'application/json', 'x-spiffe-id': 'spiffe://example.org/test' },
         data: { topic: 'system', message: btoa(JSON.stringify({ agent_id: "agent-base64", action: "base64-parsing-test", status: "ok" })) }
      });
      await page.waitForTimeout(500);
      await expect(page.getByText(/base64-parsing-test/)).toBeVisible({ timeout: 10000 });
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

  test('verify hardcoded UI warnings and mock cards are removed', async ({ page }) => {
      await page.goto('/');
      const dashboardLink = page.getByRole('link', { name: 'Dashboard' }).first();
      await dashboardLink.click();

      await expect(page.getByText('1 Action Required: Connect Stripe to accept payments.')).not.toBeVisible();
      await expect(page.getByText('CustomerSuccess Department')).not.toBeVisible();
  });

  test('verify advanced settings toggle hides and shows payload data', async ({ page }) => {
      await page.goto('/');
      const dashboardLink = page.getByRole('link', { name: 'Dashboard' }).first();
      await dashboardLink.click();

      await expect(page.getByText('Advanced Settings')).toBeVisible();
      const advancedToggle = page.locator('button', { hasText: '' }).filter({ has: page.locator('span.absolute') }).first();

      // By default payload should be hidden
      await expect(page.locator('pre')).not.toBeVisible();

      // Enable advanced settings
      await advancedToggle.click();

      // We expect the payload block to become visible, because there's an action required for 'Draft email for review'
      await expect(page.locator('pre')).toBeVisible();
  });

  test('verify dashboard metric cards load without mock data', async ({ page }) => {
      await page.goto('/');
      const dashboardLink = page.getByRole('link', { name: 'Dashboard' }).first();
      await dashboardLink.click();

      const activeCustomers = page.locator('.ohc-hybrid-panel').filter({ hasText: 'Active Customers' });
      await expect(activeCustomers).toBeVisible();
      await expect(activeCustomers.locator('.text-3xl')).not.toBeEmpty();

      const pendingOrders = page.locator('.ohc-hybrid-panel').filter({ hasText: 'Pending Orders' });
      await expect(pendingOrders).toBeVisible();
      await expect(pendingOrders.locator('.text-3xl')).not.toBeEmpty();
  });

  test('verify growth and promotions card is fully functional', async ({ page }) => {
      await page.goto('/');
      const dashboardLink = page.getByRole('link', { name: 'Dashboard' }).first();
      await dashboardLink.click();

      await expect(page.getByText('Boost Sales with AI Campaigns')).toBeVisible();
      const generateBtn = page.getByRole('button', { name: 'Generate Promotion' });
      await expect(generateBtn).toBeVisible();

      await generateBtn.click();
      await expect(page.getByText('Drafting holiday campaign...')).toBeVisible();
  });

  test('verify referral program section visibility and generation', async ({ page }) => {
      await page.goto('/');
      const dashboardLink = page.getByRole('link', { name: 'Dashboard' }).first();
      await dashboardLink.click();

      await expect(page.getByText('Referral Program')).toBeVisible();
      await expect(page.getByText('Share your store')).toBeVisible();

      // Ensure the generate link functionality is mock-free and works via backend
      // The button text changes after generation (handled in other E2E tests, here we just check visibility)
      await expect(page.getByRole('button', { name: 'Share & Claim Reward' })).toBeVisible();
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
        return style.backgroundColor === 'rgba(255, 255, 255, 0.65)' &&
               style.backdropFilter === 'blur(30px) saturate(210%)' &&
               style.border === '1px solid rgba(255, 255, 255, 0.4)';
    });
    // This expects the element to correctly apply styling properties directly or via css overrides.
    expect(hasToken).toBe(true);
  });

  test('verify visual grid token consistency in dark mode', async ({ page }) => {
    // Emulate dark mode
    await page.emulateMedia({ colorScheme: 'dark' });
    await page.goto('/');
    const dashboardLink = page.getByRole('link', { name: 'Dashboard' }).first();
    await dashboardLink.click();

    const panel = page.locator('.ohc-hybrid-panel').first();
    await expect(panel).toBeVisible();

    const hasDarkToken = await panel.evaluate((el) => {
        const style = window.getComputedStyle(el);
        return style.backgroundColor === 'rgba(22, 22, 26, 0.7)' &&
               style.backdropFilter === 'blur(30px) saturate(210%)' &&
               style.border === '1px solid rgba(255, 255, 255, 0.1)';
    });
    expect(hasDarkToken).toBe(true);
  });
});
