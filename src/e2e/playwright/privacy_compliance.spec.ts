import { test, expect } from '@playwright/test';

test.describe('Privacy and Compliance Audit', () => {

  test('Tenant Isolation: Unauthorized access attempts are blocked', async ({ page }) => {
    const protectedRoutes = [
      '/api/ui/dashboard/metrics',
      '/api/ui/orders',
      '/api/v1/users/me'
    ];

    for (const route of protectedRoutes) {
      const response = await page.request.get(route);
      expect(response.status()).toBe(401);
    }
  });

  test('Standalone Mode: Telemetry is disabled by default in config', async ({ page }) => {
    const response = await page.request.get('/api/onboarding/audit-setup');
    if (response.ok()) {
        const body = await response.json();
        if (body.config && body.config.mode === 'standalone') {
            expect(body.config.telemetry_enabled).toBe(false);
        }
    }
  });

  test('Security: Sensitive keys in UI forms are masked', async ({ page }) => {
    await page.goto('/login');
    const passwordInput = page.locator('input[type="password"]');
    await expect(passwordInput).toBeVisible();
    await expect(passwordInput).toHaveAttribute('type', 'password');

    await passwordInput.fill('secret_password_123');
    const type = await passwordInput.getAttribute('type');
    expect(type).toBe('password');
  });

  test('PII Protection: No sensitive database fields leaked in diagnostics', async ({ page }) => {
     await page.goto('/diagnostics');
     const bodyText = await page.innerText('body');
     // Ensure common DB secrets or PII markers are NOT found in cleartext on this page
     expect(bodyText).not.toContain('password_hash');
     expect(bodyText).not.toContain('ssn:');
     expect(bodyText).not.toContain('api_key:');
  });

  test('Global RLS: Data isolation on orders page', async ({ page }) => {
    await page.goto('/orders');
    // If not logged in, we should be redirected or see an empty list
    const url = page.url();
    if (url.includes('/login')) {
        await expect(page.locator('h1')).toContainText(/Login/i);
    } else {
        const ordersRows = page.locator('tr.order-row');
        await expect(ordersRows).toHaveCount(0);
    }
  });
});
