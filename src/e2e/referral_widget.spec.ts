import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';
import { createEntitlementOwner, expectEntitlementUnchanged, trackTrialClaims } from './support/entitlement_fixture';

test.describe('Referral Widget Entitlement', () => {
  test('keeps free branding and the paywall when no durable trial grant exists', async ({ page, baseURL }) => {
    const fixture = await createEntitlementOwner(page, baseURL);
    const claims = trackTrialClaims(page);
    await page.addInitScript(() => localStorage.setItem('has_pro', 'true'));
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.goto('/referral-widget');
    await expect(page.getByRole('heading', { name: 'Referral Widget Builder' })).toBeVisible();
    const branding = page.getByRole('link', { name: /Powered by OmniSolo/i });
    const checkbox = page.getByLabel('Remove "OmniSolo" Branding');
    await expect(branding).toBeVisible();
    await checkbox.click();
    const paywall = page.getByRole('heading', { name: 'Upgrade to Pro' });
    await expect(paywall).toBeVisible();
    await expect(page.getByText(/Make the Referral Widget 100% yours/)).toBeVisible();
    await page.getByRole('button', { name: 'Check trial availability' }).click();
    await expect(page.getByText(/durable grant is not verified/)).toBeVisible();
    await expect(paywall).toBeVisible();
    await expect(checkbox).not.toBeChecked();
    await expect(branding).toBeVisible();
    await page.getByRole('button', { name: 'Close paywall' }).click();
    await expect(paywall).not.toBeVisible();
    await page.getByRole('button', { name: 'Get Embed Code' }).click();
    await expect(page.locator('textarea')).toHaveValue(/hide_branding=false/);
    await page.getByRole('button', { name: 'Close', exact: true }).click();
    await page.reload();
    await expect(checkbox).not.toBeChecked();
    await expect(branding).toBeVisible();
    expect(claims).toEqual([]);
    await expectEntitlementUnchanged(page, fixture);
  });

  test('an existing server-verified Pro plan retains branding controls', async ({ page, baseURL }) => {
    const fixture = await createEntitlementOwner(page, baseURL, 'Pro');
    const loadedPlan = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/billing/my-plan' && response.status() === 200);
    await page.goto('/referral-widget');
    await loadedPlan;
    const checkbox = page.getByLabel('Remove "OmniSolo" Branding');
    await checkbox.check();
    await expect(checkbox).toBeChecked();
    await expect(page.getByRole('link', { name: /Powered by OmniSolo/i })).not.toBeVisible();
    await page.getByRole('button', { name: 'Get Embed Code' }).click();
    await expect(page.locator('textarea')).toHaveValue(/hide_branding=true/);
    await page.getByRole('button', { name: 'Close', exact: true }).click();
    await checkbox.uncheck();
    await expect(page.getByRole('link', { name: /Powered by OmniSolo/i })).toBeVisible();
    await expectEntitlementUnchanged(page, fixture);
  });

  test('Smoke test: referral_widget', async ({ page, request, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await currentAppSmoke(page, request, 'referral_widget');
  });
});
