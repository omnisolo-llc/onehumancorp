import { test, expect } from '@playwright/test';

test.describe('OHC Onboarding & Wizard Swarm', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('http://localhost:8081/');
    });

    test('Complete Business Setup Wizard - Happy Path', async ({ page }) => {
        // Step 1: Welcome
        await expect(page.locator('h1')).toContainText('Your business, live in minutes');
        await page.click('text=Launch My Business →');

        // Step 2: Business Type
        await expect(page.locator('h2')).toContainText('What are you building?');
        await page.click('text=Online Store');
        await page.click('text=Continue');

        // Step 3: Identity
        await expect(page.locator('h2')).toContainText("Let's name your dream");
        await page.fill('#biz-name', "Maya's Magic Cakes");
        // Wait for AI suggestion
        await expect(page.locator('#biz-desc')).not.toBeEmpty();
        await page.click('text=Next Step');

        // Step 4: Selling Categories
        await page.click('text=Products');
        await page.click('text=Digital');
        await page.click('text=Continue');

        // Step 5: Payments
        await page.click('text=Online Payments');
        await page.click('text=Almost There');

        // Step 6: Account
        await page.fill('#admin-name', 'Maya Smith');
        await page.fill('#admin-email', 'maya@cakes.com');
        await page.fill('#admin-pass', 'secure123');
        await page.click('text=Review & Launch');

        // Step 7: Review
        await expect(page.locator('#review-name')).toHaveText("Maya's Magic Cakes");
        await expect(page.locator('#review-selling')).toContainText('physical');
        await expect(page.locator('#review-selling')).toContainText('digital');

        // Mocking the backend provision call by clicking launch
        // In a real E2E we'd wait for the transition to Website Builder
        await page.click('#launch-btn');

        // Phase 2: Website Builder
        await expect(page.locator('h2')).toContainText('Design Your Site');
        await page.click('.template-card >> nth=0');
        await page.click('text=Next: Brand');

        await expect(page.locator('h2')).toContainText('Brand Identity');
        await page.click('text=Go Live Instantly');

        // Success
        await expect(page.locator('h1')).toContainText("You're Live!");
        await page.click('text=Meet My AI Team →');

        // AI Agent Config
        await expect(page.locator('h2')).toContainText('Your AI Team');
        await page.click('text=Finish Setup →');

        // Dashboard
        await expect(page.locator('h1')).toContainText('Dashboard');
        await expect(page.locator('#dash-biz-name')).toHaveText("Maya's Magic Cakes");
    });

    test('Progressive Disclosure - Advanced Mode', async ({ page }) => {
        await page.click('text=Launch My Business →');
        await page.click('text=Online Store');
        await page.click('text=Continue');
        await page.fill('#biz-name', 'Test Biz');
        await page.click('text=Next Step');
        await page.click('text=Products');
        await page.click('text=Continue');
        await page.click('text=Online Payments');
        await page.click('text=Almost There');

        // Account step - check advanced field
        const advancedField = page.locator('.advanced-field');
        await expect(advancedField).not.toBeVisible();

        await page.click('.advanced-toggle');
        await expect(advancedField).toBeVisible();
        await expect(advancedField.locator('input')).toHaveValue(/org\.admin/);
    });

    test('AI Agent Prompt Tuning', async ({ page }) => {
        // Go directly to dashboard state if possible, or run through quickly
        // For simplicity, we'll navigate from the start or assume the page is at dashboard
        // Let's assume we are on the dashboard (manually triggering state for test speed if needed)
        await page.evaluate(() => {
            // @ts-ignore
            goToStep('dashboard');
        });

        await page.click('text=Customer Support');
        await expect(page.locator('#tuning-agent-name')).toContainText('Customer Support');

        // Change tone
        await page.click('text=Professional');

        // Test chat sandbox
        await page.fill('#chat-input', 'Hello');
        await page.click('text=Send');
        await expect(page.locator('#chat-box')).toContainText('Hello');
        // Wait for AI response
        await expect(page.locator('#chat-box')).toContainText('As your Customer Support');

        await page.click('text=Save Changes');
        await expect(page.locator('h1')).toContainText('Dashboard');
    });
});
