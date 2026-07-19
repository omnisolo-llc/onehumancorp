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
