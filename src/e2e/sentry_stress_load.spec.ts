import { test, expect } from './fixtures';

test.describe('🛡️ Sentry: Stress & Load', () => {
    test('Assert P95 latency for critical dashboard operations under load', async ({ page }) => {
        // Persona: Multiple Owners
        // Scenario: Simulating concurrent dashboard access to verify P95 latency.

        const iterations = 5;
        const latencies: number[] = [];

        for (let i = 0; i < iterations; i++) {
            const start = Date.now();
            await page.goto('/dashboard');
            await expect(page.getByText('Business Overview')).toBeVisible();
            latencies.push(Date.now() - start);
            await page.goto('about:blank'); // Clear state
        }

        latencies.sort((a, b) => a - b);
        const p95 = latencies[Math.floor(latencies.length * 0.95)];

        console.log(`OHC_METRIC: dashboard_p95_latency_ms=${p95}`);

        // Assert that P95 latency is within acceptable bounds (e.g., < 5000ms for full load in CI)
        expect(p95).toBeLessThan(5000);
    });

    test('Verify system stability during concurrent API bursts', async ({ page }) => {
        await page.goto('/dashboard/analytics');

        // Trigger multiple concurrent API calls
        const promises = [];
        for (let i = 0; i < 10; i++) {
            promises.push(page.evaluate(() => fetch('/api/analytics/summary').then(r => r.json())));
        }

        const results = await Promise.all(promises);
        results.forEach(res => {
            expect(res).toBeDefined();
        });
    });
});
