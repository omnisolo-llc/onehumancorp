import { randomUUID } from 'node:crypto';
import { expect, test } from './fixtures';
import { e2eDbTransaction } from './db_utils';

test.describe('OmniSolo browser regressions', () => {
  test('the authenticated agent feed endpoint returns real items', async ({ page }) => {
    const response = await page.request.get('/api/v1/agent-feed?limit=5&offset=0');

    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(Array.isArray(body.items)).toBe(true);
  });

  test('Global Commerce loads and saves through the backend', async ({ page }) => {
    await page.goto('/settings/global-commerce');
    await expect(page.getByText('Global Commerce')).toBeVisible();
    await expect(page.getByText('Base Currency')).toBeVisible();

    const currency = page.getByRole('combobox');
    const initialCurrency = await currency.inputValue();
    const saveCurrency = async (value: string) => {
      const saveResponse = page.waitForResponse((response) =>
        response.url().endsWith('/api/v1/settings') && response.request().method() === 'PUT',
      );
      const dialog = page.waitForEvent('dialog');
      await currency.selectOption(value);
      await page.getByRole('button', { name: 'Save Changes' }).click();
      await (await dialog).accept();
      expect((await saveResponse).status()).toBe(200);
    };

    await saveCurrency(initialCurrency);
  });

  test('field-ops requests are tenant-scoped by the authenticated proxy', async ({ page, anonymousPage, loginAs, adminUser, unlimitedAdminUser }) => {
    // Independent real rows make a global/empty/fixed fixture response fail.
    const suffix = randomUUID();
    const rows = [adminUser.organizationId, unlimitedAdminUser.organizationId].map((tenantId, index) => ({
      tenantId, id: `scope-appointment-${suffix}-${index}`, customerId: `scope-customer-${suffix}-${index}`,
      templateId: `scope-template-${suffix}-${index}`, name: `Owned appointment ${suffix} ${index}`,
    }));
    const [mine, other] = rows;
    await e2eDbTransaction(async query => {
      for (const row of rows) {
        await query('INSERT INTO customers (id, tenant_id, name) VALUES ($1, $2, $3)', [row.customerId, row.tenantId, row.name]);
        await query('INSERT INTO job_templates (id, tenant_id, name) VALUES ($1, $2, $3)', [row.templateId, row.tenantId, row.name]);
        await query(`INSERT INTO appointments (id, tenant_id, customer_id, job_template_id, status, scheduled_start_time, scheduled_end_time)
          VALUES ($1, $2, $3, $4, 'Scheduled', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP + INTERVAL '1 hour')`,
        [row.id, row.tenantId, row.customerId, row.templateId]);
      }
    });
    try {
      const identityResponse = await page.request.get('/api/v1/auth/session-identity');
      expect(identityResponse.status()).toBe(200);
      const identity = await identityResponse.json();
      expect(identity.tenantId).toBe(mine.tenantId);
      const appointmentsResponse = page.waitForResponse(response =>
        new URL(response.url()).pathname === '/api/v1/field-ops/appointments' && response.request().method() === 'GET',
      );
      await page.goto('/field-ops/jobs');
      const response = await appointmentsResponse;
      expect(response.status()).toBe(200);
      expect(new URL(response.url()).search).toBe('');
      const headers = await response.request().allHeaders();
      expect(headers['x-ohc-expected-user']).toBe(identity.userId);
      expect(headers['x-ohc-expected-tenant']).toBe(identity.tenantId);
      const canonical = await response.json();
      const assertOwned = (body: { appointments: { id: string }[] }, owned: typeof mine, foreign: typeof mine) => {
        expect(body.appointments).toEqual(expect.arrayContaining([expect.objectContaining({
          id: owned.id, customer_id: owned.customerId, customer_name: owned.name,
          job_template_id: owned.templateId, job_name: owned.name,
        })]));
        expect(body.appointments.map(row => row.id)).not.toContain(foreign.id);
      };
      assertOwned(canonical, mine, other);
      await expect(page.getByText(mine.name).first()).toBeVisible();
      await expect(page.getByText(other.name)).toHaveCount(0);

      // Every client-side selector is untrusted. The sealed session remains the authority.
      const forgedQuery = new URLSearchParams(Object.fromEntries(
        ['tenant_id', 'tenant', 'organization_id', 'org_id'].map(key => [key, other.tenantId]),
      ));
      const forged = await page.request.get(`/api/v1/field-ops/appointments?${forgedQuery}`);
      expect(forged.status()).toBe(200);
      const forgedBody = await forged.json();
      assertOwned(forgedBody, mine, other);
      expect(forgedBody.appointments.map((row: { id: string }) => row.id).sort())
        .toEqual(canonical.appointments.map((row: { id: string }) => row.id).sort());

      // The other signed owner can read its own row, including when it forges our selector.
      await loginAs(anonymousPage, unlimitedAdminUser);
      const otherIdentity = await anonymousPage.request.get('/api/v1/auth/session-identity');
      expect(otherIdentity.status()).toBe(200);
      expect((await otherIdentity.json()).tenantId).toBe(other.tenantId);
      const otherResponse = await anonymousPage.request.get(`/api/v1/field-ops/appointments?tenant_id=${encodeURIComponent(mine.tenantId)}`);
      expect(otherResponse.status()).toBe(200);
      assertOwned(await otherResponse.json(), other, mine);
    } finally {
      await e2eDbTransaction(async query => {
        for (const row of rows) {
          await query('DELETE FROM appointments WHERE id = $1 AND tenant_id = $2', [row.id, row.tenantId]);
          await query('DELETE FROM job_templates WHERE id = $1 AND tenant_id = $2', [row.templateId, row.tenantId]);
          await query('DELETE FROM customers WHERE id = $1 AND tenant_id = $2', [row.customerId, row.tenantId]);
        }
      });
    }
  });

  test('mPOS loads its catalog from the collection endpoint', async ({ page }) => {
    const catalogResponse = page.waitForResponse((response) =>
      response.url().includes('/api/v1/catalog/products') && response.request().method() === 'GET',
    );
    await page.goto('/pos/mpos?tenantId=e2e-tenant');

    expect((await catalogResponse).status()).toBe(200);
    await expect(page.getByRole('heading', { name: 'mPOS' })).toBeVisible();
  });
});
