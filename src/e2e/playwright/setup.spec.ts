import { test, expect } from '@playwright/test';

test.describe('Onboarding Flow', () => {
  test('should allow user to complete onboarding', async ({ page }) => {
    // 1. Visit setup page
    await page.goto('/setup.html');

    // 2. Select work context
    const contextLabel = page.locator('label.radio-option').filter({ hasText: 'Local Service' });
    await contextLabel.click();
    await page.locator('#step-context button.next-step-btn').filter({ hasText: 'Next' }).click();

    // 3. Enter category
    await page.locator('select#business-categories').selectOption({ label: 'Cleaning' });
    await page.locator('#step-categories button.next-step-btn').filter({ hasText: 'Next' }).click();

    // 4. Enter business name
    await page.locator('input#business-name').fill('Test Bakery');
    await page.locator('#step-name button.next-step-btn').filter({ hasText: 'Next' }).click();

    // 5. Enter assistant details
    await page.locator('input#assistant-name').fill('Test Assistant');
    await page.locator('select#assistant-tone').selectOption({ label: 'Professional' });
    await page.locator('#step-assistant button.next-step-btn').filter({ hasText: 'Next' }).click();

    // 6. Enter first offer
    await page.locator('input#first-offer').fill('Test Offer');
    await page.locator('#step-offer button.next-step-btn').filter({ hasText: 'Next' }).click();

    // 7. Select template
    await page.locator('select#template-selection').selectOption({ label: 'Modern' });

    // 8. Submit
    await page.locator('button#finish-btn').click();

    // Wait for success page or some success indication
    await expect(page).toHaveURL(/success\.html/);
    await expect(page.locator('#success-msg')).toContainText('Workspace created for Test Bakery');
  });

  test('should allow user to go back to previous steps', async ({ page }) => {
    await page.goto('/setup.html');

    // Navigate to step 2
    await page.locator('label.radio-option').filter({ hasText: 'Local Service' }).click();
    await page.locator('#step-context button.next-step-btn').filter({ hasText: 'Next' }).click();
    await expect(page.locator('#step-categories')).toBeVisible();

    // Go back to step 1
    await page.locator('#step-categories button.prev-step-btn').click();
    await expect(page.locator('#step-context')).toBeVisible();
  });

  test('should validate required fields before proceeding', async ({ page }) => {
    await page.goto('/setup.html');

    // Step 1 validation
    await page.locator('#step-context button.next-step-btn').filter({ hasText: 'Next' }).click();
    await expect(page.locator('#context-error')).toBeVisible();

    // Proceed to Step 2
    await page.locator('label.radio-option').filter({ hasText: 'Local Service' }).click();
    await page.locator('#step-context button.next-step-btn').filter({ hasText: 'Next' }).click();

    // Step 2 validation
    await page.locator('#step-categories button.next-step-btn').filter({ hasText: 'Next' }).click();
    await expect(page.locator('#categories-error')).toBeVisible();
  });

  test('should allow saving draft state', async ({ page }) => {
    await page.goto('/setup.html');

    await page.locator('label.radio-option').filter({ hasText: 'Local Service' }).click();
    await page.locator('#step-context button.save-draft-btn').click();

    await expect(page.locator('#draft-saved-msg')).toBeVisible();
  });

  test('should show correct categories based on context', async ({ page }) => {
    await page.goto('/setup.html');

    await page.locator('label.radio-option').filter({ hasText: 'Storefront' }).click();
    await page.locator('#step-context button.next-step-btn').filter({ hasText: 'Next' }).click();

    const select = page.locator('select#business-categories');
    const options = await select.locator('option').allInnerTexts();
    expect(options).toContain('Cafe');
    expect(options).toContain('Restaurant');
  });
});
