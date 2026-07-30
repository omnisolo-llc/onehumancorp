import { test, expect } from '@playwright/test';
import { randomUUID } from 'crypto';

test.describe('Omnichannel Inbox', () => {

    // Test context for proper E2E network setup connecting to the live API
    let tenantId = randomUUID();
    let conversationId = randomUUID();

    test.beforeEach(async ({ page }) => {
        // Load UI through actual server route (fallback static if server is doing that)
        // Here we just use the static HTML approach again for simplicity
        const fs = require('fs');
        const path = require('path');
        const html = fs.readFileSync(path.resolve(__dirname, '../../server/ohc/static/inbox.html'), 'utf8');
        await page.setContent(html);

        // Emulate URL params
        await page.evaluate(({tenantId, conversationId}) => {
            const url = new URL(window.location);
            url.searchParams.set('tenant_id', tenantId);
            url.searchParams.set('conversation_id', conversationId);
            window.history.pushState({}, '', url);
        }, {tenantId, conversationId});

        await page.setViewportSize({ width: 375, height: 667 });
    });

    test('should display the header', async ({ page }) => {
        const header = page.locator('header');
        await expect(header).toHaveText('Unified Inbox');
    });

    test('should have a message input and send button', async ({ page }) => {
        const input = page.locator('#message-input');
        await expect(input).toBeVisible();
        await expect(input).toHaveAttribute('placeholder', 'Message...');

        const button = page.locator('#send-btn');
        await expect(button).toBeVisible();
        await expect(button).toHaveText('Send');
    });

    test('should append an outgoing message when sending', async ({ page }) => {
        // Intercept API call for loading messages
        await page.route('**/messages', route => {
            route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify([])
            });
        });
        await page.evaluate(() => window.loadMessages && window.loadMessages());

        // Intercept API call to prevent actual network request during UI test
        await page.route('**/messages', route => {
            route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify({ content: 'Test message', message_type: 'outgoing' })
            });
        });

        const input = page.locator('#message-input');
        await input.fill('Test message');

        const button = page.locator('#send-btn');
        await button.click();

        const messages = page.locator('.message.outgoing');
        await expect(messages.last()).toContainText('Test message');
        await expect(messages.last()).toContainText('You');
    });

    test('should append an incoming message on load', async ({ page }) => {
        // Intercept API call for loading messages
        await page.route('**/messages', route => {
            route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify([{ content: 'Incoming test', message_type: 'incoming' }])
            });
        });

        // Trigger load
        await page.evaluate(() => window.loadMessages && window.loadMessages());

        const messages = page.locator('.message.incoming');
        await expect(messages.first()).toContainText('Incoming test');
        await expect(messages.first()).toContainText('WhatsApp');
    });

    test('should send message on Enter key press', async ({ page }) => {
        // Intercept API call for loading messages
        await page.route('**/messages', route => {
            route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify([])
            });
        });
        await page.evaluate(() => window.loadMessages && window.loadMessages());

        await page.route('**/messages', route => {
            route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify({ content: 'Enter key message', message_type: 'outgoing' })
            });
        });

        const input = page.locator('#message-input');
        await input.fill('Enter key message');
        await input.press('Enter');

        const messages = page.locator('.message.outgoing');
        await expect(messages.last()).toContainText('Enter key message');
    });
});
