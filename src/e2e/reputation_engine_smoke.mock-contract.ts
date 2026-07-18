import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('reputation_engine_smoke', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'reputation_engine_smoke');
});

test.describe('Autonomous Reputation and Referral Engine', () => {
  test('should display stats and run simulations correctly', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    // 1. Go to dashboard and find the Reputation Engine link
    await page.goto('/dashboard.html');
    await page.waitForLoadState('networkidle');

    const engineLink = page.locator('#reputation-engine-link');
    await expect(engineLink).toBeVisible();
    await engineLink.click();

    // 2. We should be on the reputation engine page
    await page.waitForURL('**/reputation-engine.html');
    await expect(page.locator('h1', { hasText: 'Reputation & Referral Engine' })).toBeVisible();

    // 3. Verify initial stats exist (they might be zero or have some existing value from other tests)
    const reviewsEl = page.locator('#stat-reviews');
    const creditsEl = page.locator('#stat-credits');
    await expect(reviewsEl).toBeVisible();
    await expect(creditsEl).toBeVisible();

    // 4. Simulate a service event
    const customerInput = page.locator('#sim-customer-id');
    const simEventBtn = page.locator('#btn-sim-event');

    const uniqueCustomer = `AutoTest_${Date.now()}`;
    await customerInput.fill(uniqueCustomer);
    await simEventBtn.click();

    // Wait for the simulation to finish (the button text changes back and it re-enables)
    await expect(simEventBtn).toBeEnabled();

    // Check logs for success
    const eventLogs = page.locator('#event-logs');
    await expect(eventLogs).toContainText('Review saved');
    await expect(eventLogs).toContainText('generated');

    // 5. Read the generated referral code
    const referralCodeInput = page.locator('#sim-referral-code');
    await expect(referralCodeInput).not.toBeEmpty();
    const refCode = await referralCodeInput.inputValue();
    expect(refCode.length).toBeGreaterThan(5);

    // 6. Simulate checkout using the generated referral code
    const simCheckoutBtn = page.locator('#btn-sim-checkout');
    await simCheckoutBtn.click();

    // Wait for checkout simulation to finish
    await expect(simCheckoutBtn).toBeEnabled();

    // Check logs
    await expect(eventLogs).toContainText('Ledger updated');
    await expect(eventLogs).toContainText('Credited 10');

    // Wait a brief moment for the stat fetch to complete
    await page.waitForTimeout(500);

    // Verify stats were updated (Note: this runs in a shared environment so we can't assert exact numbers,
    // but we can assert they aren't completely broken. The log assertions prove the backend processed them).
    await expect(creditsEl).not.toHaveText('$0.00');
  });
});
