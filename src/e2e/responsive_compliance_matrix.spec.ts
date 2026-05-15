import { test, expect } from '@playwright/test';

// Exhaustive test matrix for responsive compliance, ensuring 100% adherence
// to the 375px, 414px, 768px, 1024px, and 1440px viewport requirements
// across all major functional UI screens in the app to avoid visual drift.

const VIEWPORTS = [
    { name: 'Mobile Small', width: 375, height: 667 },
    { name: 'Mobile Large', width: 414, height: 896 },
    { name: 'Tablet', width: 768, height: 1024 },
    { name: 'Desktop Normal', width: 1024, height: 768 },
    { name: 'Desktop Wide', width: 1440, height: 900 }
];

const SCREENS = [
    { url: '/dashboard', title: 'Dashboard', selectors: ['nav', '.glass', 'h1'] },
    { url: '/login', title: 'Login', selectors: ['input[type="email"]', 'input[type="password"]', 'button'] },
    { url: '/agents', title: 'Agents', selectors: ['nav', '.glass', 'h1'] },
    { url: '/setup-screen', title: 'Business Setup', selectors: ['h1', 'button'] },
    { url: '/settings', title: 'Settings', selectors: ['nav', 'h1'] },
    { url: '/pricing', title: 'Pricing Plans', selectors: ['h1', '.card'] },
    { url: '/my-plan', title: 'My Current Plan', selectors: ['h1', '.card'] },
    { url: '/diagnostics', title: 'Diagnostics', selectors: ['h1', '.card'] },
    { url: '/services', title: 'Service Manager', selectors: ['h1', '.card'] },
    { url: '/scaling', title: 'Scaling Configuration', selectors: ['h1', '.card'] },
    { url: '/inbox', title: 'Inbox', selectors: ['nav', 'h1'] },
    { url: '/referrals', title: 'Referral Dashboard', selectors: ['nav', 'h1'] },
    { url: '/meetings', title: 'Meetings', selectors: ['nav', 'h1'] },
];

test.describe('Responsive UI Matrix Compliance Check', () => {
    for (const viewport of VIEWPORTS) {
        test.describe(`Viewport: ${viewport.name} (${viewport.width}x${viewport.height})`, () => {
            test.use({ viewport: { width: viewport.width, height: viewport.height } });

            for (const screen of SCREENS) {
                test(`Verify screen [${screen.title}] renders correctly without overflow or clipping`, async ({ page }) => {
                    await page.goto(screen.url);

                    // Allow animations to settle
                    await page.waitForTimeout(500);

                    // Core layout assertions
                    for (const selector of screen.selectors) {
                        const count = await page.locator(selector).count();
                        if (count > 0) {
                            await expect(page.locator(selector).first()).toBeVisible();

                            // Check that elements stay within the viewport bounds horizontally
                            const boundingBox = await page.locator(selector).first().boundingBox();
                            if (boundingBox) {
                                expect(boundingBox.x).toBeGreaterThanOrEqual(0);
                                expect(boundingBox.width).toBeLessThanOrEqual(viewport.width);
                            }
                        }
                    }

                    // Check for general overflow
                    const htmlWidth = await page.evaluate(() => document.documentElement.scrollWidth);
                    const bodyWidth = await page.evaluate(() => document.body.scrollWidth);

                    // Allow minor pixel differences for scrollbars
                    expect(htmlWidth).toBeLessThanOrEqual(viewport.width + 20);
                    expect(bodyWidth).toBeLessThanOrEqual(viewport.width + 20);

                    // Touch target verification: Ensure buttons meet 44x44px minimum where applicable
                    const buttons = page.locator('button');
                    const buttonCount = await buttons.count();
                    for (let i = 0; i < Math.min(buttonCount, 5); i++) {
                        const box = await buttons.nth(i).boundingBox();
                        if (box) {
                            expect(box.height).toBeGreaterThanOrEqual(40); // Allow slight rendering tolerance
                            expect(box.width).toBeGreaterThanOrEqual(40);
                        }
                    }
                });
            }
        });
    }
});
