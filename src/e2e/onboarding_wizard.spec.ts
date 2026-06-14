import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard', () => {
  test.beforeEach(async ({ page }) => {
    // In a real E2E we would start from home, but here we go directly to setup.html as it's the target
    await page.goto('http://127.0.0.1:18789/api/ui/setup.html');
  });

  test('should complete basic onboarding for Maya (Baker)', async ({ page }) => {
    await expect(page.locator('h1')).toContainText('10-Minute Setup Wizard');
    await page.click('[data-testid="next-step-btn"]');

    // Persona Selection
    await page.click('[data-testid="persona-baker"]');
    await expect(page.locator('[data-testid="persona-baker"]')).toContainText('Applied!');

    // Context Step
    await expect(page.locator('input[name="work_context"][value="Storefront"]')).toBeChecked();
    await page.click('#step-context [data-testid="next-step-btn"]');

    // Categories Step
    await expect(page.locator('#business-categories')).toHaveValue('Bakery');
    await page.click('#step-categories [data-testid="next-step-btn"]');

    // Name Step
    await expect(page.locator('#business-name')).toHaveValue("Maya's Bakery");
    await page.click('#step-name [data-testid="next-step-btn"]');

    // Assistant Step
    await expect(page.locator('#assistant-name')).toHaveValue("Cookie");
    await page.click('#step-assistant [data-testid="next-step-btn"]');

    // Admin Step
    await page.fill('#admin-email', 'maya@test.com');
    await page.fill('#admin-password', 'Password123');
    await page.click('#step-admin [data-testid="next-step-btn"]');

    // Offer Step
    await expect(page.locator('#first-offer')).toHaveValue("Custom Birthday Cake");
    await page.click('#step-offer [data-testid="next-step-btn"]');

    // Template Step
    await page.selectOption('#template-selection', 'Modern');
    // Note: finish-btn logic involves a real API call, so we just verify it exists
    await expect(page.locator('#finish-btn')).toBeVisible();
  });

  test('should complete onboarding for Fatima (Food Cart)', async ({ page }) => {
    await page.click('[data-testid="next-step-btn"]');

    await page.click('[data-testid="persona-foodcart"]');
    await page.click('#step-context [data-testid="next-step-btn"]');
    await page.click('#step-categories [data-testid="next-step-btn"]');

    await expect(page.locator('#business-name')).toHaveValue("Fatima's Flavors");
    await page.click('#step-name [data-testid="next-step-btn"]');

    await expect(page.locator('#assistant-name')).toHaveValue("Spice");
  });

  test('should complete onboarding for Nora (Agency)', async ({ page }) => {
    await page.click('[data-testid="next-step-btn"]');

    await page.click('[data-testid="persona-agency"]');
    await page.click('#step-context [data-testid="next-step-btn"]');

    await expect(page.locator('input[name="work_context"][value="Agency"]')).toBeChecked();
    await page.click('#step-categories [data-testid="next-step-btn"]');

    await expect(page.locator('#business-name')).toHaveValue("Nora Studio");
  });

  test('should save draft and resume', async ({ page }) => {
    await page.click('[data-testid="next-step-btn"]');
    await page.fill('#business-name', 'Draft Business');

    // Go to name step manually or via persona
    await page.click('[data-testid="persona-baker"]');
    await page.click('#step-context [data-testid="next-step-btn"]');
    await page.click('#step-categories [data-testid="next-step-btn"]');

    await page.fill('#business-name', 'Persistent Name');
    await page.click('#step-name [data-testid="save-draft-btn"]');
    await expect(page.locator('#draft-saved-msg')).toBeVisible();

    await page.reload();
    // It should resume at step 2 (index of step-name)
    await expect(page.locator('#business-name')).toHaveValue('Persistent Name');
  });

  test('should show error on invalid admin credentials', async ({ page }) => {
    await page.click('[data-testid="next-step-btn"]');
    await page.click('[data-testid="persona-baker"]');
    await page.click('#step-context [data-testid="next-step-btn"]');
    await page.click('#step-categories [data-testid="next-step-btn"]');
    await page.click('#step-name [data-testid="next-step-btn"]');
    await page.click('#step-assistant [data-testid="next-step-btn"]');

    await page.fill('#admin-email', 'invalid-email');
    await page.fill('#admin-password', 'short');
    await page.click('#step-admin [data-testid="next-step-btn"]');

    await expect(page.locator('#email-error')).toBeVisible();
    await expect(page.locator('#password-error')).toBeVisible();
  });
});
