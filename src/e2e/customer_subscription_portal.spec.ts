import { test, expect } from '@playwright/test';
import { e2ePgQuery } from './utils/pg';

test.describe('Customer Subscription Portal', () => {
  // Use a fixed id for testing
  const subscriptionId = 'sub_test_123';
  const tenantId = 'e2e-tenant-subscriptions';

  test.beforeAll(async () => {
    // Seed real test data into the DB
    await e2ePgQuery(`
      INSERT INTO tenants (id, name, created_at, updated_at)
      VALUES ($1, 'Subscription Test Tenant', NOW(), NOW())
      ON CONFLICT (id) DO NOTHING;
    `, [tenantId]);

    await e2ePgQuery(`
      INSERT INTO customers (id, tenant_id, name, email, created_at, updated_at)
      VALUES ('cust_test_123', $1, 'Test Customer', 'test@test.com', NOW(), NOW())
      ON CONFLICT (id) DO NOTHING;
    `, [tenantId]);

    await e2ePgQuery(`
      INSERT INTO products (id, tenant_id, title, description, price_cents, status, created_at, updated_at)
      VALUES ('prod_test_123', $1, 'Artisan Coffee Blend', 'Delicious coffee', 2400, 'active', NOW(), NOW())
      ON CONFLICT (id) DO NOTHING;
    `, [tenantId]);

    await e2ePgQuery(`
      INSERT INTO subscription_plans (id, tenant_id, product_id, interval, interval_count, status, discount_percentage, created_at, updated_at)
      VALUES ('plan_test_123', $1, 'prod_test_123', 'Monthly', 1, 'active', 10, NOW(), NOW())
      ON CONFLICT (id) DO UPDATE SET discount_percentage = 10;
    `, [tenantId]);

    await e2ePgQuery(`
      INSERT INTO subscriptions (id, tenant_id, customer_id, plan_id, status, current_period_start, current_period_end, cancel_at_period_end, created_at, updated_at)
      VALUES ($2, $1, 'cust_test_123', 'plan_test_123', 'Active', NOW(), '2023-11-15 00:00:00', FALSE, NOW(), NOW())
      ON CONFLICT (id) DO UPDATE SET status = 'Active', current_period_end = '2023-11-15 00:00:00';
    `, [tenantId, subscriptionId]);
  });

  test.beforeEach(async ({ page, context }) => {
    // Inject the mock token setup if needed or just rely on public route.
    // Our route expects the subscriptionId to work out of the box with the magic-link endpoint
    // which expects a tenant. For E2E we can set a cookie or header, or since it's a test:
    // the simplest is to ensure the auth_utils parses the tenant_id via cookie.
    await context.addCookies([
      { name: 'tenant_id', value: tenantId, domain: '127.0.0.1', path: '/' },
      { name: 'tenant_id', value: tenantId, domain: 'localhost', path: '/' },
    ]);

    // Navigate directly to the real customer portal
    await page.goto(`/customer/subscriptions/${subscriptionId}`);
    await expect(page.getByRole('heading', { name: 'Manage Subscription' })).toBeVisible({ timeout: 10000 });
  });

  test('displays subscription details correctly from real db', async ({ page }) => {
    await expect(page.getByText('Artisan Coffee Blend')).toBeVisible();
    await expect(page.getByText('Monthly')).toBeVisible();
    await expect(page.getByText('Active').first()).toBeVisible();
    await expect(page.getByText('$21.60')).toBeVisible();
    // Next delivery date from db
    await expect(page.getByText('2023-11-15')).toBeVisible();
  });

  test('allows customer to skip next delivery via real endpoint', async ({ page }) => {
    const skipButton = page.getByRole('button', { name: 'Skip Next Delivery' });
    await expect(skipButton).toBeEnabled();
    await skipButton.click();

    // Verify the state update
    await expect(page.getByText('Your next delivery has been skipped.')).toBeVisible({ timeout: 10000 });
  });

  test('allows customer to pause subscription via real endpoint', async ({ page }) => {
    const pauseButton = page.getByRole('button', { name: 'Pause Subscription' });
    await expect(pauseButton).toBeEnabled();
    await pauseButton.click();

    // Verify the state update
    await expect(page.getByText('Your subscription has been paused.')).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('Paused').first()).toBeVisible();

    // The button should now say "Subscription Paused" and be disabled
    const pausedButton = page.getByRole('button', { name: 'Subscription Paused' });
    await expect(pausedButton).toBeDisabled();

    // Skip should also be disabled if paused
    const skipButton = page.getByRole('button', { name: 'Skip Next Delivery' });
    await expect(skipButton).toBeDisabled();
  });

  test('allows customer to cancel subscription via real endpoint', async ({ page }) => {
    // Make sure we are active before this test
    await e2ePgQuery(`UPDATE subscriptions SET status = 'Active' WHERE id = $1`, [subscriptionId]);
    await page.reload();
    await expect(page.getByText('Active').first()).toBeVisible();

    const cancelButton = page.getByRole('button', { name: 'Cancel Subscription' });
    await expect(cancelButton).toBeEnabled();
    await cancelButton.click();

    // Verify the state update
    await expect(page.getByText('Your subscription has been cancelled.')).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('Cancelled').first()).toBeVisible();

    // Next delivery should be replaced by '-'
    const nextDeliveryRow = page.locator('div').filter({ hasText: /^Next Delivery-$/ });
    await expect(nextDeliveryRow).toBeVisible();

    // Action buttons should disappear, replaced by cancellation text
    await expect(page.getByRole('button', { name: 'Pause Subscription' })).not.toBeVisible();
    await expect(page.getByRole('button', { name: 'Skip Next Delivery' })).not.toBeVisible();
    await expect(page.getByText('You have cancelled this subscription.')).toBeVisible();
  });
});
