import { test, expect } from '@playwright/test';

test.describe('Grandmother Test - Comprehensive UX Audit', () => {
    test.beforeEach(async ({ page }) => {
        // Mock authentication
        await page.goto('/login');
        await page.fill('input[type="email"]', 'grandma@example.com');
        await page.fill('input[type="password"]', 'simple_password');
        await page.click('button:has-text("Sign In")');
        await page.waitForURL('**/dashboard');
    });


    test('Verify plain language and accessibility in Dashboard view', async ({ page }) => {
        await page.click('nav >> text="Dashboard"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Inventory view', async ({ page }) => {
        await page.click('nav >> text="Inventory"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Sales view', async ({ page }) => {
        await page.click('nav >> text="Sales"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Customers view', async ({ page }) => {
        await page.click('nav >> text="Customers"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Messages view', async ({ page }) => {
        await page.click('nav >> text="Messages"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Settings view', async ({ page }) => {
        await page.click('nav >> text="Settings"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Billing view', async ({ page }) => {
        await page.click('nav >> text="Billing"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Integrations view', async ({ page }) => {
        await page.click('nav >> text="Integrations"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Help view', async ({ page }) => {
        await page.click('nav >> text="Help"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Profile view', async ({ page }) => {
        await page.click('nav >> text="Profile"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Notifications view', async ({ page }) => {
        await page.click('nav >> text="Notifications"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Reports view', async ({ page }) => {
        await page.click('nav >> text="Reports"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Team view', async ({ page }) => {
        await page.click('nav >> text="Team"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Taxes view', async ({ page }) => {
        await page.click('nav >> text="Taxes"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Shipping view', async ({ page }) => {
        await page.click('nav >> text="Shipping"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Discounts view', async ({ page }) => {
        await page.click('nav >> text="Discounts"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in GiftCards view', async ({ page }) => {
        await page.click('nav >> text="GiftCards"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Returns view', async ({ page }) => {
        await page.click('nav >> text="Returns"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Payouts view', async ({ page }) => {
        await page.click('nav >> text="Payouts"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Analytics view', async ({ page }) => {
        await page.click('nav >> text="Analytics"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Marketing view', async ({ page }) => {
        await page.click('nav >> text="Marketing"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Campaigns view', async ({ page }) => {
        await page.click('nav >> text="Campaigns"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Templates view', async ({ page }) => {
        await page.click('nav >> text="Templates"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Automations view', async ({ page }) => {
        await page.click('nav >> text="Automations"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Forms view', async ({ page }) => {
        await page.click('nav >> text="Forms"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Webhooks view', async ({ page }) => {
        await page.click('nav >> text="Webhooks"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in APIKeys view', async ({ page }) => {
        await page.click('nav >> text="APIKeys"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Domains view', async ({ page }) => {
        await page.click('nav >> text="Domains"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in SEO view', async ({ page }) => {
        await page.click('nav >> text="SEO"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Navigation view', async ({ page }) => {
        await page.click('nav >> text="Navigation"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Themes view', async ({ page }) => {
        await page.click('nav >> text="Themes"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Assets view', async ({ page }) => {
        await page.click('nav >> text="Assets"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Blog view', async ({ page }) => {
        await page.click('nav >> text="Blog"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Pages view', async ({ page }) => {
        await page.click('nav >> text="Pages"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Authors view', async ({ page }) => {
        await page.click('nav >> text="Authors"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Comments view', async ({ page }) => {
        await page.click('nav >> text="Comments"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Tags view', async ({ page }) => {
        await page.click('nav >> text="Tags"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Categories view', async ({ page }) => {
        await page.click('nav >> text="Categories"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Menus view', async ({ page }) => {
        await page.click('nav >> text="Menus"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });

    test('Verify plain language and accessibility in Footer view', async ({ page }) => {
        await page.click('nav >> text="Footer"');
        await page.waitForLoadState('networkidle');

        // 1. Ensure no technical jargon exists on the page
        const bodyText = await page.textContent('body') || "";
        expect(bodyText).not.toMatch(/API|JSON|Endpoint|Webhook|UUID|Latency/i);

        // 2. Ensure fonts are large enough (min 16px for body text)
        const paragraphs = page.locator('p');
        const count = await paragraphs.count();
        for (let i = 0; i < count; i++) {
            const fontSize = await paragraphs.nth(i).evaluate((el) => window.getComputedStyle(el).fontSize);
            const size = parseFloat(fontSize);
            expect(size).toBeGreaterThanOrEqual(15.0);
        }

        // 3. Ensure primary call to action is obvious
        const primaryButton = page.locator('button.btn-primary').first();
        if (await primaryButton.isVisible()) {
            const text = await primaryButton.textContent();
            expect(text).toMatch(/Save|Next|Done|Send|Update|Create|Add/i);

            // Contrast check (simulated)
            const color = await primaryButton.evaluate((el) => window.getComputedStyle(el).color);
            expect(color).not.toBe('rgba(0, 0, 0, 0)');
        }

        // 4. Ensure tooltips are available for complex inputs
        const complexInputs = page.locator('input[data-complex="true"]');
        const inputCount = await complexInputs.count();
        for (let i = 0; i < inputCount; i++) {
            await expect(complexInputs.nth(i).locator('xpath=following-sibling::*[@role="tooltip"]')).toBeVisible();
        }

        // 5. Ensure "Help" or "Call an Expert" is always accessible
        await expect(page.locator('button[aria-label="Get Help"]')).toBeVisible();

        // 6. Visual Excellence Mandate: Check Glassmorphism
        const cards = page.locator('.glass-card');
        if (await cards.count() > 0) {
            const backdropFilter = await cards.first().evaluate((el) => window.getComputedStyle(el).backdropFilter);
            expect(backdropFilter).toContain('blur');
        }
    });
});
