import { test, expect } from '../fixtures';

test.describe('Booking Slot Redis Redlock Protection', () => {
  test('Prevents double booking via Redis lock during concurrent booking slots', async ({ request, baseURL }) => {
    const payload = {
        data: {
          tenant_id: 'tenant-123',
          customer_id: 'cust-123',
          amount_cents: 1000,
          product_id: 'product-1',
          start_time: new Date(Date.now() + 86400000).toISOString()
        }
    };

    // We expect the server to be up. If it fails to connect, the test naturally fails instead of swallowing it.
    const url = `/api/v1/booking/conversational_checkout`;
    const [res1, res2] = await Promise.all([
      request.post(url, payload),
      request.post(url, payload)
    ]);

    const status1 = res1.status();
    const status2 = res2.status();

    const successCount = [status1, status2].filter(s => s === 200).length;
    const failureCount = [status1, status2].filter(s => s !== 200).length;

    // Strict assertions: We must see exactly 1 success and exactly 1 failure for a pure concurrency test on the exact same resource
    // If the backend isn't up, it throws an error and fails the test.
    // If both return 401 or 500, it also fails because successCount won't be 1.
    expect(successCount).toBe(1);
    expect(failureCount).toBe(1);
  });
});
