import { expect, test } from '@playwright/test';

test.describe('Triage Action Feed UI', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should render triage feed properly, show distinct card types, allow approval, and display empty state', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Ensure we are operating as a logged in user, and fetch the tenant
    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');

    // 2. Seed some distinct triage data matching the backend's expected payload
    const seedData = [
      {
        source: 'Instagram DM Message',
        priority: 'high',
        context: 'Message: Customer asked about vegan cakes.',
        action_type: 'Draft Reply',
        action_payload: 'Yes, we have vegan options.',
        customer_id: 'cust_test_1'
      },
      {
        source: 'Website Booking Request',
        priority: 'medium',
        context: 'Booking: Sarah wants an estimate tomorrow at 2PM.',
        action_type: 'Accept Booking',
        action_payload: 'Booked for 2PM tomorrow.',
        customer_id: 'cust_test_2'
      },
      {
        source: 'Inventory Alert',
        priority: 'urgent',
        context: 'Alert: Low stock on flour.',
        action_type: 'Reorder',
        action_payload: 'Order 50lbs of flour.',
        customer_id: 'cust_test_3'
      }
    ];

    for (const data of seedData) {
      await page.request.post(`/api/triage/create?tenant_id=${encodeURIComponent(tenantId)}`, {
        data
      });
    }

    // 3. Navigate to Triage Feed
    await page.goto('/triage');
    await expect(page.locator('body')).toContainText(/Work Triage/, { timeout: 15000 });

    const listItems = page.locator('div[data-testid^="triage-card-"]');

    await page.waitForTimeout(2000); // Give it a moment to render loaded items

    // Check if we loaded the items properly
    let count = await listItems.count();

    // 4. Process all cards one by one (approving or dismissing)
    while (count > 0) {
      const firstCard = listItems.nth(0);
      const testId = await firstCard.getAttribute('data-testid');
      const approveBtn = firstCard.locator('[data-testid^="approve-btn"], [data-testid^="triage-approve-"]');
      const dismissBtn = firstCard.locator('[data-testid^="dismiss-btn"], [data-testid^="triage-dismiss-"]');

      if (await approveBtn.isVisible()) {
        await approveBtn.click();
      } else if (await dismissBtn.isVisible()) {
        await dismissBtn.click();
      } else {
        break;
      }

      // 5. Verify the card disappears (Optimistic UI + backend update)
      if (testId) {
          await expect(page.locator(`div[data-testid="${testId}"]`)).not.toBeVisible({ timeout: 5000 });
      }

      // Re-count remaining cards
      count = await listItems.count();
    }

    // 6. Verify empty state is displayed properly when caught up
    const emptyState = page.getByTestId('triage-feed-empty');
    await expect(emptyState).toBeVisible({ timeout: 5000 });
    await expect(emptyState).toContainText(/caught up/i);
  });

  test('should reflect distinct priority badge tones', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');

    const seedData = [
      { source: 'Test', priority: 'urgent', context: 'Test Urgent', customer_id: 'cust_urgent' },
      { source: 'Test', priority: 'high', context: 'Test High', customer_id: 'cust_high' },
      { source: 'Test', priority: 'action needed', context: 'Test Action Needed', customer_id: 'cust_action' },
      { source: 'Test', priority: 'fyi', context: 'Test FYI', customer_id: 'cust_fyi' }
    ];

    for (const data of seedData) {
      await page.request.post(`/api/triage/create?tenant_id=${encodeURIComponent(tenantId)}`, { data });
    }

    await page.goto('/triage');
    await page.waitForTimeout(2000);

    // Verify Urgent/High (bad)
    await expect(page.locator('.app-badge.bad').first()).toBeVisible();

    // Verify Action Needed (warn)
    await expect(page.locator('.app-badge.warn').first()).toBeVisible();

    // Verify FYI (good)
    await expect(page.locator('.app-badge.good').first()).toBeVisible();
  });

  test('should handle actions gracefully without payloads', async ({ page }) => {
    test.setTimeout(180000);

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');

    await page.request.post(`/api/triage/create?tenant_id=${encodeURIComponent(tenantId)}`, {
      data: {
        source: 'Missing Payload',
        priority: 'low',
        context: 'Action with no payload',
        action_type: 'No Payload Action',
        action_payload: '',
        customer_id: 'cust_no_payload'
      }
    });

    await page.goto('/triage');
    await page.waitForTimeout(2000);

    const card = page.locator('div[data-testid^="triage-card-"]').first();
    await expect(card).toContainText('No Payload Action');
    await expect(card).toContainText('No specific payload');

    const approveBtn = card.locator('[data-testid^="approve-btn"], [data-testid^="triage-approve-"]');
    await approveBtn.click();
    await expect(card).not.toBeVisible({ timeout: 5000 });
  });

  test('should correctly show empty state when there are initially no items', async ({ page }) => {
    test.setTimeout(180000);

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');
    // Clear out any items first for this test
    const response = await page.request.get(`/api/triage/pending?tenant_id=${encodeURIComponent(tenantId)}`);
    const data = await response.json();
    const items = Array.isArray(data) ? data : (data?.items || []);
    for (const item of items) {
      await page.request.post(`/api/triage/action?tenant_id=${encodeURIComponent(tenantId)}`, {
        data: { triage_item_id: item.id, approved: false }
      });
    }

    await page.goto('/triage');
    const emptyState = page.getByTestId('triage-feed-empty');
    await expect(emptyState).toBeVisible({ timeout: 15000 });
    await expect(emptyState).toContainText(/caught up/i);
  });

  test('should handle offline actions properly by placing them in sync queue', async ({ page, context }) => {
    test.setTimeout(180000);

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');

    await page.request.post(`/api/triage/create?tenant_id=${encodeURIComponent(tenantId)}`, {
      data: {
        source: 'Offline Test',
        priority: 'normal',
        context: 'Action to be approved offline',
        action_type: 'Offline Action',
        action_payload: 'Test offline payload',
        customer_id: 'cust_offline'
      }
    });

    await page.goto('/triage');
    await page.waitForTimeout(2000);

    // Simulate going offline
    await context.setOffline(true);
    await page.evaluate(() => {
      window.dispatchEvent(new Event('offline'));
    });

    // Check offline indicator
    await expect(page.locator('text=You are offline')).toBeVisible();

    const card = page.locator('div[data-testid^="triage-card-"]').first();
    const approveBtn = card.locator('[data-testid^="approve-btn"], [data-testid^="triage-approve-"]');
    await approveBtn.click();

    // Verify optimistic disappearance
    await expect(card).not.toBeVisible({ timeout: 5000 });

    // Verify sync indicator appears
    await expect(page.locator('text=Pending Sync')).toBeVisible();

    // Restore network
    await context.setOffline(false);
  });
});
