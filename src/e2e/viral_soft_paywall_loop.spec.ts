import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';
import { createEntitlementOwner, expectEntitlementUnchanged, trackTrialClaims } from './support/entitlement_fixture';

test('viral_soft_paywall_loop', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_soft_paywall_loop');
});

test('automation setup preserves the verified plan and does not claim activation', async ({ page, baseURL }) => {
  const fixture = await createEntitlementOwner(page, baseURL);
  const claims = trackTrialClaims(page);
  await page.goto('/dashboard');
  await expect(page.getByRole('heading', { name: 'Advanced AI Automations', exact: true })).toBeVisible();
  const open = page.getByRole('button', { name: 'Review automation setup', exact: true });
  await open.click();
  const dialog = page.getByRole('dialog', { name: 'Automation setup' });
  await expect(dialog.getByText('Current verified plan: Free.')).toBeVisible();
  await expect(dialog.getByText(/No automation is activated from this card/)).toBeVisible();
  await dialog.getByRole('button', { name: 'Check trial availability' }).click();
  await expect(dialog.getByText(/durable grant is not verified/)).toBeVisible();
  await expect(dialog).toBeVisible();
  await expect(page.getByText('✅ Enabled', { exact: true })).not.toBeVisible();
  await dialog.getByRole('button', { name: 'Close setup' }).click();
  await expect(dialog).not.toBeVisible();
  await open.click();
  await dialog.getByRole('link', { name: 'Open Agents' }).click();
  await expect(page).toHaveURL(/\/agents$/);
  await expect(page.getByRole('button', { name: 'Toggle Pro Mode' })).toHaveAttribute('aria-pressed', 'false');
  expect(claims).toEqual([]);
  await expectEntitlementUnchanged(page, fixture);
});
