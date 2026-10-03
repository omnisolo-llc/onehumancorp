import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';
import { createEntitlementOwner, expectEntitlementUnchanged, trackTrialClaims } from './support/entitlement_fixture';

test('viral_upgrade_paywall', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_upgrade_paywall');
});

test('dashboard referral rewards disclose the unconfigured program and retain real plan review', async ({ page, baseURL }) => {
  const fixture = await createEntitlementOwner(page, baseURL);
  const claims = trackTrialClaims(page);
  await page.goto('/dashboard');
  const heading = page.getByRole('heading', { name: 'Referral rewards', exact: true });
  await expect(heading).toBeVisible();
  const widget = page.locator('section').filter({ has: heading });
  await expect(widget.getByRole('status')).toHaveText(/unavailable until a verified program is configured/);
  await expect(widget.getByText(/\d+ \/ 3|more to unlock/)).toHaveCount(0);
  await expect(widget.getByRole('button', { name: /Copy|Share|Unlock/ })).toHaveCount(0);
  await expect(widget.getByRole('link', { name: 'Review plans' })).toHaveAttribute('href', '/pricing');
  await widget.getByRole('link', { name: 'Review plans' }).click();
  await expect(page).toHaveURL(/\/pricing$/);
  expect(claims).toEqual([]);
  await expectEntitlementUnchanged(page, fixture);
});
