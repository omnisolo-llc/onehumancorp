import { test, expect } from '@playwright/test';

test.describe('Cross Device Onboarding CUJ', () => {
  test('Persona: Business Owner can save draft and resume cross device', async ({ page, browser }) => {
    let serverState = {};
    const fs = require('fs');
    const path = require('path');

    await page.route('**/setup.html', async route => {
        const fileContent = fs.readFileSync(path.join(process.cwd(), 'src/ui/tauri/src/ui/setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: fileContent });
    });
    await page.route('**/api/onboarding/state', async route => {
        if (route.request().method() === 'POST') {
            const body = JSON.parse(route.request().postData());
            serverState = body.wizardState;
            await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ success: true }) });
        } else {
            await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify(serverState) });
        }
    });

    await page.goto('http://mock/setup.html');
    await page.evaluate(() => {
      localStorage.setItem('tenant_id', 'storefront');
      localStorage.setItem('user_id', 'test-user');
    });
    await page.reload();

    await expect(page.getByText('How do you work?')).toBeVisible();
    await page.getByText("I'm a Baker").click();
    await page.getByText('Next').first().click();
    await page.locator('#business-categories').selectOption('Home Baker');
    await page.getByRole('button', { name: 'Next' }).click();

    const nameInput = page.locator('#business-name');
    await nameInput.fill('Cross Device Bakery');

    const saveDraftBtn = page.getByRole('button', { name: /Save Draft/i }).first();
    await saveDraftBtn.click();
    await expect(page.getByText('Draft Saved!')).toBeVisible();

    await page.waitForTimeout(500);

    const newContext = await browser.newContext();
    const newPage = await newContext.newPage();

    await newPage.route('**/setup.html', async route => {
        const fileContent = fs.readFileSync(path.join(process.cwd(), 'src/ui/tauri/src/ui/setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: fileContent });
    });
    await newPage.route('**/api/onboarding/state', async route => {
        if (route.request().method() === 'POST') {
            const body = JSON.parse(route.request().postData());
            serverState = body.wizardState;
            await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ success: true }) });
        } else {
            await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify(serverState) });
        }
    });

    await newPage.goto('http://mock/setup.html');
    await newPage.evaluate(() => {
      localStorage.setItem('tenant_id', 'storefront');
      localStorage.setItem('user_id', 'test-user');
    });
    await newPage.reload();

    // Since the API fetches asynchronously on load, wait a bit
    await newPage.waitForTimeout(1000);

    // We should be able to see the business name in the DOM
    await expect(newPage.locator('#business-name')).toHaveValue('Cross Device Bakery', { timeout: 10000 });

    await newContext.close();
  });
});
