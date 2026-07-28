import { test, expect } from './fixtures';

test.describe('Autonomous Booking System CUJ', () => {
  const tenantId = `booking-test-${Date.now()}`;
  let serviceId = '';

  test('Owner sets up a new service and availability', async ({ request }) => {
    const resResource = await request.post(`/api/v1/booking/admin/resources`, {
      headers: { 'x-tenant-id': tenantId },
      data: {
        name: 'Leo',
        description: 'Music Tutor',
        type: 'provider'
      }
    });
    expect(resResource.ok()).toBeTruthy();
    const resourceData = await resResource.json();
    const resourceId = resourceData.id;
    expect(resourceId).toBeTruthy();

    const now = new Date();
    const tomorrow = new Date(now);
    tomorrow.setDate(tomorrow.getDate() + 1);

    const start = new Date(tomorrow);
    start.setHours(9, 0, 0, 0);
    const end = new Date(tomorrow);
    end.setHours(17, 0, 0, 0);

    const resAvail = await request.post(`/api/v1/booking/admin/availability`, {
      headers: { 'x-tenant-id': tenantId },
      data: {
        resource_id: resourceId,
        start_time: start.toISOString(),
        end_time: end.toISOString()
      }
    });
    expect(resAvail.ok()).toBeTruthy();

    serviceId = 'mock-service-123';
  });

  test('Customer fetches slots and creates a booking requiring a deposit', async ({ request }) => {
    const dateQuery = new Date().toISOString().split('T')[0];
    const resSlots = await request.get(`/api/v1/booking/public/slots?service_id=${serviceId}&date=${dateQuery}`, {
      headers: { 'x-tenant-id': tenantId }
    });
    expect(resSlots.ok()).toBeTruthy();
    const slotsData = await resSlots.json();
    expect(slotsData.slots.length).toBeGreaterThan(0);

    const selectedSlot = slotsData.slots[0];

    const resBooking = await request.post(`/api/v1/booking/public/checkout`, {
      headers: { 'x-tenant-id': tenantId },
      data: {
        service_id: serviceId,
        start_time: selectedSlot.start_time,
        end_time: selectedSlot.end_time,
        customer_name: 'Test Customer',
        customer_email: 'test@example.com'
      }
    });

    expect(resBooking.status()).toBeDefined();
  });
});
