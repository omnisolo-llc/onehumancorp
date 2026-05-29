import { test, expect } from './fixtures';

test.describe('Onboarding Wizard', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Maya (The Home Baker) onboarding flow', async ({ page }) => {
    // 0. Start from UI Login
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('maya@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    // Wait for Dashboard
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // 1. Acquisition & Onboarding start
    await page.goto('/onboarding');

    // Wait for the Smart Builder welcome screen (Step 1 - Chat 1)
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    const intakePromise1 = page.waitForRequest(request =>
      request.url().includes('/api/onboarding/intake') && request.method() === 'POST'
    );

    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill("Maya's Custom Cakes");
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    const intakeReq1 = await intakePromise1;
    expect(JSON.parse(intakeReq1.postData() || '{}').description).toBe("Maya's Custom Cakes");

    // Step 1 - Chat 2
    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
    await page.getByPlaceholder('e.g. I bake custom vegan cakes for weddings and parties...').fill('Custom vegan cakes');
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    // Step 1 - Chat 3
    await expect(page.getByRole('heading', { name: 'Where are you located?' })).toBeVisible();
    await page.getByPlaceholder('e.g. Portland, OR').fill('Portland, OR');

    const startPromise = page.waitForRequest(request =>
      request.url().includes('/api/onboarding/start') && request.method() === 'POST'
    );

    // Click Generate
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    await startPromise;

    // Step 2 - Review
    await expect(page.getByRole('heading', { name: "Review Details" })).toBeVisible();
    await page.getByRole('button', { name: /Continue/i }).click();

    // Step 3 - Style
    await expect(page.getByRole('heading', { name: "Style & Team" })).toBeVisible();
    await page.getByRole('button', { name: /Launch Store/i }).click();

    // Simplified Mobile First Onboarding - wait for it to generate
    // Step 2 is automatic, so wait for Step 3 directly
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });

    // Verify shareable link is present
    await expect(page.getByText('my-business.ohc.store')).toBeVisible();

    // 4. Verify Dashboard redirect and action banner
    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // Assert Database state instead of relying on exact mock banners
    // Let's verify that the name appears somewhere in the UI
    const nameLocator = page.getByText("Maya's Custom Cakes");
    await expect(nameLocator.first()).toBeVisible({ timeout: 15000 });

    // Assert Full-Stack State Verification:
    // Query the database to ensure the data was actually written
    const { Pool } = require('pg');
    const pool = new Pool({ connectionString: process.env.DATABASE_URL || 'postgresql://ohc:ohc@localhost:5432/ohc' });
    const res = await pool.query("SELECT * FROM onboarding_state WHERE state_json->>'company_name' ILIKE $1", ["%Maya%"]);
    expect(res.rows.length).toBeGreaterThan(0);

    // Also check tenants table since the UI creates a tenant
    const tenantRes = await pool.query("SELECT name FROM tenants WHERE name ILIKE $1", ["%Maya%"]);
    expect(tenantRes.rows.length).toBeGreaterThan(0);

    await pool.end();
  });

  test('Carlos (Handyman) onboarding flow', async ({ page }) => {
    // 0. Start from UI Login
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('carlos@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    // Wait for Dashboard
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // 1. Acquisition & Onboarding start
    await page.goto('/onboarding');

    // Wait for the Smart Builder welcome screen (Step 1 - Chat 1)
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    const intakePromise2 = page.waitForRequest(request =>
      request.url().includes('/api/onboarding/intake') && request.method() === 'POST'
    );

    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill("Carlos Plumbing");
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    const intakeReq2 = await intakePromise2;
    expect(JSON.parse(intakeReq2.postData() || '{}').description).toBe("Carlos Plumbing");

    // Step 1 - Chat 2
    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
    await page.getByPlaceholder('e.g. I bake custom vegan cakes for weddings and parties...').fill('Handyman services');
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    // Step 1 - Chat 3
    await expect(page.getByRole('heading', { name: 'Where are you located?' })).toBeVisible();
    await page.getByPlaceholder('e.g. Portland, OR').fill('Miami, FL');

    const startPromise2 = page.waitForRequest(request =>
      request.url().includes('/api/onboarding/start') && request.method() === 'POST'
    );

    // Click Generate
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    await startPromise2;

    // Step 2 - Review
    await expect(page.getByRole('heading', { name: "Review Details" })).toBeVisible();
    await page.getByRole('button', { name: /Continue/i }).click();

    // Step 3 - Style
    await expect(page.getByRole('heading', { name: "Style & Team" })).toBeVisible();
    await page.getByRole('button', { name: /Launch Store/i }).click();

    // Simplified Mobile First Onboarding - wait for it to generate
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });

    // 4. Verify Dashboard redirect and action banner
    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // Assert Database state instead of relying on exact mock banners
    // Let's verify that the name appears somewhere in the UI
    const nameLocator = page.getByText("Carlos Plumbing");
    await expect(nameLocator.first()).toBeVisible({ timeout: 15000 });

    // Assert Full-Stack State Verification:
    // Query the database to ensure the data was actually written
    const { Pool } = require('pg');
    const pool = new Pool({ connectionString: process.env.DATABASE_URL || 'postgresql://ohc:ohc@localhost:5432/ohc' });
    const res = await pool.query("SELECT * FROM onboarding_state WHERE state_json->>'company_name' ILIKE $1", ["%Carlos%"]);
    expect(res.rows.length).toBeGreaterThan(0);

    // Also check tenants table since the UI creates a tenant
    const tenantRes = await pool.query("SELECT name FROM tenants WHERE name ILIKE $1", ["%Carlos%"]);
    expect(tenantRes.rows.length).toBeGreaterThan(0);

    await pool.end();
  });
});
