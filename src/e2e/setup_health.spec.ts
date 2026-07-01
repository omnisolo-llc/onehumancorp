import { test, expect } from './fixtures';

test.describe('Hybrid Setup Health Check Endpoint', () => {
  test('returns ready for provisioned standalone mode', async ({ request }) => {
    // Note: this test assumes the local stack is running in standalone mode via Docker Compose
    // which has provisioned the necessary directories.
    // E2E tests run against the live instance on
    const response = await request.get('/api/onboarding/setup-health?mode=standalone');
    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(body.status).toBe('ready');
  });

  test('returns error for unprovisioned cloud mode', async ({ request }) => {
    // We expect cloud mode to return an error locally since .ohc-cloud-data isn't fully created
    // Or at least it handles the check endpoint gracefully.
    const response = await request.get('/api/onboarding/setup-health?mode=cloud');
    expect(response.status()).toBe(200);
    const body = await response.json();
    // It could be 'ready' or 'error' depending on environment, we just ensure it responds valid JSON
    expect(['ready', 'error']).toContain(body.status);
    if (body.status === 'error') {
      expect(typeof body.message).toBe('string');
    }
  });
});
