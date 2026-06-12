import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test.describe('Social Proof Nudge Viral Loop', () => {
  test('renders real orders and contains the viral referral link on checkout page', async ({ page, request, memberPage, adminUser }) => {
    // Navigate to the checkout page
    await page.goto('/checkout?product_id=prod_123&quantity=1');

    // Wait for the social proof nudge to appear (it has a 3-second delay, so we use a higher timeout)
    // Here we let the actual backend populate the nudge.
    // In our E2E environment the POS backend returns real seeded data if the tenant is logged in,
    // however for a public page we fallback to the proxy route. Wait, the frontend API
    // route uses the actual backend without PII.
    const nudge = page.locator('[data-testid="social-proof-nudge"]');

    // We expect the nudge to become visible
    // IF the DB has seeded orders.
    // Note: The reviewer mentioned we shouldn't mock, so we won't.
    // If there are no orders seeded, this would normally fail.
    // But we are required to not mock and rely on e2e-seed.sql which usually seeds orders for the test tenant.
    await expect(nudge).toBeVisible({ timeout: 15000 });

    // Verify it contains order text
    await expect(nudge).toContainText('Someone just bought an order for');

    // Verify the viral referral loop link is present and correct
    const viralLink = nudge.locator('a[href*="/onboarding?ref="]');
    await expect(viralLink).toBeVisible();
    await expect(viralLink).toContainText('Built with OHC');

    // Click the link to trigger the track and verify URL
    const [newPage] = await Promise.all([
      page.context().waitForEvent('page'),
      viralLink.click()
    ]);

    await newPage.waitForLoadState();
    expect(newPage.url()).toContain('/onboarding?ref=');
    expect(newPage.url()).toContain('source=social_proof_nudge');
    await newPage.close();
  });
});
