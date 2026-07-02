import { test, expect } from '@playwright/test';
import { memberPage } from './fixtures';
import { e2eDbQuery as executeSql } from './db_utils';

test.describe('Agentic Field Service Scheduling & Quoting', () => {
  let customerId: string;
  let serviceId: string;

  test.beforeAll(async () => {
    await executeSql(`
      DELETE FROM interactive_proposal_line_items;
      DELETE FROM interactive_proposals;
      DELETE FROM booking_slots;
    `);

    const customerRes = await executeSql(`
      INSERT INTO customers (id, tenant_id, name, email)
      VALUES (gen_random_uuid(), 'e2e-tenant', 'Test Customer', 'carlos-client@example.com')
      RETURNING id
    `);
    customerId = customerRes[0].id;

    const serviceRes = await executeSql(`
      INSERT INTO services (id, tenant_id, title, price_cents)
      VALUES ('svc-' || gen_random_uuid(), 'e2e-tenant', 'Plumbing Repair', 15000)
      RETURNING id
    `);
    serviceId = serviceRes[0].id;
  });

  test('Concurrent requests for the same slot are prevented by Redlock', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/');

    const startTime = new Date();
    startTime.setUTCHours(startTime.getUTCHours() + 24);
    const endTime = new Date(startTime);
    endTime.setUTCHours(endTime.getUTCHours() + 1);

    const startTimeStr = startTime.toISOString();
    const endTimeStr = endTime.toISOString();

    const req1 = page.request.post('/api/booking/reserve', {
      data: {
        customer_id: customerId,
        product_id: serviceId,
        start_time: startTimeStr,
        end_time: endTimeStr,
        requires_deposit: true,
        timezone: 'UTC',
      },
    });

    const req2 = page.request.post('/api/booking/reserve', {
      data: {
        customer_id: customerId,
        product_id: serviceId,
        start_time: startTimeStr,
        end_time: endTimeStr,
        requires_deposit: true,
        timezone: 'UTC',
      },
    });

    const [res1, res2] = await Promise.all([req1, req2]);

    const status1 = res1.status();
    const status2 = res2.status();

    expect((status1 === 200 && status2 === 409) || (status1 === 409 && status2 === 200)).toBeTruthy();

    if (status1 === 200) {
       const body = await res1.json();
       expect(body.booking_id).toBeDefined();
    } else {
       const body = await res2.json();
       expect(body.booking_id).toBeDefined();
    }
  });
});
