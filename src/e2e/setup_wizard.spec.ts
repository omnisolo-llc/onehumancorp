import { test, expect } from './fixtures';

test.describe('Setup Wizard', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/setup.html');
  });

  test('should navigate through the wizard successfully', async ({ page }) => {
    // Step Initial
    const initialStep = page.locator('#step-initial');
    await expect(initialStep).toBeVisible();
    await initialStep.locator('.next-step-btn').click();

    // Step Context
    const contextStep = page.locator('#step-context');
    await expect(contextStep).toBeVisible();
    await contextStep.locator('.context-card').first().click();
    await contextStep.locator('.next-step-btn').click();

    // Step Categories
    const categoriesStep = page.locator('#step-categories');
    await expect(categoriesStep).toBeVisible();
    await categoriesStep.locator('.next-step-btn').click();

    // Step Name
    const nameStep = page.locator('#step-name');
    await expect(nameStep).toBeVisible();
    await nameStep.locator('#business-name').fill('Test Business');
    await nameStep.locator('.next-step-btn').click();

    // Step Assistant
    const assistantStep = page.locator('#step-assistant');
    await expect(assistantStep).toBeVisible();
    await assistantStep.locator('.next-step-btn').click();

    // Step Admin
    const adminStep = page.locator('#step-admin');
    await expect(adminStep).toBeVisible();
    await adminStep.locator('#admin-name').fill('Test Admin');
    await adminStep.locator('#admin-email').fill('testadmin@example.com');
    await adminStep.locator('#admin-password').fill('testpassword123!');
    await adminStep.locator('.next-step-btn').click();

    // Step Offer
    const offerStep = page.locator('#step-offer');
    await expect(offerStep).toBeVisible();
    await offerStep.locator('#first-offer').fill('We offer testing services');
    await offerStep.locator('.next-step-btn').click();

    // Step Location
    const locationStep = page.locator('#step-location');
    await expect(locationStep).toBeVisible();
    await locationStep.locator('#location-input').fill('Test City, ST');
    await locationStep.locator('.next-step-btn').click();

    // Step Target Audience
    const targetAudienceStep = page.locator('#step-target-audience');
    await expect(targetAudienceStep).toBeVisible();
    await targetAudienceStep.locator('#target-audience').fill('Test Audience');
    await targetAudienceStep.locator('.next-step-btn').click();

    // Step Domain
    const domainStep = page.locator('#step-domain');
    await expect(domainStep).toBeVisible();
    await domainStep.locator('#domain-name').fill('test-business');
    await domainStep.locator('.next-step-btn').click();

    // Step Template
    const templateStep = page.locator('#step-template');
    await expect(templateStep).toBeVisible();
    await templateStep.locator('#template-selection').selectOption('Modern');

    const finishBtn = templateStep.locator('#finish-btn');
    await expect(finishBtn).toBeVisible();
    await expect(finishBtn).toBeEnabled();

    // Verify successful launch fallback (since TAURI is undefined, it stays on template step)
    await finishBtn.click();
    await expect(templateStep).toBeVisible();
  });

  test('should show validation error when business name is empty', async ({ page }) => {
    // Initial
    await page.locator('#step-initial .next-step-btn').click();

    // Context
    await page.locator('#step-context .context-card').first().click();
    await page.locator('#step-context .next-step-btn').click();

    // Categories
    await page.locator('#step-categories .next-step-btn').click();

    // Name - intentionally skip filling
    const nameStep = page.locator('#step-name');
    await expect(nameStep).toBeVisible();
    await nameStep.locator('.next-step-btn').click();

    const nameError = nameStep.locator('#name-error');
    await expect(nameError).toBeVisible();
  });

  test('should persist state when navigating back', async ({ page }) => {
    // Initial
    await page.locator('#step-initial .next-step-btn').click();

    // Context
    await page.locator('#step-context .context-card').first().click();
    await page.locator('#step-context .next-step-btn').click();

    // Categories
    await page.locator('#step-categories .next-step-btn').click();

    // Name
    const nameStep = page.locator('#step-name');
    await nameStep.locator('#business-name').fill('Persistent Business');
    await nameStep.locator('.next-step-btn').click();

    // Assistant (Next step)
    const assistantStep = page.locator('#step-assistant');
    await expect(assistantStep).toBeVisible();

    // Click back
    await assistantStep.locator('.prev-step-btn').click();

    // Ensure state is persisted
    await expect(nameStep).toBeVisible();
    await expect(nameStep.locator('#business-name')).toHaveValue('Persistent Business');
  });

  test('should toggle AI capabilities in assistant step', async ({ page }) => {
    // Initial -> Context -> Categories -> Name -> Assistant
    await page.locator('#step-initial .next-step-btn').click();
    await page.locator('#step-context .context-card').first().click();
    await page.locator('#step-context .next-step-btn').click();
    await page.locator('#step-categories .next-step-btn').click();
    await page.locator('#step-name #business-name').fill('Test Business');
    await page.locator('#step-name .next-step-btn').click();

    const assistantStep = page.locator('#step-assistant');
    await expect(assistantStep).toBeVisible();

    // Toggle the 'Draft Replies' toggle. In HTML, it's a checkbox inside a .switch
    // The toggle row wraps it, we can click the label
    const draftToggleRow = assistantStep.locator('.toggle-row').first();
    const draftCheckbox = draftToggleRow.locator('input[type="checkbox"]');

    const isChecked = await draftCheckbox.isChecked();
    await draftToggleRow.click(); // Click the toggle label

    // Should be inverted now
    expect(await draftCheckbox.isChecked()).not.toBe(isChecked);
  });

  test('should allow selecting different persona chips', async ({ page }) => {
    // Initial
    await page.locator('#step-initial .next-step-btn').click();

    // Context
    const contextStep = page.locator('#step-context');
    await expect(contextStep).toBeVisible();

    const secondPersonaChip = contextStep.locator('.persona-chip').nth(1);
    await secondPersonaChip.click();

    // Should have selected class
    await expect(secondPersonaChip).toHaveClass(/selected/);
  });
});
