import { test, expect } from './fixtures';
import { createEntitlementOwner, expectEntitlementUnchanged, trackTrialClaims } from './support/entitlement_fixture';

const routes = ['/work-intake-widget.html', '/ui/work-intake-widget.html'];

async function readEmbed(page: import('@playwright/test').Page, tenantId: string, branding: boolean) {
  const code = await page.locator('#embed-textarea').inputValue();
  const iframeSrc = code.match(/src="([^"]+)"/)?.[1];
  expect(iframeSrc).toBeTruthy();
  const url = new URL(iframeSrc!);
  expect(url.origin).toBe(new URL(page.url()).origin);
  expect(url.pathname).toBe('/api/v1/growth/work-intake/embed');
  expect(url.searchParams.get('tenant')).toBe(tenantId);
  expect(url.searchParams.get('branding')).toBe(String(branding));
  const response = await page.request.get(url.href);
  expect(response.status()).toBe(200);
  expect(response.headers()['content-type']).toContain('text/html');
  const html = await response.text();
  expect(html).toContain('Owned intake request');
  if (branding) {
    expect(code).toContain('OmniSolo');
    expect(html).toContain('Powered by OmniSolo');
  } else {
    expect(code).not.toContain('OmniSolo');
    expect(html).not.toContain('Powered by OmniSolo');
  }
}

test.describe('Work-Intake Widget Viral Loop', () => {
  for (const route of routes) {
    const suffix = route === routes[0] ? '' : ` (${route})`;
    test(`should display the soft paywall modal and handle share bypass${suffix}`, async ({ page, baseURL }) => {
      const fixture = await createEntitlementOwner(page, baseURL);
      const claims = trackTrialClaims(page);
      let popups = 0;
      page.on('popup', () => { popups += 1; });
      await page.goto(route);
      await expect(page.getByRole('heading', { name: 'Work-Intake Widget 📋' })).toBeVisible();
      await expect(page.locator('#entitlement-status')).toHaveText('Current verified plan: Free. Sharing does not change your plan.');
      await page.getByLabel('Form Title').fill('Owned intake request');
      await page.getByLabel('Theme').selectOption('dark');
      const removeBranding = page.getByLabel('Remove "OmniSolo" branding');
      await removeBranding.click();
      const modalHeading = page.getByRole('heading', { name: 'Upgrade to Pro' });
      await expect(modalHeading).toBeVisible();
      await expect(removeBranding).not.toBeChecked();
      const localFlag = await page.evaluate(() => localStorage.getItem('has_pro'));

      for (let attempt = 0; attempt < 2; attempt += 1) {
        const origin = new URL(page.url()).origin;
        const pending = page.waitForResponse(response => {
          const url = new URL(response.url());
          return url.origin === origin && url.pathname === '/api/v1/growth/trial-extension/claim'
            && response.request().method() === 'POST' && response.request().frame() === page.mainFrame();
        });
        await page.getByRole('button', { name: 'Check trial availability' }).click();
        const response = await pending;
        expect(response.status()).toBe(501);
        expect(await response.json()).toMatchObject({ success: false, code: 'capability_unavailable', capability: 'trial_entitlement' });
        await expect(page.locator('#soft-paywall-status')).toHaveText('Trial activation is unavailable. No action was completed. Branding remains enabled.');
        await expect(page.locator('#soft-paywall-status')).toBeVisible();
        await expect(modalHeading).toBeVisible();
        await expect(removeBranding).not.toBeChecked();
        expect(await page.evaluate(() => localStorage.getItem('has_pro'))).toBe(localFlag);
        await expectEntitlementUnchanged(page, fixture);
      }

      await page.getByRole('button', { name: 'Cancel', exact: true }).click();
      await expect(page.getByLabel('Form Title')).toHaveValue('Owned intake request');
      await expect(page.getByLabel('Theme')).toHaveValue('dark');
      await expect(page.frameLocator('#preview-iframe').locator('.footer')).toBeVisible();
      await expect(page.frameLocator('#preview-iframe').locator('.footer')).toContainText('Powered by OmniSolo');
      await page.getByRole('button', { name: 'Get Widget Code' }).click();
      await expect(page.getByRole('heading', { name: 'Embed Work-Intake Widget' })).toBeVisible();
      await readEmbed(page, fixture.owner.tenantId, true);
      await page.reload();
      await expect(page.locator('#entitlement-status')).toHaveText('Current verified plan: Free. Sharing does not change your plan.');
      await expect(removeBranding).not.toBeChecked();
      await expect(page.frameLocator('#preview-iframe').locator('.footer')).toBeVisible();
      await expect(page.frameLocator('#preview-iframe').locator('.footer')).toContainText('Powered by OmniSolo');
      await expectEntitlementUnchanged(page, fixture);
      expect(claims).toHaveLength(2);
      expect(popups).toBe(0);
    });

    test(`an existing Pro owner can remove rendered branding without a trial grant (${route})`, async ({ page, baseURL }) => {
      const fixture = await createEntitlementOwner(page, baseURL, 'Pro');
      const claims = trackTrialClaims(page);
      await page.goto(route);
      await expect(page.locator('#entitlement-status')).toHaveText('Current verified plan: Pro. Sharing does not change your plan.');
      await page.getByLabel('Form Title').fill('Owned intake request');
      const removeBranding = page.getByLabel('Remove "OmniSolo" branding');
      await removeBranding.click();
      await expect(removeBranding).toBeChecked();
      await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).not.toBeVisible();
      await expect(page.locator('#preview-iframe')).toHaveAttribute('src', /branding=false/);
      await expect(page.frameLocator('#preview-iframe').getByRole('heading', { name: 'Owned intake request' })).toBeVisible();
      await expect(page.frameLocator('#preview-iframe').locator('.footer')).toHaveCount(0);
      await page.getByRole('button', { name: 'Get Widget Code' }).click();
      await expect(page.getByRole('heading', { name: 'Embed Work-Intake Widget' })).toBeVisible();
      await readEmbed(page, fixture.owner.tenantId, false);
      await expectEntitlementUnchanged(page, fixture);
      expect(claims).toEqual([]);
      expect(await page.evaluate(() => localStorage.getItem('has_pro'))).toBeNull();
    });
  }
});
