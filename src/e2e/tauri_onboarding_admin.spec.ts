import { test, expect } from './onboarding_fixtures';

test.describe('Legacy authenticated onboarding team review', () => {
  test('Uses the signed-in account without collecting replacement credentials', async ({ page, onboardingOwner }) => {
    await page.route('**/api/v1/auth/session-identity', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({ userId: onboardingOwner.userId, tenantId: onboardingOwner.tenantId, expiresAt: Date.now() + 3600000 }) });
    });
    // Navigate to the onboarding route directly
    await page.goto('/setup.html');

    // We expect the setup to redirect or load the initial step
    await page.waitForSelector('#step-initial .next-step-btn');
    await page.locator('#step-initial [data-next="step-context"]').click();

    // Step Context
    await page.waitForSelector('#step-context:not([style*="display: none"])');
    await page.click('[data-testid="context-local"]');
    await page.click('#step-context .next-step-btn');

    // Step Categories
    await page.waitForSelector('#step-categories:not([style*="display: none"])');

    await page.locator('#business-categories').selectOption('Handyman');
    await page.click('#step-categories .next-step-btn');

    // Step Name
    await page.waitForSelector('#step-name:not([style*="display: none"])');
    await page.fill('#business-name', 'Test Business');
    await page.click('#step-name .next-step-btn');

    // Step Assistant
    await page.waitForSelector('#step-assistant:not([style*="display: none"])');
    await page.getByTestId('team-operations').click();
    await page.locator('#assistant-tone').selectOption('Friendly');
    await page.click('#step-assistant .next-step-btn');

    // Existing step IDs stay stable for saved business drafts.
    const teamReview = page.locator('#step-admin');
    await expect(teamReview).toBeVisible();
    await expect(teamReview.getByRole('heading', { name: 'Review your team' })).toBeVisible();
    await expect(page.locator('input[type="password"], #admin-email, #admin-name')).toHaveCount(0);
    await teamReview.locator('.next-step-btn').click();
    await expect(page.locator('#step-offer')).toBeVisible();

    const credentialFields = await page.evaluate(() => {
      const keys = Object.keys(localStorage).filter(k => k.startsWith('omnisolo_onboarding_owned_v1:') && k.endsWith(':draft'));
      const payloadStr = keys.length ? localStorage.getItem(keys[0]) : '{}';
      const payload = JSON.parse(payloadStr || '{}');
      const draft = payload.state || {};
      return ['admin_name', 'admin_email', 'admin_password'].filter(field => Object.hasOwn(draft, field));
    });
    expect(credentialFields).toEqual([]);
  });
});
