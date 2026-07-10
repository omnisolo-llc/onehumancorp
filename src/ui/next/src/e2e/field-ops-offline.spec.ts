import { test, expect } from '../../../../e2e/fixtures';

test.describe('Offline-Tolerant Field Ops CUJ', () => {
  test('Owner completes a field ops job offline and syncs it back', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    // Seed test data using API
    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');
    const appointmentId = 'appt-' + Date.now();
    await page.request.post(`/api/v1/field-ops/appointments`, {
        headers: { 'x-tenant-id': tenantId },
        data: {
            id: appointmentId,
            customer_id: 'cust-1',
            customer_name: 'Offline Test Customer',
            job_template_id: 'job-1',
            job_name: 'Offline Plumbing Repair',
            status: 'Requested',
            scheduled_start_time: new Date().toISOString(),
            scheduled_end_time: new Date(Date.now() + 3600000).toISOString(),
            location_address: '123 Offline St',
            notes: ''
        }
    });

    // 1. Load the Field Ops Jobs page
    await page.goto('/field-ops/jobs');

    // 2. Wait for the page to load and display the job
    await expect(page.getByText('Offline Plumbing Repair')).toBeVisible();
    await expect(page.getByText('Heading to Job')).toBeVisible();

    // 3. Go offline using CDP to simulate offline environment
    const context = page.context();
    await context.setOffline(true);
    await expect(page.getByText('Offline Mode')).toBeVisible();

    // 4. Perform an action - change status
    const headingBtn = page.getByText('Heading to Job');
    await headingBtn.click();

    // Wait for the status to optimistically update (UI only, as we are offline)
    await expect(page.getByText('EN-ROUTE')).toBeVisible();

    // Add notes
    const notesBox = page.locator('textarea').first();
    await notesBox.fill('Arriving shortly. Offline notes.');

    // Mark as started
    const startBtn = page.getByText('Start Work');
    await startBtn.click();
    await expect(page.getByText('IN-PROGRESS')).toBeVisible();

    // Mark as complete
    const doneBtn = page.getByText('Complete & Pay');
    await doneBtn.click();

    // Verify optimistic UI update
    await expect(page.getByText('COMPLETED')).toBeVisible();
    await expect(page.getByText('Saved Notes:')).toBeVisible();

    // 5. Restore connection and wait for sync
    await context.setOffline(false);

    // Give it a moment to sync
    await page.waitForTimeout(2000);

    // Check if the backend updated correctly by going to the backend directly via API
    const getRes = await page.request.get(`/api/v1/field-ops/appointments?tenant_id=${tenantId}`, {
        headers: { 'x-tenant-id': tenantId }
    });

    const data = await getRes.json();
    const syncedJob = data.appointments.find((a: any) => a.id === appointmentId);

    expect(syncedJob).toBeDefined();
    expect(syncedJob.status).toBe('Completed');
    expect(syncedJob.notes).toBe('Arriving shortly. Offline notes.');
  });
});
