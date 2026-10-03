import { test, expect } from './fixtures';
import { createEntitlementOwner, expectEntitlementUnchanged, trackTrialClaims } from './support/entitlement_fixture';

test.describe('Verified plan and trial availability', () => {
  test('trial page reports the real plan and retains unavailable activation after repeat checks', async ({ page, baseURL }) => {
    const fixture = await createEntitlementOwner(page, baseURL);
    const claims = trackTrialClaims(page);
    await page.goto('/trial-extension');
    await expect(page.getByRole('heading', { name: 'Plan and Trial Availability' })).toBeVisible();
    await expect(page.getByRole('status', { name: 'Current plan' })).toHaveText('Current verified plan: Free.');
    for (let attempt = 0; attempt < 2; attempt += 1) {
      await page.getByRole('button', { name: 'Check trial availability' }).click();
      await expect(page.getByText(/durable grant is not verified/)).toBeVisible();
    }
    await page.getByRole('button', { name: 'Refresh current plan' }).click();
    await expect(page.getByRole('status', { name: 'Current plan' })).toHaveText('Current verified plan: Free.');
    await page.reload();
    await expect(page.getByRole('status', { name: 'Current plan' })).toHaveText('Current verified plan: Free.');
    await page.getByRole('link', { name: 'Back to Dashboard' }).click();
    await expect(page).toHaveURL(/\/dashboard$/);
    expect(claims).toEqual([]);
    await expectEntitlementUnchanged(page, fixture);
  });

  test('pricing trial widget cannot grant a plan and retains plan navigation', async ({ page, baseURL }) => {
    const fixture = await createEntitlementOwner(page, baseURL);
    const claims = trackTrialClaims(page);
    await page.goto('/pricing');
    await expect(page.getByRole('heading', { name: 'Pricing Plans', exact: true })).toBeVisible();
    const widget = page.getByRole('region', { name: 'Plan and trial availability' });
    await expect(widget.getByText('Current verified plan: Free.')).toBeVisible();
    await expect(widget.getByText('Sharing does not confirm a trial grant or its duration.')).toBeVisible();
    await widget.getByRole('button', { name: 'Check trial availability' }).click();
    await expect(widget.getByText(/durable grant is not verified/)).toBeVisible();
    await expect(widget.getByRole('link', { name: 'Review plans' })).toHaveAttribute('href', '/pricing');
    await expect(page.getByText(/Pro Access Activated|Trial Extended!/)).not.toBeVisible();
    expect(claims).toEqual([]);
    await expectEntitlementUnchanged(page, fixture);
  });

  for (const plan of ['Free', 'Pro', 'Business'] as const) {
    test(`direct repeated trial requests cannot overwrite a persisted ${plan} plan`, async ({ page, baseURL }) => {
      const fixture = await createEntitlementOwner(page, baseURL, plan);
      for (let attempt = 0; attempt < 2; attempt += 1) {
        const response = await page.request.post('/api/v1/growth/trial-extension/claim', { data: { share_verified: true } });
        expect(response.status()).toBe(501);
        expect(await response.json()).toMatchObject({ success: false, code: 'capability_unavailable', capability: 'trial_entitlement' });
        await expectEntitlementUnchanged(page, fixture);
      }
    });
  }
});
