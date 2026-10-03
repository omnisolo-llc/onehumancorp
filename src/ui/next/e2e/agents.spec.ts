import { test, expect } from '../../../e2e/fixtures';
import { createEntitlementOwner, expectEntitlementUnchanged, trackTrialClaims } from '../../../e2e/support/entitlement_fixture';

test.describe('AI Agent Department Architecture', () => {
  test('should display approval inbox and activity feed', async ({ page }) => {
    // Navigate to agents page
    await page.goto('/agents');

    // Ensure "My Team" tab is visible
    await expect(page.locator('text=My Team')).toBeVisible();
    await expect(page.locator('text=The Manager')).toBeVisible();

    // Navigate to "Activity Feed" tab
    await page.locator('text=Activity Feed').click();
    await expect(page.locator('text=Fetching feed...').or(page.locator('text=No activity yet.'))).toBeVisible();

    // Navigate to "Needs Approval" tab
    await page.locator('text=Needs Approval').click();
    await expect(page.locator('text=Fetching approvals...').or(page.locator('text=All Caught Up!')).or(page.locator('text=Approve & Send').first())).toBeVisible();
  });

  test('Pro Mode remains gated by the server plan when trial availability cannot be verified', async ({ page, baseURL }) => {
    const fixture = await createEntitlementOwner(page, baseURL);
    const claims = trackTrialClaims(page);
    await page.addInitScript(() => localStorage.setItem('has_pro', 'true'));
    await page.goto('/agents');
    const toggle = page.getByRole('button', { name: 'Toggle Pro Mode' });
    await expect(toggle).toHaveAttribute('aria-pressed', 'false');
    await toggle.click();
    const paywall = page.getByRole('heading', { name: 'Upgrade to Pro' });
    await expect(paywall).toBeVisible();
    await expect(page.getByRole('link', { name: 'Upgrade to Pro', exact: true })).toHaveAttribute('href', '/pricing');
    await page.getByRole('button', { name: 'Check trial availability' }).click();
    await expect(page.getByRole('alert').filter({ hasText: 'durable grant is not verified' })).toBeVisible();
    await expect(paywall).toBeVisible();
    await expect(toggle).toHaveAttribute('aria-pressed', 'false');
    await page.getByRole('button', { name: 'Close', exact: true }).click();
    await expect(paywall).not.toBeVisible();
    await toggle.click();
    await expect(paywall).toBeVisible();
    expect(claims).toEqual([]);
    await expectEntitlementUnchanged(page, fixture);
  });

  test('Pro Mode still recognizes an existing persisted Pro entitlement', async ({ page, baseURL }) => {
    const fixture = await createEntitlementOwner(page, baseURL, 'Pro');
    await page.goto('/agents');
    const toggle = page.getByRole('button', { name: 'Toggle Pro Mode' });
    await expect(toggle).toHaveAttribute('aria-pressed', 'true');
    await toggle.click();
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).not.toBeVisible();
    await expect(toggle).toHaveAttribute('aria-pressed', 'true');
    await expectEntitlementUnchanged(page, fixture);
  });
});
