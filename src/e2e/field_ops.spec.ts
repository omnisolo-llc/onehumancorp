import { test, expect } from '@playwright/test';

test.describe('Field Ops Appointments', () => {
  test('should load appointments successfully from backend', async ({ request }) => {
    // We send a request to the backend using the field-ops endpoint
    const response = await request.get('/api/v1/field-ops/appointments?tenant_id=storefront');

    // Expect success
    expect(response.ok()).toBeTruthy();

    const body = await response.json();

    // Validate schema
    expect(body).toHaveProperty('appointments');
    expect(Array.isArray(body.appointments)).toBeTruthy();
  });

  test('should update appointment status', async ({ request }) => {
    const response = await request.post('/api/v1/field-ops/appointments', {
      data: {
        id: 'appt-1',
        status: 'Completed',
        notes: 'Fixed the leak'
      }
    });

    expect(response.ok()).toBeTruthy();
    const body = await response.json();
    expect(body.success).toBe(true);
    expect(body.status).toBe('Completed');
  });

  test('should handle invalid requests', async ({ request }) => {
    // Missing required fields
    const response = await request.post('/api/v1/field-ops/appointments', {
      data: {
        status: 'Completed'
      }
    });
    // It should fail in db query or validation
    expect(response.ok()).toBeFalsy();
  });

  test('should fail gracefully when backend query errors on invalid tenant', async ({ request }) => {
    const response = await request.get('/api/v1/field-ops/appointments?tenant_id=');
    // If tenant empty, it might still return [] or an error.
    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(body.appointments).toEqual([]);
  });

  test('should return empty list for unknown tenant', async ({ request }) => {
    const response = await request.get('/api/v1/field-ops/appointments?tenant_id=unknown123');
    expect(response.ok()).toBeTruthy();
    const body = await response.json();
    expect(body.appointments).toEqual([]);
  });

  test('should optimize route based on distance', async ({ request }) => {
    const response = await request.post('/api/v1/field-ops/optimize-route', {
      data: {
        appointments: [
          { id: '1', status: 'Scheduled', location_lat: 40.7128, location_lng: -74.0060, notes: '' }, // NY
          { id: '2', status: 'Scheduled', location_lat: 34.0522, location_lng: -118.2437, notes: '' }, // LA
          { id: '3', status: 'Scheduled', location_lat: 41.8781, location_lng: -87.6298, notes: '' }  // Chicago
        ],
        currentLocationLat: 40.0,
        currentLocationLng: -74.0
      }
    });

    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(body.success).toBe(true);
    expect(body.optimizedRoute.length).toBe(3);

    // NY is closest to 40,-74, then Chicago, then LA
    expect(body.optimizedRoute[0].id).toBe('1');
    expect(body.optimizedRoute[1].id).toBe('3');
    expect(body.optimizedRoute[2].id).toBe('2');

    // Check travel time mock insertion
    expect(body.optimizedRoute[0].notes).toContain('[Travel:');
  });

  test('should update subsequent jobs when running late', async ({ request }) => {
    const t1 = "2023-10-10T09:00:00Z";
    const t2 = "2023-10-10T10:00:00Z";
    const t3 = "2023-10-10T11:00:00Z";

    const response = await request.post('/api/v1/field-ops/running-late', {
      data: {
        delayJobId: 'job2',
        appointments: [
          {
            id: 'job1',
            customer_id: 'c1',
            customer_name: 'Customer 1',
            job_template_id: 'jt1',
            job_name: 'Job 1',
            status: 'Completed',
            scheduled_start_time: t1,
            scheduled_end_time: t1
          },
          {
            id: 'job2',
            customer_id: 'c2',
            customer_name: 'Customer 2',
            job_template_id: 'jt2',
            job_name: 'Job 2',
            status: 'In-Progress',
            scheduled_start_time: t2,
            scheduled_end_time: t2
          },
          {
            id: 'job3',
            customer_id: 'c3',
            customer_name: 'Customer 3',
            job_template_id: 'jt3',
            job_name: 'Job 3',
            status: 'Scheduled',
            scheduled_start_time: t3,
            scheduled_end_time: t3
          }
        ]
      }
    });

    expect(response.ok()).toBeTruthy();
    const body = await response.json();
    expect(body.success).toBe(true);
    expect(body.subsequentCount).toBe(1);
    expect(body.optimizedRoute.length).toBe(3);

    // Test the expected delay in the output payload
    const originalDate = new Date(t3);
    const delayedDate = new Date(originalDate.getTime() + 15 * 60000);
    expect(body.optimizedRoute[2].scheduled_start_time).toBe(delayedDate.toISOString());
  });
});
