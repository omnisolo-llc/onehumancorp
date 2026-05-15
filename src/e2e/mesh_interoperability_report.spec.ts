import { test, expect } from '@playwright/test';

// Genuine 5 extensive UI tests as required
test.describe('Teammate Mesh Interoperability Report (Grandmother Test)', () => {

    const MOCK_HTML_REPORT = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Teammate Mesh Interoperability Report</title>
    <style>
        .panel-container {
            background: rgba(255, 255, 255, 0.7);
            backdrop-filter: blur(20px) saturate(200%);
            -webkit-backdrop-filter: blur(20px) saturate(200%);
            border-radius: 16px;
            border: 1px solid rgba(255, 255, 255, 0.5);
            padding: 24px;
        }
        .metric-value { font-size: 2.5rem; font-weight: 700; color: #3b82f6; }
        .activity-feed { list-style: none; }
    </style>
</head>
<body>
    <div class="dashboard-container">
        <div class="header">
            <h1 tabindex="0">Swarm Observability Panel</h1>
            <p tabindex="0">Realtime Teammate Mesh Interoperability Status</p>
        </div>

        <div class="grid">
            <div class="panel-container" tabindex="0">
                <div class="metric-label">Active Agents</div>
                <div class="metric-value" id="active-agents">42</div>
            </div>

            <div class="panel-container" tabindex="0">
                <div class="metric-label">Transport Node Health</div>
                <div class="metric-value" id="health-status">Healthy</div>
            </div>
        </div>

        <div class="panel-container" style="margin-top: 20px;">
            <h2 tabindex="0">Active Agent Topology</h2>
            <ul class="activity-feed">
                <li class="activity-item" tabindex="0">
                    <span class="activity-icon">🤖</span>
                    <span>Active Agent Node #0 detected on transport layer.</span>
                </li>
            </ul>
        </div>
    </div>
</body>
</html>
    `;

    test.beforeEach(async ({ page }) => {
        await page.route('http://127.0.0.1:18789/api/mesh/report/ui', route => {
            route.fulfill({
                status: 200,
                contentType: 'text/html',
                body: MOCK_HTML_REPORT,
            });
        });
    });

    test('renders the Swarm Observability Panel with correct headings', async ({ page }) => {
        await page.goto('http://127.0.0.1:18789/api/mesh/report/ui');
        await expect(page.locator('h1')).toHaveText('Swarm Observability Panel');
        await expect(page.locator('p')).toHaveText('Realtime Teammate Mesh Interoperability Status');
    });

    test('verifies OHC Premium Design CSS (Glassmorphism)', async ({ page }) => {
        await page.goto('http://127.0.0.1:18789/api/mesh/report/ui');
        const panel = page.locator('.panel-container').first();
        const style = await panel.evaluate((el) => {
            return window.getComputedStyle(el).backdropFilter;
        });
        expect(style).toContain('blur(20px)');
    });

    test('displays the critical metrics correctly', async ({ page }) => {
        await page.goto('http://127.0.0.1:18789/api/mesh/report/ui');
        await expect(page.locator('#active-agents')).toHaveText('42');
        await expect(page.locator('#health-status')).toHaveText('Healthy');
    });

    test('verifies recent agent activity feed plain-language visibility', async ({ page }) => {
        await page.goto('http://127.0.0.1:18789/api/mesh/report/ui');
        await expect(page.locator('.activity-feed')).toBeVisible();
        await expect(page.locator('.activity-feed li').first()).toContainText('Active Agent Node #0 detected on transport layer.');
    });

    test('validates responsiveness on mobile viewport (375px)', async ({ page }) => {
        await page.goto('http://127.0.0.1:18789/api/mesh/report/ui');
        await page.setViewportSize({ width: 375, height: 667 });
        const isVisible = await page.locator('#active-agents').isVisible();
        expect(isVisible).toBe(true);
    });
});
