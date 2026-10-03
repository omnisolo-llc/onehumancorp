import { test, expect } from '../../../../e2e/fixtures';
import { createEntitlementOwner, expectEntitlementUnchanged, trackTrialClaims } from '../../../../e2e/support/entitlement_fixture';

for (const plan of ['Free', 'Pro', 'Business'] as const) {
  test(`exit-intent template editing and copying preserve the actual ${plan} plan`, async ({ page, context, baseURL }) => {
    const fixture = await createEntitlementOwner(page, baseURL, plan);
    const claims = trackTrialClaims(page);
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);
    await page.goto('/exit-intent-builder');
    await expect(page.getByRole('heading', { name: 'Exit-Intent Pop-up Builder' })).toBeVisible();
    await expect(page.getByText(/Offer text is a draft/)).toBeVisible();
    await page.getByPlaceholder('Wait! Before you go...').fill('Owner supplied offer');
    await page.getByPlaceholder('Get 10% off your first order...').fill('Ask us about our current offer.');
    await expect(page.locator('.max-w-4xl').filter({ hasText: 'Live Preview' }).getByRole('heading', { name: 'Owner supplied offer' })).toBeVisible();
    const toggle = page.getByRole('switch', { name: 'Remove OmniSolo Branding' });
    await expect(toggle).toBeEnabled();
    await toggle.click();
    if (plan === 'Free') {
      await expect(page.getByRole('link', { name: 'Review plans' })).toHaveAttribute('href', '/pricing');
      await page.getByRole('button', { name: 'Cancel', exact: true }).click();
      await expect(toggle).toHaveAttribute('aria-checked', 'false');
    } else {
      await expect(toggle).toHaveAttribute('aria-checked', 'true');
    }
    await page.getByRole('button', { name: 'Copy to Clipboard' }).click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();
    const copied = await page.evaluate(() => navigator.clipboard.readText());
    expect(copied).toContain('Owner supplied offer');
    expect(copied).toContain('Ask us about our current offer.');
    expect(copied.includes('Powered by OmniSolo')).toBe(plan === 'Free');
    if (plan === 'Free') {
      await toggle.click(); await page.getByRole('link', { name: 'Review plans' }).click();
      await expect(page).toHaveURL(/\/pricing$/);
    }
    expect(claims).toEqual([]);
    await expectEntitlementUnchanged(page, fixture);
  });
}
