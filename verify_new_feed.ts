import { chromium } from 'playwright';
import fs from 'fs';
import path from 'path';

async function verifyUnifiedFeed() {
    const browser = await chromium.launch();
    const page = await browser.newPage({
        viewport: { width: 375, height: 812 }, // iPhone 13 Pro size
        deviceScaleFactor: 3,
        isMobile: true,
        hasTouch: true,
    });

    try {
        // Set tenant ID in localStorage
        await page.addInitScript(() => {
            localStorage.setItem('tenant_id', 'test-tenant-feed');
            localStorage.setItem('user_id', 'test-user');
        });

        // Mock the API
        await page.route('**/api/agent-feed*', async (route) => {
            await route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify({
                    items: [
                        {
                            id: 'test-ops-1',
                            tenant_id: 'test-tenant-feed',
                            event_source: 'inventory_agent',
                            lifecycle_state: 'PENDING_APPROVAL',
                            proposed_action: { message: 'Restock Flour' },
                            context_payload: { product: 'Flour', amount: '10kg' },
                            created_at: new Date().toISOString(),
                            updated_at: new Date().toISOString(),
                        },
                        {
                            id: 'test-marketing-1',
                            tenant_id: 'test-tenant-feed',
                            event_source: 'marketing_agent',
                            lifecycle_state: 'PENDING_APPROVAL',
                            proposed_action: { message: 'Launch Winter Sale' },
                            context_payload: { discount: '20%', duration: '7 days' },
                            created_at: new Date().toISOString(),
                            updated_at: new Date().toISOString(),
                        }
                    ]
                })
            });
        });

        console.log('Navigating to dashboard...');
        // Using localhost:3000 assuming the dev server I started earlier is still running
        // Or I can try to find where the app is served.
        // Given I'm in a sandbox, let's try to start the server if not sure.
        // But I'll try localhost first.
        await page.goto('http://localhost:3000/dashboard', { waitUntil: 'networkidle' });

        const screenshotDir = '/home/jules/verification/screenshots';
        if (!fs.existsSync(screenshotDir)) {
            fs.mkdirSync(screenshotDir, { recursive: true });
        }

        const screenshotPath = path.join(screenshotDir, 'unified_feed_mobile_v2.png');
        await page.screenshot({ path: screenshotPath, fullPage: true });
        console.log(`Screenshot saved to ${screenshotPath}`);

        // Verify elements
        const feedHeader = page.locator('h3:has-text("Restock Flour")');
        if (await feedHeader.isVisible()) {
            console.log('✅ Action Card Title is visible');
        } else {
            console.log('❌ Action Card Title is NOT visible');
        }

        const approveBtn = page.locator('button[aria-label="Approve"]').first();
        const box = await approveBtn.boundingBox();
        if (box && box.height >= 44 && box.width >= 44) {
            console.log(`✅ Approve Button touch target is OK: ${box.width}x${box.height}`);
        } else {
            console.log(`❌ Approve Button touch target too small: ${box?.width}x${box?.height}`);
        }

        // Expand Edit
        const editBtn = page.locator('button[aria-label="Edit"]').first();
        await editBtn.click();
        await page.waitForTimeout(500); // animation
        const contextText = page.locator('text=Context & Details');
        if (await contextText.isVisible()) {
            console.log('✅ Expansion works - Context visible');
        } else {
            console.log('❌ Expansion FAILED');
        }

        await page.screenshot({ path: path.join(screenshotDir, 'unified_feed_expanded.png'), fullPage: true });

    } catch (error) {
        console.error('Verification failed:', error);
    } finally {
        await browser.close();
    }
}

verifyUnifiedFeed();
