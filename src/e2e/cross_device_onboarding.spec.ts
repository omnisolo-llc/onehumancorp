import { test, expect } from '@playwright/test';

test.describe('Cross Device Onboarding CUJ', () => {
  test('Persona: Business Owner can save draft and resume cross device', async ({ page, browser }) => {
    let serverState = {};
    const fs = require('fs');
    const path = require('path');

    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR || path.resolve(__dirname, '..', '..'), process.env.TEST_WORKSPACE)
        : path.resolve(__dirname, '..', '..');

    await page.route('**/setup.html', async route => {
        const fileContent = fs.readFileSync(path.join(workspaceRoot, 'src/ui/tauri/src/ui/setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: fileContent });
    });

    await page.route('**/api/onboarding/draft', async route => {
        if (route.request().method() === 'POST') {
            serverState = JSON.parse(route.request().postData());
            await route.fulfill({ status: 200 });
        } else {
            await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify(serverState) });
        }
    });

    await page.route('**/api/onboarding/state', async route => {
        if (route.request().method() === 'POST') {
            serverState = JSON.parse(route.request().postData());
            await route.fulfill({ status: 200 });
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

    const chatButton = page.locator('button', { hasText: 'Conversational Setup' });
    if(await chatButton.isVisible()) {
        await page.evaluate(() => { (window as any).goToStep('step-context') });
    }

    await expect(page.getByText('How do you work?')).toBeVisible();
    await page.getByText("I'm a Baker").click();
    await page.locator('#step-context .next-step-btn').click();

    // We need to re-select because categories populate on step show
    await page.evaluate(() => {
        const select = document.querySelector('#business-categories') as HTMLSelectElement;
        if(select) {
            const opt = document.createElement('option');
            opt.value = 'Bakery';
            opt.textContent = 'Bakery';
            select.appendChild(opt);
            select.value = 'Bakery';
        }
    });

    await page.locator('#step-categories').getByRole('button', { name: 'Next' }).click();

    const nameInput = page.locator('#business-name');
    await nameInput.fill('Cross Device Bakery');

    const saveDraftBtn = page.getByRole('button', { name: /Save Draft/i }).first();
    await saveDraftBtn.click();
    await expect(page.getByText('Draft Saved!')).toBeVisible();

    await page.waitForTimeout(1500);

    const newContext = await browser.newContext();
    const newPage = await newContext.newPage();

    await newPage.route('**/setup.html', async route => {
        const fileContent = fs.readFileSync(path.join(workspaceRoot, 'src/ui/tauri/src/ui/setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: fileContent });
    });
    await newPage.route('**/api/onboarding/draft', async route => {
        if (route.request().method() === 'POST') {
            serverState = JSON.parse(route.request().postData());
            await route.fulfill({ status: 200 });
        } else {
            await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify(serverState) });
        }
    });

    await newPage.route('**/api/onboarding/state', async route => {
        if (route.request().method() === 'POST') {
            serverState = JSON.parse(route.request().postData());
            await route.fulfill({ status: 200 });
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
    await newPage.waitForTimeout(2000);

    // We should be able to see the business name in the DOM
    await expect(newPage.locator('#business-name')).toHaveValue('Cross Device Bakery', { timeout: 10000 });

    await newContext.close();
  });
});
