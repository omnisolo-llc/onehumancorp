import { expect, test } from './fixtures';

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

  test('field-ops requests are tenant-scoped by the authenticated proxy', async ({ page }) => {
    const appointmentsResponse = page.waitForResponse((response) =>
      response.url().includes('/api/v1/field-ops/appointments') && response.request().method() === 'GET',
    );
    await page.goto('/field-ops/jobs');

    const response = await appointmentsResponse;
    expect(response.status()).toBe(200);
    expect(response.url()).toContain('tenant_id=authenticated');

    const [forgedResponse, canonicalResponse] = await Promise.all([
      page.request.get('/api/v1/field-ops/appointments?tenant_id=attacker'),
      page.request.get('/api/v1/field-ops/appointments?tenant_id=e2e-tenant'),
    ]);
    expect(forgedResponse.status()).toBe(200);
    expect(canonicalResponse.status()).toBe(200);
    const appointmentIds = async (candidate: typeof forgedResponse) => {
      const body = await candidate.json();
      return body.appointments.map((appointment: { id: string }) => appointment.id).sort();
    };
    const [forgedIds, canonicalIds] = await Promise.all([
      appointmentIds(forgedResponse),
      appointmentIds(canonicalResponse),
    ]);
    expect(forgedIds).toEqual(canonicalIds);
    expect(forgedIds).toContain('e2e-appointment');
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
