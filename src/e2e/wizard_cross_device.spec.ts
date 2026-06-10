import { test, expect } from '@playwright/test';

test.describe('Wizard Cross Device E2E', () => {
  test('Persona: Business Owner can resume setup wizard cross device', async ({ page, browser }) => {
    // 1. Owner starts wizard directly from the current route.
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('website-builder-storage');
      localStorage.removeItem('onboardingState');
    }, 'storefront');
    await page.goto('/src/ui/setup.html');
    await page.waitForLoadState('domcontentloaded');

    // 2. Click a step to advance state
    await expect(page.getByRole('heading', { name: "How do you work?" })).toBeVisible();
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();

    // 3. Move to step 2 and enter business category
    await expect(page.getByRole('heading', { name: "What's your category?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Graphic Design").fill("Home Repair");
    await page.getByRole('button', { name: 'Next' }).click();

    // 4. Fill name
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Bakery").fill("Cross Device Business");
    await page.getByRole('button', { name: 'Next' }).click();

    // We are now on assistant setup

    // Save Draft equivalent
    // Let's pretend the browser is closed or moved device
    // Since we don't have an explicit save button, it saves manually on next in code
    const wizardStateStr = await page.evaluate(() => localStorage.getItem('onboardingState'));

    // 5. Simulate a cross-device session with a new browser context
    const newContext = await browser.newContext();
    const newPage = await newContext.newPage();

    await newPage.addInitScript((stateStr) => {
        if(stateStr) {
            localStorage.setItem('onboardingState', stateStr);
        }
    }, wizardStateStr);

    await newPage.goto('/src/ui/setup.html');

    // 6. Verify the business state was restored (it might not automatically jump steps, but inputs should be populated)
    // Actually the code populates form on load. Let's see if the inputs have values.

    await expect(newPage.locator('input[value="Local Service"]')).toBeChecked();

    await newPage.getByRole('button', { name: 'Next' }).click();
    await expect(newPage.getByPlaceholder("e.g. Graphic Design")).toHaveValue("Home Repair");
    await newPage.getByRole('button', { name: 'Next' }).click();
    await expect(newPage.getByPlaceholder("e.g. Maya's Bakery")).toHaveValue("Cross Device Business");

    await newContext.close();
  });
});
