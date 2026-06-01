import { expect, test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('viral_growth_loops');

test('viral growth loops contain "Powered by OHC"', async ({ page, request }) => {
  // Test the fallback API response logic
  const response = await request.post('/api/v1/growth/campaign/generate-review', {
    data: {
      customer_name: 'E2E User',
      product_name: 'E2E Product',
      order_id: 'e2e-123'
    }
  });

  expect(response.ok()).toBeTruthy();
  const data = await response.json();

  expect(data.message).toContain('E2E User');
  expect(data.message).toContain('E2E Product');
  expect(data.message).toContain('e2e-123');
  expect(data.message).toContain('⚡ Powered by OHC');
});
