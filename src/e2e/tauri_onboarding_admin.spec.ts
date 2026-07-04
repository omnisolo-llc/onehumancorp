import { test, expect } from '@playwright/test';

// we just use the raw test instead of the fixture that requires login to /dashboard
test.describe('Tauri Onboarding Admin Setup', () => {
  test('Requires admin name on step-admin', async ({ page }) => {
    // Navigate to the onboarding route directly
    await page.goto(`file://${process.cwd()}/src/ui/tauri/src/ui/setup.html`);

    // We expect the setup to redirect or load the initial step
    await page.waitForSelector('#step-initial .next-step-btn');
    await page.locator('#step-initial [data-next="step-context"]').click();

    // Step Context
    await page.waitForSelector('#step-context:not([style*="display: none"])');
    await page.click('[data-testid="context-local"]');
    await page.click('#step-context .next-step-btn');

    // Step Categories
    await page.waitForSelector('#step-categories:not([style*="display: none"])');

    // evaluate business categories dropdown to have a value, test environment may mock it
    await page.evaluate(() => {
        const sel = document.getElementById('business-categories') as HTMLSelectElement;
        if (sel) {
            const opt = document.createElement('option');
            opt.value = 'Baking';
            opt.text = 'Baking';
            sel.appendChild(opt);
            sel.value = 'Baking';
        }
    });
    // await page.selectOption('#business-categories', 'Baking');
    await page.click('#step-categories .next-step-btn');

    // Step Name
    await page.waitForSelector('#step-name:not([style*="display: none"])');
    await page.fill('#business-name', 'Test Business');
    await page.click('#step-name .next-step-btn');

    // Step Assistant
    await page.waitForSelector('#step-assistant:not([style*="display: none"])');
    await page.getByTestId('team-operations').click();
    await page.evaluate(() => {
        const sel = document.getElementById('assistant-tone') as HTMLSelectElement;
        if (sel) {
            const opt = document.createElement('option');
            opt.value = 'Friendly';
            opt.text = 'Friendly';
            sel.appendChild(opt);
            sel.value = 'Friendly';
        }
    });
    await page.click('#step-assistant .next-step-btn');

    // Step Admin
    await page.waitForSelector('#step-admin:not([style*="display: none"])');

    // Try to proceed without filling out the admin name
    await page.click('#step-admin .next-step-btn');

    // Verify admin-name error appears
    const isErrorVisible = await page.evaluate(() => {
        const err = document.getElementById('admin-name-error');
        return err && window.getComputedStyle(err).display === 'block';
    });
    expect(isErrorVisible).toBe(true);

    // Fill in the admin name
    await page.fill('#admin-name', 'Test Admin');
    await page.click('#step-admin .next-step-btn');

    // Verify error is gone
    const isErrorStillVisible = await page.evaluate(() => {
        const err = document.getElementById('admin-name-error');
        return err && window.getComputedStyle(err).display === 'block';
    });
    expect(isErrorStillVisible).toBe(false);
  });
});
