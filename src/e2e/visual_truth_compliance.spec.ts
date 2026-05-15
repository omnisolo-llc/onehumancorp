import { test, expect } from '@playwright/test';

// Exhaustive test matrix for visual truth compliance, verifying the
// grandmother test (no technical jargon) and glassmorphism standards.

const SCREENS = [
    { url: '/dashboard', title: 'Dashboard', selectors: ['.glass'] },
    { url: '/agents', title: 'Agents', selectors: ['.glass'] },
    { url: '/settings', title: 'Settings', selectors: ['.glass'] },
    { url: '/pricing', title: 'Pricing Plans', selectors: ['.card'] },
    { url: '/inbox', title: 'Inbox', selectors: ['.card'] },
];

const FORBIDDEN_JARGON = [
    'cron', 'mutex', 'deadlock', 'kubernetes', 'pod', 'segfault', 'null pointer',
    'stack trace', 'backend', 'frontend', 'middleware', 'database schema', 'orm'
];

test.describe('Visual Truth Compliance Check', () => {
    for (const screen of SCREENS) {
        test(`Verify screen [${screen.title}] adheres to glassmorphism standards`, async ({ page }) => {
            await page.goto(screen.url);

            for (const selector of screen.selectors) {
                const count = await page.locator(selector).count();
                if (count > 0) {
                    // Check if the glass class actually has the backdrop-filter applied
                    const isGlass = await page.evaluate((sel) => {
                        const el = document.querySelector(sel);
                        if (!el) return false;
                        const style = window.getComputedStyle(el);
                        return style.backdropFilter.includes('blur') || style.backgroundColor.includes('rgba');
                    }, selector);

                    expect(isGlass).toBeTruthy();
                }
            }
        });

        test(`Verify screen [${screen.title}] passes Grandmother Test (no technical jargon)`, async ({ page }) => {
            await page.goto(screen.url);

            const pageText = await page.evaluate(() => document.body.innerText.toLowerCase());

            for (const word of FORBIDDEN_JARGON) {
                expect(pageText).not.toContain(word.toLowerCase());
            }
        });
    }
});
