import { test, expect } from '@playwright/test';

test('Health Endpoints', async ({ request }) => {
  const healthz = await request.get('/healthz');
  expect(healthz.status()).toBe(200);
  expect(await healthz.text()).toBe('200 OK');

  const readyz = await request.get('/readyz');
  expect(readyz.status()).toBe(200);
  expect(await readyz.text()).toBe('200 OK');
});
