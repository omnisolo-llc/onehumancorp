import { test, expect } from '@playwright/test';

test.describe('AutoDream API endpoints', () => {
  test('should return 200 for a basic POST query', async ({ request }) => {
    const response = await request.post('/api/autodream/query', {
      data: {
        limit: 5,
        embedding: Array(1536).fill(0.1)
      }
    });
    expect(response.status()).toBe(200);
  });

  test('should return 405 for GET request', async ({ request }) => {
    const response = await request.get('/api/autodream/query');
    expect(response.status()).toBe(405);
  });
});
