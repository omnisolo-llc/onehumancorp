import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the real server route
    await page.goto('/ui/setup.html');
  });

  test('Initial load shows the correct header', async ({ page }) => {
    await expect(page.locator('h1').first()).toBeVisible();
    await expect(page.locator('#instant-bio')).toBeVisible();
    await expect(page.locator('#generate-storefront-btn')).toBeVisible();
  });

  test('Conversational setup button proceeds to chat', async ({ page }) => {
    const chatBtn = page.locator('button', { hasText: 'Conversational Setup' }).first();
    await expect(chatBtn).toBeVisible();
    await chatBtn.click();

    // Using standard assertions instead of evaluating javascript
    const chatStep = page.locator('#step-chat');
    await expect(chatStep).toBeVisible({ timeout: 10000 });
  });

  test('Manual Next step proceeds to manual setup', async ({ page }) => {
    const nextBtn = page.locator('button[data-next="step-categories"]').first();
    await expect(nextBtn).toBeVisible();
    await nextBtn.click();

    const categoriesStep = page.locator('#step-categories');
    await expect(categoriesStep).toBeVisible({ timeout: 10000 });
  });

  test('Business setup fields accept input', async ({ page }) => {
    const nextBtn1 = page.locator('button[data-next="step-categories"]').first();
    await expect(nextBtn1).toBeVisible();
    await nextBtn1.click();

    const categoriesStep = page.locator('#step-categories');
    await expect(categoriesStep).toBeVisible({ timeout: 10000 });

    const categoriesSelect = page.locator('#business-categories');
    await expect(categoriesSelect).toBeVisible();
    await categoriesSelect.selectOption({ index: 1 });

    const nextBtn2 = page.locator('button[data-next="step-name"]').first();
    await expect(nextBtn2).toBeVisible();
    await nextBtn2.click();

    const nameStep = page.locator('#step-name');
    await expect(nameStep).toBeVisible({ timeout: 10000 });

    await page.fill('#business-name', 'Maya Custom Cakes');
    await expect(page.locator('#business-name')).toHaveValue('Maya Custom Cakes');
  });

  test('AI capabilities toggles are present', async ({ page }) => {
    const nextBtn1 = page.locator('button[data-next="step-categories"]').first();
    await expect(nextBtn1).toBeVisible();
    await nextBtn1.click();

    const categoriesStep = page.locator('#step-categories');
    await expect(categoriesStep).toBeVisible({ timeout: 10000 });
    const categoriesSelect = page.locator('#business-categories');
    await categoriesSelect.selectOption({ index: 1 });

    const nextBtn2 = page.locator('button[data-next="step-name"]').first();
    await nextBtn2.click();

    const nameStep = page.locator('#step-name');
    await expect(nameStep).toBeVisible({ timeout: 10000 });
    await page.fill('#business-name', 'Maya Custom Cakes');

    const nextBtn3 = page.locator('button[data-next="step-assistant"]').first();
    await nextBtn3.click();

    const assistantStep = page.locator('#step-assistant');
    await expect(assistantStep).toBeVisible({ timeout: 10000 });

    await expect(page.locator('.capability-toggles')).toBeVisible();
    const toggle = page.locator('input[type="checkbox"]').first();
    await expect(toggle).toBeDefined();
  });
});
