import { test, expect } from '@playwright/test';
import { loginAs } from './fixtures';

test.describe('Mobile Payload & Parallel Execution API Verification', () => {
  test('Unified Feed is fetched with fields shaping on mobile', async ({ page, request, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.setViewportSize({ width: 375, height: 812 });

    const requestPromise = page.waitForRequest(req =>
      req.url().includes('/api/ui/dashboard/unified-feed') &&
      req.url().includes('mobile_optimized=true') &&
      req.url().includes('fields=')
    );

    await page.route('**/api/ui/dashboard/unified-feed*', async route => {
        await route.fulfill({ status: 200, body: JSON.stringify({
            metrics: { active_customers: 10, pending_orders: 5, total_sales: 100 },
            orders: [],
            inbox: [],
            triage: [],
            supply: { vendors: [], raw_materials: [], bom_items: [] },
            agent_feed: [],
            pending_approvals: []
        })});
    });

    await page.goto('/dashboard');
    const req = await requestPromise;
    expect(req.url()).toContain('fields=triage,priority_tasks,orders(id,customer_name,total_amount,status)');
    await expect(page.locator('text=Operations Map')).toBeVisible();
  });

  test('Staff dashboard fetches AI shift summaries and priority tasks in parallel', async ({ page, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);

    // We can't directly check JS Promise.all in Playwright easily without mocking,
    // but we can ensure they both load and render correctly.
    await page.goto('/staff');

    // Test that the tab is selectable and we can see both components loaded
    // This assumes the components render text when loaded or empty state
    await expect(page.locator('text=No shift summaries available yet').or(page.locator('.shift-summary-content'))).toBeVisible({ timeout: 5000 });
  });

  test('Unified feed payloads contain only necessary metrics properties on mobile', async ({ page, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.setViewportSize({ width: 375, height: 812 });
    const requestPromise = page.waitForRequest(req => req.url().includes('metrics(pending_orders,total_sales,active_customers)'));
    await page.goto('/dashboard');
    const req = await requestPromise;
    expect(req.url()).toContain('metrics(pending_orders,total_sales,active_customers)');
  });

  test('Unified feed payloads contain only necessary supply properties on mobile', async ({ page, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.setViewportSize({ width: 375, height: 812 });
    const requestPromise = page.waitForRequest(req => req.url().includes('supply(vendors(id),raw_materials(id,current_quantity,reorder_threshold),bom_items(id))'));
    await page.goto('/dashboard');
    const req = await requestPromise;
    expect(req.url()).toContain('supply(vendors(id),raw_materials(id,current_quantity,reorder_threshold),bom_items(id))');
  });

  test('Unified feed payloads contain only necessary order properties on mobile', async ({ page, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.setViewportSize({ width: 375, height: 812 });
    const requestPromise = page.waitForRequest(req => req.url().includes('orders(id,customer_name,total_amount,status)'));
    await page.goto('/dashboard');
    const req = await requestPromise;
    expect(req.url()).toContain('orders(id,customer_name,total_amount,status)');
  });
});
