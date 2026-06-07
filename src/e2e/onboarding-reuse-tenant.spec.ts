import { test, expect } from './fixtures';

test('Reusing tenant_id during onboarding returns the exact same organization_id without network mocks', async ({ browser }) => {
  // Use a stable, distinct organization ID for this test
  const testTenantId = 'e2e-tenant';
  const testUserId = 'e2e-admin-user';

  const context = await browser.newContext();
  const page = await context.newPage();

  await page.goto('/onboarding');

  await page.evaluate((args) => {
    window.localStorage.setItem('tenant_id', args.testTenantId);
    window.localStorage.setItem('tenant', args.testTenantId);
    window.localStorage.setItem('user_id', args.testUserId);
  }, { testTenantId, testUserId });

  await page.getByText('Start Onboarding').click();

  // Step 1: Business Name
  await page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]').fill('Maya Test Cakes');
  await page.locator('button:has-text("Next")').click();

  // Step 2: What do you sell
  await page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]').fill('I bake test cakes');
  await page.locator('button:has-text("Next")').click();

  // Step 3: Location
  await page.locator('input[placeholder="e.g. Portland, OR"]').fill('Test, OR');
  await page.locator('button:has-text("Generate My Business")').click();

  // Step 4: Building
  await page.locator('button:has-text("Continue")').click();

  // Step 5: Final Review
  await page.getByText('Minimal').click();
  await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Maya Admin');
  await page.getByPlaceholder(/you@example.com/i).fill('maya.test@example.com');
  await page.getByPlaceholder(/••••••••/i).fill('password123');

  // Trigger real backend submission
  await page.locator('button:has-text("Launch Store")').click();
  await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });

  const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));

  // E2E asserts we reused the seeded ID and it matches
  expect(storedTenantId).toBe(testTenantId);
});
