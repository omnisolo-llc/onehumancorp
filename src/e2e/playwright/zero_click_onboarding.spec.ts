import { test, expect } from '../onboarding_fixtures';

test.describe('Zero-Click Onboarding Flow', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // strictly mobile viewport

  test('should complete the zero-click onboarding flow on mobile', async ({ page }) => {
    // Navigate to the real local server
    await page.goto('/setup.html');
    await expect(page).toHaveTitle(/OmniSolo|OmniSolo/);

    await page.getByRole('button', { name: 'Generate My Workspace', exact: true }).click();

    // Initial Screen
    await expect(page.locator('h1', { hasText: 'Tell us about your business' })).toBeVisible({ timeout: 15000 });

    // Check if the input loaded
    await expect(page.locator('#instant-bio')).toBeVisible();

    // Type into the input
    await page.locator('#instant-bio').fill('I am a baker in Austin selling custom cakes');

    // The instant path prepares a workspace. Completion still requires approval.
    const generated = page.waitForResponse(response => response.url().endsWith('/api/v1/onboarding/start_zero_click') && response.request().method() === 'POST')
      .then(async response => ({ status: response.status(), body: await response.json() }));
    const [preparedResponse] = await Promise.all([generated, page.locator('#generate-storefront-btn').click()]);
    expect(preparedResponse.status).toBe(200);
    const prepared = preparedResponse.body;
    expect(prepared).toMatchObject({ success: true, status: 'prepared' });
    expect(typeof prepared.preparation_id).toBe('string');
    expect(prepared.preparation_id.length).toBeGreaterThan(0);
    const identityResponse = await page.request.get('/api/v1/auth/session-identity');
    expect(identityResponse.status()).toBe(200);
    const identity = await identityResponse.json();
    expect(typeof identity.userId).toBe('string');
    expect(identity.userId.length).toBeGreaterThan(0);
    expect(typeof identity.tenantId).toBe('string');
    expect(identity.tenantId.length).toBeGreaterThan(0);
    expect(prepared.organization_id).toBe(identity.tenantId);
    expect(prepared.user_id).toBe(identity.userId);
    const approval = page.locator('#step-approval');
    await expect(approval).toBeVisible();
    const launch = page.waitForResponse(response => response.url().endsWith('/api/v1/onboarding/launch') && response.request().method() === 'POST')
      .then(async response => ({ status: response.status(), body: await response.json() }));
    // Start reading the real receipt when its response arrives, before the click
    // completes navigation and Chromium retires the previous document's body.
    const [launchedResponse] = await Promise.all([launch, approval.getByRole('button', { name: 'Approve & Complete Setup' }).click()]);
    expect(launchedResponse.status).toBe(200);
    expect(launchedResponse.body).toMatchObject({ success: true, status: 'launched', preparation_id: prepared.preparation_id, organization_id: identity.tenantId, user_id: identity.userId });

    // The acknowledged local setup leads to the dashboard.
    await expect(page).toHaveURL(/.*(dashboard\.html|dashboard|success\.html).*/, { timeout: 30000 });
  });
});
