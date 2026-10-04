import { test, expect } from '../../../e2e/onboarding_fixtures';

test.describe('Onboarding Flow E2E', () => {
  test('Complete setup from scratch with explicit approval', async ({ page, onboardingOwner }, testInfo) => {
    // Navigate to the business setup start screen
    await page.goto('/business-setup');
    await expect(page.getByText('Your business, live in minutes.')).toBeVisible();

    // Go to onboarding
    await page.click('text=Start Business Setup');
    await expect(page).toHaveURL(/\/onboarding/);
    const identityResponse = await page.request.get('/api/v1/auth/session-identity');
    expect(identityResponse.status()).toBe(200);
    const identity = await identityResponse.json();
    expect(identity).toMatchObject({ userId: onboardingOwner.userId, tenantId: onboardingOwner.tenantId });

    // A fresh persisted step0 opens conversational setup. Choose the guided
    // path explicitly instead of treating an optional Welcome check as readiness.
    const start = page.getByRole('button', { name: 'Start My Business', exact: true });
    const conversational = page.getByPlaceholder('Type a message...', { exact: true });
    await expect(start.or(conversational)).toBeVisible();
    if (await conversational.isVisible()) await page.getByRole('button', { name: 'Back', exact: true }).click();
    await expect(start).toBeVisible();
    await start.click();

    // Chat Step 1: Business Name
    await expect(page.getByText("What's the name of your business?")).toBeVisible();
    await page.fill('input[placeholder="e.g. Maya\'s Custom Cakes"]', 'Maya Cakes');
    await page.click('text=Next');

    // Chat Step 2: Description
    await expect(page.getByText('What do you sell?')).toBeVisible();
    await page.locator('textarea[placeholder*="I bake custom vegan cakes"]').fill('Vegan cakes');
    await page.click('text=Next');

    // Chat Step 3: Location
    await expect(page.getByText('Where are you located?')).toBeVisible();
    await page.fill('input[placeholder="e.g. Portland, OR"]', 'San Francisco, CA');

    // The real intake endpoint prepares the review after the entered details.
    await page.click('text=Next');

    await expect(page.getByText('Who is your target audience?')).toBeVisible();
    await page.getByPlaceholder('e.g. Local families, Tech startups').fill('Local families');
    const intake = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/onboarding/intake' && response.request().method() === 'POST')
      .then(async response => ({ status: response.status(), body: await response.json() }));
    const [intakeResult] = await Promise.all([intake, page.getByRole('button', { name: 'Next', exact: true }).click()]);
    if (intakeResult.status === 503) {
      expect(intakeResult.body).toMatchObject({ error: 'onboarding_ai_unconfigured' });
      testInfo.annotations.push({ type: 'setup-mode', description: 'Explicit manual setup; no AI provider configured' });
      await page.getByRole('button', { name: 'Review Details Manually', exact: true }).click();
      await expect(page.getByRole('textbox', { name: 'First Product', exact: true })).toHaveValue('');
      await expect(page.getByRole('textbox', { name: 'Price', exact: true })).toHaveValue('');
    } else {
      expect(intakeResult.status).toBe(200);
      expect(intakeResult.body.initial_products.length).toBeGreaterThan(0);
      testInfo.annotations.push({ type: 'setup-mode', description: 'Configured intake endpoint returned a reviewable catalogue' });
    }

    // It should progress to Step 2: Review Details
    await expect(page.getByText('Review Details')).toBeVisible();

    // Verify some pre-filled fields from the intake response
    await expect(page.locator('input[type="text"]').first()).toBeVisible();

    // Ensure First Product is filled before continuing
    await page.getByRole('textbox', { name: 'First Product', exact: true }).fill('Vegan Birthday Cake');
    await page.getByRole('textbox', { name: 'Price', exact: true }).fill('45.00');

    // Click Continue
    await page.click('text=Continue');

    // Step 3: Style & Team
    await expect(page.getByText('Style & Team')).toBeVisible();

    // Select domain type
    await page.click('text=Custom Domain');
    await page.click('text=Free Subdomain'); // toggle back to test it

    // Setup uses the signed-in owner; it does not create replacement credentials.
    await expect(page.locator('input[type="password"]')).toHaveCount(0);
    await page.getByText('Sales Assistant', { exact: true }).click();

    // Capture real committed receipts before the action completes any navigation.
    const capture = (path: string) => page.waitForResponse(response => new URL(response.url()).pathname === path && response.request().method() === 'POST')
      .then(async response => {
        const text = await response.text();
        expect(response.status(), text).toBe(200);
        return JSON.parse(text);
      });
    const [prepared, launched] = await Promise.all([
      capture('/api/v1/onboarding/start'), capture('/api/v1/onboarding/launch'),
      page.getByRole('button', { name: 'Approve & Complete Setup' }).click(),
    ]);
    expect(prepared).toMatchObject({ success: true, status: 'prepared', organization_id: identity.tenantId, user_id: identity.userId });
    expect(typeof prepared.preparation_id).toBe('string');
    expect(prepared.preparation_id.length).toBeGreaterThan(0);
    expect(prepared.preparation.catalog).toEqual(expect.arrayContaining([expect.objectContaining({ name: 'Vegan Birthday Cake', price: '45.00' })]));
    expect(launched).toMatchObject({ success: true, status: 'launched', preparation_id: prepared.preparation_id, organization_id: identity.tenantId, user_id: identity.userId });

    // Should see loading spinner / Step 4
    // A quick local response may complete before an intermediate spinner is observed.

    // Eventually transition to Step 5 (Live)
    // The delay might take a few seconds
    await expect(page.getByText("Setup complete", { exact: true })).toBeVisible({ timeout: 15000 });
    // Read through the authenticated browser that just launched setup. The
    // separate Node request connection has been idle throughout the UI flow.
    const state = await page.evaluate(async () => {
      const response = await fetch('/api/v1/onboarding/state', { cache: 'no-store' });
      return { status: response.status, body: await response.json() };
    });
    expect(state.status).toBe(200);
    expect(state.body.preparation).toMatchObject({ status: 'launched', preparation_id: prepared.preparation_id, organization_id: identity.tenantId, user_id: identity.userId });

    // Ensure final dashboard links exist
    await expect(page.getByRole('link', { name: 'Open Assistant' })).toBeVisible();
    await expect(page.getByText('Preview Storefront')).toBeVisible();
  });
});
