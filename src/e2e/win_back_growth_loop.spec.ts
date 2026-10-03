import { test, expect } from './fixtures';
import { createEntitlementOwner, expectEntitlementUnchanged, trackTrialClaims } from './support/entitlement_fixture';

test.describe('Customer Win-back Campaign', () => {
  test('free-owner trial checks preserve the paywall, entered offer and server entitlement', async ({ page, baseURL }) => {
    const fixture = await createEntitlementOwner(page, baseURL);
    const claims = trackTrialClaims(page);
    const generations: string[] = [];
    page.on('request', request => {
      if (request.method() === 'POST' && new URL(request.url()).pathname === '/api/v1/growth/campaign/generate-win-back') generations.push(request.url());
    });
    await page.goto('/win-back');
    await expect(page.getByRole('heading', { name: 'Customer Win-back Campaign 💌' })).toBeVisible();
    await page.getByLabel('Product to Feature (Optional)').fill('Premium Leather Bag');
    await page.getByLabel('Discount Offer (%)').fill('20');
    await page.getByRole('button', { name: 'Generate Campaign Template' }).click();
    const paywall = page.getByRole('heading', { name: 'Upgrade to Pro' });
    await expect(paywall).toBeVisible();
    await page.getByRole('button', { name: 'Check trial availability' }).click();
    await expect(page.getByText(/durable grant is not verified/)).toBeVisible();
    await expect(paywall).toBeVisible();
    await expect(page.getByRole('textbox', { name: 'Campaign draft' })).not.toBeVisible();
    await page.getByRole('button', { name: 'Close paywall' }).click();
    await expect(paywall).not.toBeVisible();
    await expect(page.getByLabel('Product to Feature (Optional)')).toHaveValue('Premium Leather Bag');
    await expect(page.getByLabel('Discount Offer (%)')).toHaveValue('20');
    await page.getByRole('button', { name: 'Generate Campaign Template' }).click();
    await expect(paywall).toBeVisible();
    expect(generations).toEqual([]);
    expect(claims).toEqual([]);
    await expectEntitlementUnchanged(page, fixture);
  });

  test('an existing Pro plan can create the real offer template while delivery stays unavailable', async ({ page, baseURL }) => {
    const fixture = await createEntitlementOwner(page, baseURL, 'Pro');
    const claims = trackTrialClaims(page);
    await page.goto('/win-back');
    await page.getByLabel('Product to Feature (Optional)').fill('Owner Product');
    await page.getByLabel('Discount Offer (%)').fill('17');
    const generated = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/growth/campaign/generate-win-back' && response.request().method() === 'POST');
    await page.getByRole('button', { name: 'Generate Campaign Template' }).click();
    const response = await generated;
    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(body.subject).toContain('17% off Owner Product');
    expect(body.body).toContain('17% off');
    expect(body.body).toContain('Owner Product');
    expect(body.body).not.toContain('WINBACK');
    await expect(page.getByRole('textbox', { name: 'Campaign draft' })).toHaveValue(`Subject: ${body.subject}\n\n${body.body}`);
    await expect(page.getByText(/template does not create a discount code/i)).toBeVisible();
    await expect(page.getByRole('button', { name: 'Campaign sending unavailable' })).toBeDisabled();
    expect(claims).toEqual([]);
    await expectEntitlementUnchanged(page, fixture);
  });
});
