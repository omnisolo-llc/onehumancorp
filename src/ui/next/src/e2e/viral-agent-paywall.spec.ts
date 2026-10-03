import { test, expect } from '../../../../e2e/fixtures';
import { createEntitlementOwner, expectEntitlementUnchanged, trackTrialClaims } from '../../../../e2e/support/entitlement_fixture';

for (const plan of ['Free', 'Pro', 'Business'] as const) {
  test(`agent paywall preserves the actual ${plan} entitlement without share activation`, async ({ page, baseURL }) => {
    const fixture = await createEntitlementOwner(page, baseURL, plan);
    const claims = trackTrialClaims(page);
    const loaded = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/billing/my-plan' && response.status() === 200);
    await page.goto('/agents'); await loaded;
    const toggle = page.getByRole('button', { name: 'Toggle Pro Mode', exact: true });
    await expect(toggle).toHaveAttribute('aria-pressed', String(plan !== 'Free'));
    await toggle.click();
    const heading = page.getByRole('heading', { name: 'Upgrade to Pro', exact: true });
    if (plan === 'Free') {
      await expect(heading).toBeVisible();
      await expect(page.getByRole('button', { name: /Share on X.*Pro/ })).toHaveCount(0);
      await page.getByRole('button', { name: 'Check trial availability', exact: true }).click();
      await expect(page.getByRole('alert').filter({ hasText: 'Trial activation is unavailable because a durable grant is not verified.' })).toBeVisible();
      await expect(toggle).toHaveAttribute('aria-pressed', 'false');
      await page.getByRole('button', { name: 'Close', exact: true }).click();
      await expect(heading).not.toBeVisible();
    } else {
      await expect(heading).not.toBeVisible();
      await expect(toggle).toHaveAttribute('aria-pressed', 'true');
    }
    expect(claims).toEqual([]);
    await expectEntitlementUnchanged(page, fixture);
  });
}
