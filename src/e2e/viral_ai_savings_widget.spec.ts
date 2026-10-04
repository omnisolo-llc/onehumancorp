import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';
import { createEntitlementOwner, expectEntitlementUnchanged, trackTrialClaims } from './support/entitlement_fixture';
import { captureResponseBody } from '../../scripts/playwright/response-body.mjs';

test('viral_ai_savings_widget', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_ai_savings_widget');
});

test('dashboard reports missing measured savings without sample metrics or trial activation', async ({ page, baseURL }) => {
  const fixture = await createEntitlementOwner(page, baseURL);
  const claims = trackTrialClaims(page);
  const savingsUrl = new URL('/api/v1/growth/time-savings', baseURL).href;
  // Identity revalidation can abort an earlier read after its headers arrive.
  // Observe the completed owner-bound GET, then retain its actual body.
  const savingsRead = captureResponseBody(page.waitForEvent('requestfinished', {
    predicate: request => request.url() === savingsUrl && request.method() === 'GET'
      && request.headers()['x-ohc-expected-user'] === fixture.owner.userId
      && request.headers()['x-ohc-expected-tenant'] === fixture.owner.tenantId,
  }).then(async request => {
    const response = await request.response();
    if (!response) throw new Error('Completed savings read has no response');
    return response;
  }));
  const [{ response, body }] = await Promise.all([savingsRead, page.goto('/dashboard')]);
  expect(response.status()).toBe(501);
  expect(JSON.parse((await body()).toString('utf8'))).toMatchObject({ success: false, code: 'capability_unavailable', capability: 'measured_time_savings' });
  const widget = page.getByRole('region', { name: 'Recorded time savings' });
  await expect(widget.getByRole('heading', { name: 'Recorded time savings' })).toBeVisible();
  await expect(widget.getByText('Recorded time-savings data is unavailable.')).toBeVisible();
  await expect(widget.getByText(/hours saved|inquiries handled|appointments scheduled/i)).not.toBeVisible();
  await widget.getByRole('button', { name: 'Check trial availability' }).click();
  await expect(widget.getByText(/durable grant is not verified/)).toBeVisible();
  await expect(widget.getByText(/Trial Extended!|Pro Access Activated/)).not.toBeVisible();
  expect(claims).toEqual([]);
  await expectEntitlementUnchanged(page, fixture);
});
