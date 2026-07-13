import { test, expect } from './fixtures';

test.describe('Stress Verification', () => {
  test('handles 100 concurrent health-check requests in Cloud mode under 500ms p95 latency', async ({ request }) => {
    const concurrentRequests = 100;
    const startTimes: number[] = [];
    const latencies: number[] = [];

    const promises = Array.from({ length: concurrentRequests }, async (_, i) => {
      const start = Date.now();
      startTimes[i] = start;
      const res = await request.get('/health');
      const end = Date.now();

      expect(res.ok()).toBeTruthy();

      latencies.push(end - start);
    });

    await Promise.all(promises);

    latencies.sort((a, b) => a - b);

    // Calculate p95 latency
    const p95Index = Math.floor(latencies.length * 0.95);
    const p95Latency = latencies[p95Index];

    console.info(`p95 Latency: ${p95Latency}ms`);

    expect(p95Latency).toBeLessThan(500);
  });
});
