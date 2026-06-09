import { test, expect } from './fixtures';

test.describe('Global Search API', () => {
  test('should return search results for a valid query', async ({ request }) => {
    // Make an authenticated request using the shared request context
    const response = await request.get('/api/v1/search?q=Alice');

    // Assert 200 OK
    expect(response.status()).toBe(200);

    const data = await response.json();

    // Check results format
    expect(data.results).toBeDefined();
    expect(Array.isArray(data.results)).toBeTruthy();
  });
});
