import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';
import { createEntitlementOwner, expectEntitlementUnchanged, trackTrialClaims } from './support/entitlement_fixture';

test.describe('Post-Purchase Share Widget Generator', () => {
  for (const plan of ['Free', 'Pro', 'Business'] as const) {
    test(`preserves the real ${plan} entitlement while generating an authenticated offer draft`, async ({ page, baseURL }) => {
      const fixture = await createEntitlementOwner(page, baseURL, plan);
      const claims = trackTrialClaims(page);
      await page.addInitScript(() => {
        localStorage.setItem('has_pro', 'true');
        localStorage.setItem('tenant', 'forged-local-tenant');
      });
      await page.setViewportSize({ width: 1440, height: 900 });
      await page.goto('/post-purchase-share.html');
      await expect(page.getByRole('heading', { name: 'Post-Purchase Share Widget', exact: true })).toBeVisible();
      const checkbox = page.getByLabel(/Remove "OmniSolo"/);
      await expect(checkbox).toBeEnabled();
      await checkbox.click();
      if (plan === 'Free') {
        await expect(page.getByRole('heading', { name: 'Upgrade to Pro', exact: true })).toBeVisible();
        await expect(page.getByText(/Sharing does not change your plan/)).toBeVisible();
        await expect(checkbox).not.toBeChecked();
        await expect(page.locator('#preview-branding')).toBeVisible();
        await page.getByRole('button', { name: 'Close upgrade options', exact: true }).click();
      } else {
        await expect(checkbox).toBeChecked();
        await expect(page.locator('#preview-branding')).not.toBeVisible();
      }
      await expect(page.locator('#preview-link')).toHaveValue('');
      await expect(page.getByRole('button', { name: 'Link unavailable', exact: true })).toBeDisabled();
      await page.getByRole('button', { name: 'Get Embed Code', exact: true }).click();
      await expect(page.getByText('Authenticated preview code. Public embedding and discount fulfillment are not configured.', { exact: true })).toBeVisible();
      const code = await page.locator('#embed-code').inputValue();
      const source = code.match(/src="([^"]+)"/);
      expect(source).not.toBeNull();
      const preview = new URL(source![1]);
      expect(preview.origin).toBe(new URL(page.url()).origin);
      expect(preview.searchParams.get('tenant')).toBe(fixture.owner.tenantId);
      expect(preview.searchParams.get('hideBranding')).toBe(String(plan !== 'Free'));
      // A forged hideBranding query still requires the actual current server plan.
      preview.searchParams.set('hideBranding', 'true');
      const response = await page.request.get(preview.toString());
      expect(response.status()).toBe(200);
      expect(response.headers()['cache-control']).toContain('no-store');
      const html = await response.text();
      expect(html).toContain('offer-draft');
      expect(html).toContain('not configured');
      expect(html).not.toContain('Share and Get');
      expect(html).not.toContain('/referrals/click');
      expect(html.includes('⚡ OmniSolo')).toBe(plan === 'Free');
      await page.getByRole('button', { name: 'Close', exact: true }).click();
      expect(claims).toEqual([]);
      await expectEntitlementUnchanged(page, fixture);
    });
  }

  test('Smoke test: post_purchase_share_widget', async ({ page, request }) => {
    await currentAppSmoke(page, request, 'post_purchase_share_widget');
  });
});
