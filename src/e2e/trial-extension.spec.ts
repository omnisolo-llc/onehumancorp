import { test, expect } from './fixtures';
import { expectEntitlementUnchanged, trackTrialClaims } from './support/entitlement_fixture';
import { useTrialSeedOwner } from './support/trial_retirement_fixture';

for (const path of ['/trial-extension', '/trial-extension.html', '/ui/trial-extension.html']) {
  for (const plan of ['Free', 'Pro'] as const) {
    test(`${path} preserves the real ${plan} plan without share-to-Pro claims`, async ({ page, baseURL, loginAs }) => {
      const fixture = await useTrialSeedOwner(page, loginAs, plan);
      const claims = trackTrialClaims(page);
      await page.goto(path);
      await expect(page).toHaveURL(new URL('/trial-extension', baseURL).href);
      await expect(page.getByRole('heading', { name: 'Plan and Trial Availability' })).toBeVisible();
      await expect(page.getByRole('status', { name: 'Current plan' })).toHaveText(`Current verified plan: ${plan}.`);
      await expect(page.getByRole('button', { name: /Share on X|Unlock 7 Days/ })).not.toBeVisible();
      await expect(page.getByRole('link', { name: 'Review plans' })).toHaveAttribute('href', '/pricing');
      await page.getByRole('button', { name: 'Check trial availability' }).click();
      await expect(page.getByText(/durable grant is not verified/)).toBeVisible();
      const refreshedPlan = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/billing/my-plan' && response.request().method() === 'GET');
      await page.getByRole('button', { name: 'Refresh current plan' }).click();
      const refreshResponse = await refreshedPlan;
      expect(refreshResponse.status()).toBe(200);
      expect((await refreshResponse.json()).current_plan.toLowerCase()).toBe(plan.toLowerCase());
      await expect(page.getByRole('status', { name: 'Current plan' })).toHaveText(`Current verified plan: ${plan}.`);
      await page.getByRole('link', { name: 'Review plans' }).click();
      await expect(page).toHaveURL(/\/pricing$/);
      await page.goBack();
      await expect(page).toHaveURL(new URL('/trial-extension', baseURL).href);
      await page.getByRole('link', { name: 'Back to Dashboard' }).click();
      await expect(page).toHaveURL(/\/dashboard$/);
      expect(claims).toEqual([]);
      await expectEntitlementUnchanged(page, fixture);
    });
  }
}
