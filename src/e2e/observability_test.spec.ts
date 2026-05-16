import { test, expect } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';
import * as http from 'http';

let server: http.Server;
let htmlContent = '';

test.beforeAll(async () => {
    // Add fallback for bazel sandbox reading
    const possiblePaths = [
        path.join(__dirname, '../../src/server/lib.rs'), // Local structure
        path.join(process.cwd(), 'src/server/lib.rs'),   // Bazel sandbox sometimes
        '/app/src/server/lib.rs',                        // App root fallback
        '/workspace/src/server/lib.rs'
    ];

    let libRsContent = '';
    for (const p of possiblePaths) {
        if (fs.existsSync(p)) {
            libRsContent = fs.readFileSync(p, 'utf-8');
            break;
        }
    }

    if (!libRsContent) {
        throw new Error("Could not find lib.rs in any expected location");
    }

    const match = libRsContent.match(/r#"([\s\S]*?<!DOCTYPE html>[\s\S]*?<\/html>)[\s\S]*?"#/);
    if (match) {
        htmlContent = match[1] + '</html>';
    } else {
        throw new Error("Could not extract HTML from lib.rs");
    }

    server = http.createServer((req, res) => {
        res.writeHead(200, { 'Content-Type': 'text/html' });
        res.end(htmlContent);
    });

    await new Promise<void>((resolve) => {
        server.listen(3333, () => resolve());
    });
});

test.afterAll(async () => {
    if (server) {
        await new Promise<void>((resolve) => {
            server.close(() => resolve());
        });
    }
});

test.describe('Swarm Observability Panel', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('http://localhost:3333/login');
    });

    test('simulate order adds an activity feed item', async ({ page }) => {
        // We use the UI to navigate
        await page.locator('#login-screen input[type="email"]').fill( 'test@example.com');
        await page.locator('#login-screen input[type="password"]').fill( 'password123');
        await page.locator('#login-screen button:has-text("Login")').click();

        // Wait for the Dashboard
        await expect(page.locator('text="Welcome back, Human."')).toBeVisible();
        await expect(page.locator('text="Agent Activity"')).toBeVisible();

        // Initial state
        await expect(page.locator('#agent-activity-feed >> text="No recent activity."')).toBeVisible();

        // Simulate order
        await page.click('button:has-text("Simulate Order")');

        // Verify that the empty placeholder is removed
        await expect(page.locator('#agent-activity-feed >> text="No recent activity."')).not.toBeVisible();

        // Verify that a new item is added
        const feedItems = page.locator('#agent-activity-feed p');
        await expect(feedItems).toHaveCount(1);
    });

    test('simulate order multiple times adds multiple feed items', async ({ page }) => {
        await page.locator('#login-screen input[type="email"]').fill( 'test@example.com');
        await page.locator('#login-screen input[type="password"]').fill( 'password123');
        await page.locator('#login-screen button:has-text("Login")').click();

        await expect(page.locator('text="Welcome back, Human."')).toBeVisible();

        // Click 3 times
        for (let i = 0; i < 3; i++) {
            await page.click('button:has-text("Simulate Order")');
        }

        // Verify that 3 items are added
        const feedItems = page.locator('#agent-activity-feed p');
        await expect(feedItems).toHaveCount(3);
    });

    test('feed items contain valid plain-language activity strings', async ({ page }) => {
        await page.locator('#login-screen input[type="email"]').fill( 'test@example.com');
        await page.locator('#login-screen input[type="password"]').fill( 'password123');
        await page.locator('#login-screen button:has-text("Login")').click();

        await expect(page.locator('text="Welcome back, Human."')).toBeVisible();

        await page.click('button:has-text("Simulate Order")');

        const feedText = await page.locator('#agent-activity-feed p').first().textContent() || '';

        const validMessages = [
            "✅ Your Support Agent replied to 3 customers",
            "📦 Order Manager updated stock for 12 items",
            "🚀 Marketing Pro launched a new campaign",
            "💬 Sales Bot answered 5 inquiries"
        ];

        // Handle utf-8 emoji discrepancies between node process and browser
        const matched = validMessages.some(msg => {
            const trimmedFeedText = feedText.replace(/^\S+\s+/, '').trim();
            const trimmedMsg = msg.replace(/^\S+\s+/, '').trim();
            return trimmedMsg === trimmedFeedText;
        });
        expect(matched).toBe(true);

    });

    test('feed items have premium glassmorphism styling applied', async ({ page }) => {
        await page.locator('#login-screen input[type="email"]').fill( 'test@example.com');
        await page.locator('#login-screen input[type="password"]').fill( 'password123');
        await page.locator('#login-screen button:has-text("Login")').click();

        await expect(page.locator('text="Welcome back, Human."')).toBeVisible();

        await page.click('button:has-text("Simulate Order")');

        const firstItem = page.locator('#agent-activity-feed p').first();
        await expect(firstItem).toBeVisible();

        // Check styles
        const opacity = await firstItem.evaluate(el => el.style.opacity);
        expect(opacity).toBe('1'); // Should be 1 after animation

        const backdropFilter = await firstItem.evaluate(el => el.style.backdropFilter);
        expect(backdropFilter).toBe('blur(20px) saturate(200%)');
    });

    test('optimistic UI update maintains state activity correctly', async ({ page }) => {
        await page.locator('#login-screen input[type="email"]').fill( 'test@example.com');
        await page.locator('#login-screen input[type="password"]').fill( 'password123');
        await page.locator('#login-screen button:has-text("Login")').click();

        await expect(page.locator('text="Welcome back, Human."')).toBeVisible();

        await page.click('button:has-text("Simulate Order")');
        await page.click('button:has-text("Simulate Order")');

        // Evaluate the global state object in the page
        const stateActivityLength = await page.evaluate(() => {
            return (window as any).state ? (window as any).state.activity.length : 0;
        });

        expect(stateActivityLength).toBe(2);
    });
});
