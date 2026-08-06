import { test, expect } from '@playwright/test';

test.describe('Onboarding Navigation and Aesthetics', () => {

  test.beforeEach(async ({ page }) => {
    // Navigate using the real stack URL
    await page.goto('/setup.html');
  });

  test('Back button in target audience step navigates to location step', async ({ page }) => {
    await page.getByTestId('next-step-btn').first().click(); // to step-context
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click(); // to step-categories
    await page.locator('#business-categories').selectOption('Other');
    await page.getByTestId('next-step-btn').nth(2).click(); // to step-name
    await page.locator('#business-name').fill('My Test Business');
    await page.getByTestId('next-step-btn').nth(3).click(); // to step-assistant
    await page.getByTestId('team-support').click();
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByTestId('next-step-btn').nth(4).click(); // to step-admin
    await page.locator('#admin-name').fill('Test User');
    await page.locator('#admin-email').fill('test@test.com');
    await page.locator('#admin-password').fill('password123');
    await page.getByTestId('next-step-btn').nth(5).click(); // to step-offer
    await page.locator('#first-offer').fill('Awesome stuff');
    await page.getByTestId('next-step-btn').nth(6).click(); // to step-location
    await page.locator('#location-input').fill('Local');
    await page.getByTestId('next-step-btn').nth(7).click(); // to step-target-audience

    await expect(page.locator('body')).toContainText('Who is your target audience?');
    const targetAudienceStep = page.locator('#step-target-audience');
    await targetAudienceStep.getByTestId('prev-step-btn').click();

    await expect(page.locator('#step-location')).toBeVisible();
    await expect(page.locator('#step-target-audience')).toBeHidden();
  });

  test('Back button in location step navigates to offer step', async ({ page }) => {
    await page.getByTestId('next-step-btn').first().click(); // to step-context
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click(); // to step-categories
    await page.locator('#business-categories').selectOption('Other');
    await page.getByTestId('next-step-btn').nth(2).click(); // to step-name
    await page.locator('#business-name').fill('My Test Business');
    await page.getByTestId('next-step-btn').nth(3).click(); // to step-assistant
    await page.getByTestId('team-support').click();
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByTestId('next-step-btn').nth(4).click(); // to step-admin
    await page.locator('#admin-name').fill('Test User');
    await page.locator('#admin-email').fill('test@test.com');
    await page.locator('#admin-password').fill('password123');
    await page.getByTestId('next-step-btn').nth(5).click(); // to step-offer
    await page.locator('#first-offer').fill('Awesome stuff');
    await page.getByTestId('next-step-btn').nth(6).click(); // to step-location

    await expect(page.locator('#step-location')).toBeVisible();
    const locationStep = page.locator('#step-location');
    await locationStep.getByTestId('prev-step-btn').click();

    await expect(page.locator('#step-offer')).toBeVisible();
    await expect(page.locator('#step-location')).toBeHidden();
  });

  test('Back button in offer step navigates to admin step', async ({ page }) => {
    await page.getByTestId('next-step-btn').first().click(); // to step-context
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click(); // to step-categories
    await page.locator('#business-categories').selectOption('Other');
    await page.getByTestId('next-step-btn').nth(2).click(); // to step-name
    await page.locator('#business-name').fill('My Test Business');
    await page.getByTestId('next-step-btn').nth(3).click(); // to step-assistant
    await page.getByTestId('team-support').click();
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByTestId('next-step-btn').nth(4).click(); // to step-admin
    await page.locator('#admin-name').fill('Test User');
    await page.locator('#admin-email').fill('test@test.com');
    await page.locator('#admin-password').fill('password123');
    await page.getByTestId('next-step-btn').nth(5).click(); // to step-offer

    await expect(page.locator('#step-offer')).toBeVisible();
    const offerStep = page.locator('#step-offer');
    await offerStep.getByTestId('prev-step-btn').click();

    await expect(page.locator('#step-admin')).toBeVisible();
    await expect(page.locator('#step-offer')).toBeHidden();
  });

  test('Back button in admin step navigates to assistant step', async ({ page }) => {
    await page.getByTestId('next-step-btn').first().click(); // to step-context
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click(); // to step-categories
    await page.locator('#business-categories').selectOption('Other');
    await page.getByTestId('next-step-btn').nth(2).click(); // to step-name
    await page.locator('#business-name').fill('My Test Business');
    await page.getByTestId('next-step-btn').nth(3).click(); // to step-assistant
    await page.getByTestId('team-support').click();
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByTestId('next-step-btn').nth(4).click(); // to step-admin

    await expect(page.locator('#step-admin')).toBeVisible();
    const adminStep = page.locator('#step-admin');
    await adminStep.getByTestId('prev-step-btn').click();

    await expect(page.locator('#step-assistant')).toBeVisible();
    await expect(page.locator('#step-admin')).toBeHidden();
  });

  test('Back button in assistant step navigates to name step', async ({ page }) => {
    await page.getByTestId('next-step-btn').first().click(); // to step-context
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click(); // to step-categories
    await page.locator('#business-categories').selectOption('Other');
    await page.getByTestId('next-step-btn').nth(2).click(); // to step-name
    await page.locator('#business-name').fill('My Test Business');
    await page.getByTestId('next-step-btn').nth(3).click(); // to step-assistant

    await expect(page.locator('#step-assistant')).toBeVisible();
    const assistantStep = page.locator('#step-assistant');
    await assistantStep.getByTestId('prev-step-btn').click();

    await expect(page.locator('#step-name')).toBeVisible();
    await expect(page.locator('#step-assistant')).toBeHidden();
  });

  test('Back button in name step navigates to categories step', async ({ page }) => {
    await page.getByTestId('next-step-btn').first().click(); // to step-context
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click(); // to step-categories
    await page.locator('#business-categories').selectOption('Other');
    await page.getByTestId('next-step-btn').nth(2).click(); // to step-name

    await expect(page.locator('#step-name')).toBeVisible();
    const nameStep = page.locator('#step-name');
    await nameStep.getByTestId('prev-step-btn').click();

    await expect(page.locator('#step-categories')).toBeVisible();
    await expect(page.locator('#step-name')).toBeHidden();
  });

  test('Back button in categories step navigates to context step', async ({ page }) => {
    await page.getByTestId('next-step-btn').first().click(); // to step-context
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click(); // to step-categories

    await expect(page.locator('#step-categories')).toBeVisible();
    const categoriesStep = page.locator('#step-categories');
    await categoriesStep.getByTestId('prev-step-btn').click();

    await expect(page.locator('#step-context')).toBeVisible();
    await expect(page.locator('#step-categories')).toBeHidden();
  });

  test('Back button in domain step navigates to target audience step', async ({ page }) => {
    await page.getByTestId('next-step-btn').first().click(); // to step-context
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click(); // to step-categories
    await page.locator('#business-categories').selectOption('Other');
    await page.getByTestId('next-step-btn').nth(2).click(); // to step-name
    await page.locator('#business-name').fill('My Test Business');
    await page.getByTestId('next-step-btn').nth(3).click(); // to step-assistant
    await page.getByTestId('team-support').click();
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByTestId('next-step-btn').nth(4).click(); // to step-admin
    await page.locator('#admin-name').fill('Test User');
    await page.locator('#admin-email').fill('test@test.com');
    await page.locator('#admin-password').fill('password123');
    await page.getByTestId('next-step-btn').nth(5).click(); // to step-offer
    await page.locator('#first-offer').fill('Awesome stuff');
    await page.getByTestId('next-step-btn').nth(6).click(); // to step-location
    await page.locator('#location-input').fill('Local');
    await page.getByTestId('next-step-btn').nth(7).click(); // to step-target-audience

    // Fill target audience and go to domain
    await expect(page.locator('body')).toContainText('Who is your target audience?');
    await page.locator('#target-audience').fill('Everyone');
    await page.getByTestId('next-step-btn').nth(8).click();

    // Domain step
    await expect(page.locator('body')).toContainText('Where will your business live?');
    const domainStep = page.locator('#step-domain');
    await domainStep.getByTestId('prev-step-btn').click();

    // Assert that we are back at Target Audience step, NOT offer step
    await expect(page.locator('body')).toContainText('Who is your target audience?');
    await expect(page.locator('#step-target-audience')).toBeVisible();
    await expect(page.locator('#step-domain')).toBeHidden();
    await expect(page.locator('#step-offer')).toBeHidden();
  });

  test('Setup UI should apply macOS translucent glass standards to offer input', async ({ page }) => {
    // Navigate to step offer
    await page.getByTestId('next-step-btn').first().click();
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click();
    await page.locator('#business-categories').selectOption('Other');
    await page.getByTestId('next-step-btn').nth(2).click();
    await page.locator('#business-name').fill('My Test Business');
    await page.getByTestId('next-step-btn').nth(3).click();
    await page.getByTestId('team-support').click();
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByTestId('next-step-btn').nth(4).click();
    await page.locator('#admin-name').fill('Test User');
    await page.locator('#admin-email').fill('test@test.com');
    await page.locator('#admin-password').fill('password123');
    await page.getByTestId('next-step-btn').nth(5).click();

    const offerInput = page.locator('#first-offer');
    await expect(offerInput).toHaveClass(/glass-control/);
    await expect(offerInput).toHaveClass(/glassmorphism/);
  });

  test('Setup UI should apply macOS translucent glass standards to location input', async ({ page }) => {
    const locationInput = page.locator('#location-input');
    await expect(locationInput).toHaveClass(/glass-control/);
    await expect(locationInput).toHaveClass(/glassmorphism/);
  });

  test('Setup UI should apply macOS translucent glass standards to target audience input', async ({ page }) => {
    const audienceInput = page.locator('#target-audience');
    await expect(audienceInput).toHaveClass(/glass-control/);
    await expect(audienceInput).toHaveClass(/glassmorphism/);
  });

  test('Setup UI navigation flows linearly from start to template selection without errors', async ({ page }) => {
    await page.getByTestId('next-step-btn').first().click();
    await page.getByTestId('context-storefront').click();
    await page.getByTestId('next-step-btn').nth(1).click();
    await page.locator('#business-categories').selectOption('Other');
    await page.getByTestId('next-step-btn').nth(2).click();
    await page.locator('#business-name').fill('My Test Business');
    await page.getByTestId('next-step-btn').nth(3).click();
    await page.getByTestId('team-support').click();
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByTestId('next-step-btn').nth(4).click();
    await page.locator('#admin-name').fill('Test User');
    await page.locator('#admin-email').fill('test@test.com');
    await page.locator('#admin-password').fill('password123');
    await page.getByTestId('next-step-btn').nth(5).click();
    await page.locator('#first-offer').fill('Awesome stuff');
    await page.getByTestId('next-step-btn').nth(6).click();
    await page.locator('#location-input').fill('Local');
    await page.getByTestId('next-step-btn').nth(7).click();
    await page.locator('#target-audience').fill('Everyone');
    await page.getByTestId('next-step-btn').nth(8).click();
    await page.locator('#domain-name').fill('my-store');
    await page.getByTestId('next-step-btn').nth(9).click();

    await expect(page.locator('body')).toContainText('Template Selection');
    await expect(page.locator('#step-template')).toBeVisible();
  });
});
