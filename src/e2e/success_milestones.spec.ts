import { E2E_ROUTES, UI_LOCATORS } from "./playwright_test_constants";
import { test, expect } from '@playwright/test';

test.describe('Success Milestones Notifications', () => {
  test('should verify milestone functionality when order threshold is reached', async ({ page }) => {
    // 1. Authenticate and navigate to dashboard
    await page.goto(E2E_ROUTES.LOGIN);
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator(UI_LOCATORS.PASSWORD_INPUT).filter({ visible: true }).first().fill('password123');
    await page.locator(UI_LOCATORS.LOGIN_BUTTON).filter({ visible: true }).first().click();
    await page.waitForURL('**/*');

    // 2. Wait for the dashboard to load and show the "Mark Order Ready" button
    // The test mock usually sets new_orders_count = 3
    const markReadyBtn = page.locator(UI_LOCATORS.MARK_ORDER_READY);
    await expect(markReadyBtn).toBeVisible({ timeout: 10000 });

    // 3. Click the button 3 times to trigger the milestone
    for (let i = 0; i < 3; i++) {
        await markReadyBtn.click();
        await page.waitForTimeout(100);
    }

    // 4. Assert the milestone UI appears
    const milestoneTitle = page.locator('text=🎉 3rd Order!');
    await expect(milestoneTitle).toBeVisible({ timeout: 5000 });
    await expect(page.locator('text=You completed 3 orders!')).toBeVisible();

    // 5. Dismiss the milestone
    const dismissBtn = page.locator(UI_LOCATORS.DISMISS_BTN);
    await expect(dismissBtn).toBeVisible();
    await dismissBtn.click();

    // 6. Assert the milestone UI disappears
    await expect(milestoneTitle).toBeHidden();
  });

  test('should verify 1st order milestone', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator(UI_LOCATORS.PASSWORD_INPUT).filter({ visible: true }).first().fill('password123');
    await page.locator(UI_LOCATORS.LOGIN_BUTTON).filter({ visible: true }).first().click();
    await page.waitForURL('**/dashboard');

    const markReadyBtn = page.locator(UI_LOCATORS.MARK_ORDER_READY);
    await expect(markReadyBtn).toBeVisible();
    await markReadyBtn.click();

    await expect(page.locator(UI_LOCATORS.FIRST_SALE)).toBeVisible();
  });

  test('should verify 10th order milestone', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator(UI_LOCATORS.PASSWORD_INPUT).filter({ visible: true }).first().fill('password123');
    await page.locator(UI_LOCATORS.LOGIN_BUTTON).filter({ visible: true }).first().click();
    await page.waitForURL('**/dashboard');

    const markReadyBtn = page.locator(UI_LOCATORS.MARK_ORDER_READY);
    await expect(markReadyBtn).toBeVisible();

    // Using simple mock triggers if the loop isn't sufficient for 10
    // If the loop doesn't generate 10, the mock provides an automated path. We just test the loop
    for (let i = 0; i < 10; i++) {
        await markReadyBtn.click();
        await page.waitForTimeout(50);
    }

    // Might appear later
    await expect(page.locator('text=🎉 10th Order!')).toBeVisible({ timeout: 10000 });
  });

  test('should verify 100 visitors milestone', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator(UI_LOCATORS.PASSWORD_INPUT).filter({ visible: true }).first().fill('password123');
    await page.locator(UI_LOCATORS.LOGIN_BUTTON).filter({ visible: true }).first().click();
    await page.waitForURL('**/dashboard');

    // Our test framework has a single-shot timer for 5s that triggers 100 visitors milestone
    await expect(page.locator('text=🚀 100 Visitors Today!')).toBeVisible({ timeout: 10000 });
  });

  test('should verify milestone dismissal', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator(UI_LOCATORS.PASSWORD_INPUT).filter({ visible: true }).first().fill('password123');
    await page.locator(UI_LOCATORS.LOGIN_BUTTON).filter({ visible: true }).first().click();
    await page.waitForURL('**/dashboard');

    const markReadyBtn = page.locator(UI_LOCATORS.MARK_ORDER_READY);
    await expect(markReadyBtn).toBeVisible();
    await markReadyBtn.click();

    await expect(page.locator(UI_LOCATORS.FIRST_SALE)).toBeVisible();

    const dismissBtn = page.locator(UI_LOCATORS.DISMISS_BTN);
    await expect(dismissBtn).toBeVisible();
    await dismissBtn.click();

    await expect(page.locator(UI_LOCATORS.FIRST_SALE)).toBeHidden();
  });
});
