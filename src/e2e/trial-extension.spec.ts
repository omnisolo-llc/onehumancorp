import { test, expect } from './fixtures';
import { createEntitlementOwner, expectEntitlementUnchanged, trackTrialClaims } from './support/entitlement_fixture';

for (const path of ['/trial-extension.html', '/ui/trial-extension.html']) {
  test(`${path} preserves plan review without share-to-Pro claims`, async ({ page, baseURL }) => {
    const fixture = await createEntitlementOwner(page, baseURL);
    const claims = trackTrialClaims(page);
    await page.goto(path);
    await expect(page.getByRole('heading', { name: 'Plan and Trial Availability' })).toBeVisible();
    await expect(page.getByRole('status')).toContainText('durable grant is not verified');
    await expect(page.getByRole('button', { name: /Share on X|Unlock 7 Days/ })).not.toBeVisible();
    await expect(page.getByRole('link', { name: 'Review plans' })).toHaveAttribute('href', '/pricing');
    await page.getByRole('link', { name: 'Check current plan' }).click();
    await expect(page).toHaveURL(/\/trial-extension$/);
    await expect(page.getByRole('status', { name: 'Current plan' })).toHaveText('Current verified plan: Free.');
    await page.getByRole('button', { name: 'Check trial availability' }).click();
    await expect(page.getByText(/durable grant is not verified/)).toBeVisible();
    await page.goBack();
    await expect(page).toHaveURL(new URL(path, baseURL).href);
    await page.getByRole('link', { name: 'Back to Dashboard' }).click();
    await expect(page).toHaveURL(/\/dashboard$/);
    expect(claims).toEqual([]);
    await expectEntitlementUnchanged(page, fixture);
  });
}
