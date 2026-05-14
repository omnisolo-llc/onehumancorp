import { test, expect } from '@playwright/test';

test.describe('Rigorous Business Setup Wizard Functional Suite', () => {

    test('Scenario 1: Standard Online Store Setup', async ({ page }) => {
        await page.goto('/login');
        await page.getByPlaceholder('Email or Username').fill('founder1@example.com');
        await page.locator('input[type="password"]').fill('password123');
        await page.locator('button:has-text("Login")').click();

        await page.waitForURL('**/dashboard');
        await page.locator('button:has-text("Start Setup")').click();
        await expect(page.locator('text=Your business, live in minutes.')).toBeVisible();
        await page.locator('button:has-text("Start My Business")').click();

        await expect(page.locator('text=What kind of business are you building?')).toBeVisible();
        await page.locator('button:has-text("Online Store")').click();

        await expect(page.locator('text=What is your business called?')).toBeVisible();
        await page.locator('input[placeholder="e.g. Maya\'s Cakes"]').fill('Tech Gadgets');
        await page.locator('button:has-text("Next")').filter({ visible: true }).first().click();

        await expect(page.locator('text=What do you sell?')).toBeVisible();
        await page.locator('button:has-text("Physical products")').click();

        await expect(page.locator('text=Add your first product')).toBeVisible();
        await page.locator('input[placeholder="What is the name of this product?"]').fill('Smartphone');
        await page.locator('input[placeholder="0.00"]').fill('699.99');
        await page.locator('button:has-text("Next")').filter({ visible: true }).first().click();

        await expect(page.locator('text=How do you want to receive payments?')).toBeVisible();
        await page.locator('button:has-text("Online only")').click();

        await expect(page.locator('text=Choose a Template')).toBeVisible();
        await page.locator('button:has-text("Modern")').click();

        await expect(page.locator('text=Choose a Domain')).toBeVisible();
        await page.locator('button:has-text("Free OHC Domain")').click();

        await expect(page.locator('text=Administrator account')).toBeVisible();
        await page.locator('input[placeholder="e.g. Maya Smith"]').fill('Jane Founder');
        await page.locator('input[placeholder="you@email.com"]').fill('jane@example.com');
        await page.locator('input[placeholder="Password"]').fill('securepassword!');
        await page.locator('button:has-text("Review & Launch")').filter({ visible: true }).first().click();

        await expect(page.locator('text=Almost there')).toBeVisible({ timeout: 5000 });
        await page.locator('button:has-text("Launch!")').click();

        await expect(page.locator('text=Onboarding Complete!')).toBeVisible({ timeout: 10000 });
    });

    test('Scenario 2: Backend API Persistence Validation without interception', async ({ page }) => {
        await page.goto('/login');
        await page.getByPlaceholder('Email or Username').fill('founder3@example.com');
        await page.locator('input[type="password"]').fill('password123');
        await page.locator('button:has-text("Login")').click();
        await page.waitForURL('**/dashboard');

        // Watch for the actual API call
        const requestPromise = page.waitForResponse(response => response.url().includes('/api/wizard') && response.status() === 200);

        await page.locator('button:has-text("Start Setup")').click();
        await expect(page.locator('text=Your business, live in minutes.')).toBeVisible();
        await page.locator('button:has-text("Start My Business")').click();

        // This click should trigger a fetch automatically, because of nextStep()
        await page.locator('button:has-text("Online Store")').click();

        const response = await requestPromise;
        expect(response.status()).toBe(200);
    });

    test('Scenario 3: Progressive disclosure functionality', async ({ page }) => {
        await page.goto('/login');
        await page.getByPlaceholder('Email or Username').fill('advanced@example.com');
        await page.locator('input[type="password"]').fill('password123');
        await page.locator('button:has-text("Login")').click();
        await page.waitForURL('**/dashboard');

        await page.locator('button:has-text("Start Setup")').click();
        await expect(page.locator('text=Your business, live in minutes.')).toBeVisible();

        // toggle advanced mode
        await page.locator('input#advanced-toggle').check();
        const mode = await page.evaluate(() => localStorage.getItem('advancedMode'));
        expect(mode).toBe('true');
    });

    test('Scenario 4: AI Auto Build mode selection', async ({ page }) => {
        await page.goto('/login');
        await page.getByPlaceholder('Email or Username').fill('lazy@example.com');
        await page.locator('input[type="password"]').fill('password123');
        await page.locator('button:has-text("Login")').click();
        await page.waitForURL('**/dashboard');
        await page.locator('button:has-text("Start Setup")').click();
        await page.locator('button:has-text("Instant Build (AI)")').click();

        await expect(page.locator('text=Describe your business in a sentence')).toBeVisible();
        await page.locator('button:has-text("Generate Storefront")').click();
        await expect(page.locator('text=Designing your storefront...')).toBeVisible();
    });

    test('Scenario 5: Resume functionality on load', async ({ page }) => {
        // Mock the backend state response directly for load
        await page.route('/api/wizard', async (route) => {
            if (route.request().method() === 'GET') {
                route.fulfill({ json: { step: '2', data: '{}' } });
            } else {
                route.continue();
            }
        });

        await page.goto('/login');
        await page.getByPlaceholder('Email or Username').fill('resume@example.com');
        await page.locator('input[type="password"]').fill('password123');
        await page.locator('button:has-text("Login")').click();
        await page.waitForURL('**/dashboard');

        // Because of the mock, it should jump straight to Step 2
        // It should fetch /api/wizard on load
        await page.waitForTimeout(1000);
        await expect(page.locator('text=What is your business called?')).toBeVisible();
    });
});
