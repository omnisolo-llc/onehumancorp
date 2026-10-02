import { test, expect } from './fixtures';

test.describe("Mobile Payload Optimization", () => {
  test.use({ viewport: { width: 375, height: 667 } });

  test('supply returns mobile_optimized payload', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);

    // Use the real browser session authenticated by loginAs; do not invent token fields.
    const response = await page.request.get('/api/v1/ui/supply?mobile_optimized=true');

    expect(response.status()).toBe(200);
    const data = await response.json();

    expect(data.vendors).toBeDefined();
    if (data.vendors && data.vendors.length > 0) {
        expect(data.vendors[0].contact_info).toBeUndefined();
    }

    expect(data.raw_materials).toBeDefined();
    if (data.raw_materials && data.raw_materials.length > 0) {
        expect(data.raw_materials[0].reorder_threshold).toBeUndefined();
    }
  });

  test('list_jobs returns mobile_optimized payload', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    // Use the real browser session authenticated by loginAs; do not invent token fields.
    const response = await page.request.get('/api/v1/ohc-job-queue/?mobile_optimized=true');

    expect(response.status()).toBe(200);
    const data = await response.json();

    expect(data.jobs).toBeDefined();
    if (data.jobs && data.jobs.length > 0) {
        expect(data.jobs[0].retry_count).toBeUndefined();
    }
  });

});
