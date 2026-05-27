import { test, expect } from '@playwright/test';
import { E2E_ADMIN_USER } from './fixtures';

test.describe('Department Orchestration - AI Agent Approvals', () => {
  test('Test 1: Inbox navigation', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill(E2E_ADMIN_USER.email);
    await page.locator('input[type="password"]').fill(E2E_ADMIN_USER.password);
    await page.locator('button:has-text("Login")').click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    await page.goto('/team');
    await expect(page.getByRole('heading', { name: 'Your Team' })).toBeVisible();
    await expect(page.getByText('The Ambassador')).toBeVisible();
    await expect(page.getByText('The Manager')).toBeVisible();
  });

  test('Test 2: Customer Success default state', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill(E2E_ADMIN_USER.email);
    await page.locator('input[type="password"]').fill(E2E_ADMIN_USER.password);
    await page.locator('button:has-text("Login")').click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    await page.goto('/team');
    await expect(page.getByText('The Ambassador')).toBeVisible();
    await page.getByText('The Ambassador').click();
    await expect(page.getByText('All caught up!')).toBeHidden();
  });

  test('Test 3: Operations default state', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill(E2E_ADMIN_USER.email);
    await page.locator('input[type="password"]').fill(E2E_ADMIN_USER.password);
    await page.locator('button:has-text("Login")').click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    await page.goto('/team');
    await expect(page.getByText('The Manager')).toBeVisible();
    await page.getByText('The Manager').click();
    await expect(page.getByText('All caught up!')).toBeVisible();
  });

  test('Test 4: Trigger CUJ Webhook & Flow', async ({ page, request }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill(E2E_ADMIN_USER.email);
    await page.locator('input[type="password"]').fill(E2E_ADMIN_USER.password);
    await page.locator('button:has-text("Login")').click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    const response = await request.post('/api/agents/webhook', {
      data: {
        source: 'stripe',
        message: 'order_placed',
        tenant_id: 'e2e-tenant'
      },
      headers: {
        'Content-Type': 'application/json'
      }
    });
    expect(response.ok()).toBeTruthy();

    await page.goto('/team');
    await expect(page.getByText('The Ambassador')).toBeVisible();
    await page.getByText('The Ambassador').click();

    // Verify the newly drafted action is visible
    await expect(page.getByText('Send personalized thank you & shipping ETA').first()).toBeVisible({ timeout: 10000 });
  });

  test('Test 5: Approve Draft', async ({ page, request }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill(E2E_ADMIN_USER.email);
    await page.locator('input[type="password"]').fill(E2E_ADMIN_USER.password);
    await page.locator('button:has-text("Login")').click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    const response = await request.post('/api/agents/webhook', {
      data: {
        source: 'stripe',
        message: 'order_placed',
        tenant_id: 'e2e-tenant'
      },
      headers: {
        'Content-Type': 'application/json'
      }
    });
    expect(response.ok()).toBeTruthy();

    await page.goto('/team');
    await expect(page.getByText('The Ambassador')).toBeVisible();
    await page.getByText('The Ambassador').click();

    await expect(page.getByText('Send personalized thank you & shipping ETA').first()).toBeVisible({ timeout: 10000 });

    // Click Approve on the new action
    const approveButton = page.locator('button:has-text("Approve")').first();
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    await page.waitForTimeout(500);
  });

  test('Test 6: Toggle Configuration', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill(E2E_ADMIN_USER.email);
    await page.locator('input[type="password"]').fill(E2E_ADMIN_USER.password);
    await page.locator('button:has-text("Login")').click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    await page.goto('/team');
    await expect(page.getByText('The Ambassador')).toBeVisible();
    await page.getByText('The Ambassador').click();

    // Settings icon
    await page.locator('svg').nth(1).click();

    await expect(page.getByText('The Ambassador Settings')).toBeVisible();

    // Choose auto pilot mode
    await page.getByText('Auto-pilot').click();
    await page.getByRole('button', { name: 'Save Preferences' }).click();

    // Reopen and check if it saved
    await page.locator('svg').nth(1).click();
    await expect(page.getByText('The Ambassador Settings')).toBeVisible();
    await expect(page.locator('button.bg-blue-50').getByText('Auto-pilot')).toBeVisible();
  });
});
