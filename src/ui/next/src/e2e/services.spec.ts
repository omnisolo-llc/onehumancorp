import { test, expect } from './fixtures';

test.describe('ServicesPage CUJ', () => {
  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => {
      window.localStorage.clear();
      window.localStorage.setItem('tenant_id', 'test-tenant');
      window.localStorage.setItem('user_id', 'test-user');
    });

    await page.route('/api/onboarding/state', async (route) => {
      if (route.request().method() === 'POST') {
        await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ ok: true }) });
      } else {
        await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ services: [] }) });
      }
    });
  });

  test('User can create a new service and see it on the services list', async ({ page }) => {
    await page.goto('/services');
    await expect(page.getByRole('heading', { name: 'Service Manager' })).toBeVisible();
    await expect(page.getByText('No services yet')).toBeVisible();

    await page.getByRole('link', { name: 'Add Service', exact: true }).click();

    await expect(page.getByRole('heading', { name: 'Add Service' })).toBeVisible();
    await page.locator('input[placeholder="e.g. Weekly Music Tutoring"]').fill('Test Service');
    await page.locator('textarea[placeholder="Describe the service..."]').fill('This is a test service description.');
    await page.locator('input[type="number"]').fill('10.00');

    await page.route('/api/onboarding/state', async (route) => {
        await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ services: [{ title: 'Test Service', description: 'This is a test service description.', price: '10.00' }] }) });
    });

    await page.getByRole('button', { name: 'Save Service' }).click();

    await expect(page.getByText('Service Saved!')).toBeVisible();

    await page.goto('/services');
    await expect(page.getByRole('heading', { name: 'Service Manager' })).toBeVisible();

    await expect(page.getByRole('heading', { name: 'Test Service' })).toBeVisible();
    await expect(page.getByText('This is a test service description.')).toBeVisible();
    await expect(page.getByText('$10.00')).toBeVisible();
  });
});
