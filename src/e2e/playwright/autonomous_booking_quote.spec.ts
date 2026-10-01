import { randomUUID } from 'node:crypto';
import { test, expect } from '../fixtures';
import { e2eDbQuery } from '../db_utils';
import { createOwnerQuote } from './quote_fixture';

test.describe('Owner service and quote review', () => {
  test('creates a service, reviews its persisted quote and saves pricing edits', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');
    await page.getByRole('button', { name: 'Quick Actions', exact: true }).click();
    await page.getByRole('link', { name: 'New Service' }).click();
    await expect(page.getByRole('heading', { name: 'Add Service', exact: true })).toBeVisible();

    const title = `Sink Repair ${randomUUID()}`;
    await page.getByLabel('Service Title', { exact: true }).fill(title);
    await page.getByLabel('Price', { exact: true }).fill('50');
    await page.getByLabel('Description', { exact: true }).fill('Fix leaky sinks and replace pipes.');
    const serviceResponsePromise = page.waitForResponse((response) =>
      new URL(response.url()).pathname === '/api/v1/booking/services'
      && response.request().method() === 'POST');
    await page.getByRole('button', { name: 'Save Service', exact: true }).click();
    const serviceResponse = await serviceResponsePromise;
    expect(serviceResponse.ok(), await serviceResponse.text()).toBe(true);
    const service = await serviceResponse.json();
    expect(service.success).toBe(true);
    expect(service.service_id).toMatch(/^[0-9a-f-]{36}$/);
    await expect(page.getByRole('heading', { name: 'Service Saved!', exact: true })).toBeVisible();
    const [persistedService] = await e2eDbQuery(
      'SELECT tenant_id, name AS title, (price * 100)::BIGINT AS price_cents FROM services WHERE id = $1', [service.service_id],
    );
    expect(persistedService).toMatchObject({ tenant_id: adminUser.organizationId, title });
    expect(Number(persistedService.price_cents)).toBe(5000);
    await page.getByRole('link', { name: 'Back to dashboard', exact: true }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard', exact: true })).toBeVisible();

    // Service creation does not invoke an AI draft or schedule customer visits.
    // Exercise the existing owner-priced quote API, then the rendered review UI.
    // AI parsing and usage accounting retain their separate backend tests.
    const { quoteId, customerId } = await createOwnerQuote(page, adminUser.organizationId, {
      description: title, serviceId: service.service_id,
    });
    await page.goto(`/ui/quote.html?id=${quoteId}&mode=owner`);
    await expect(page.locator('#line-items-container')).toContainText(title);
    await expect(page.locator('#quote-total')).toHaveText('$50.00');
    await expect(page.locator('#deposit-amount')).toHaveText('$10.00');
    await expect(page.getByRole('button', { name: 'Approve quote', exact: true })).toBeVisible();
    await expect(page.locator('.glassmorphism').first()).toBeVisible();

    await page.getByRole('button', { name: 'Edit Quote', exact: true }).click();
    await expect(page.locator('#edit-quote-sheet')).toBeVisible();
    await page.locator('.edit-price').fill('45');
    await page.locator('#edit-total-amount').fill('45');
    await page.locator('#edit-required-deposit').fill('15');
    const updatePromise = page.waitForResponse((response) =>
      new URL(response.url()).pathname === `/api/v1/quotes/${quoteId}`
      && response.request().method() === 'PUT');
    await page.getByRole('button', { name: 'Save Changes', exact: true }).click();
    const updated = await updatePromise;
    expect(updated.ok(), await updated.text()).toBe(true);
    await expect(page.locator('#edit-quote-sheet')).toBeHidden();
    await page.reload();
    await expect(page.locator('#quote-total')).toHaveText('$45.00');
    await expect(page.locator('#deposit-amount')).toHaveText('$15.00');
    await expect(page.locator('#line-items-container')).toContainText('$45.00');

    const persisted = await page.request.get(`/api/v1/quotes/${quoteId}`);
    expect(persisted.ok()).toBe(true);
    expect(await persisted.json()).toMatchObject({
      quote: {
        id: quoteId, tenant_id: adminUser.organizationId, customer_id: customerId,
        service_id: service.service_id, status: 'DRAFT',
        total_amount_cents: 4500, required_deposit_cents: 1500,
      },
      line_items: [{ description: title, unit_price_cents: 4500, quantity: 1 }],
    });
  });
});
