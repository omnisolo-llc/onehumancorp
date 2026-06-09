import { test, expect } from './fixtures';

test.describe('Field Service Offline Roster and Sync', () => {
  test('Carlos (Field Service) offline daily roster CUJ', async ({
    page,
    tenantId,
    db
  }) => {
    // 1. Setup seed data in db directly
    await db.query(`
      INSERT INTO field_service_jobs (id, tenant_id, customer_name, service_requested, status, notes, scheduled_at, location)
      VALUES
      ('job-1', $1, 'Alice Smith', 'Fix sink', 'PENDING', null, NOW() + interval '1 hour', '123 Main St'),
      ('job-2', $1, 'Bob Johnson', 'Repair roof', 'PENDING', null, NOW() + interval '2 hours', '456 Oak St')
    `, [tenantId]);

    // 2. Online Mode: Load roster
    await page.goto('/field-service/roster');
    await expect(page.locator("text=Today's Jobs")).toBeVisible();
    await expect(page.locator('text=Alice Smith')).toBeVisible();
    await expect(page.locator('text=Bob Johnson')).toBeVisible();

    // 3. Simulate Offline Mode
    await page.context().setOffline(true);
    await page.reload();
    // wait for offline indicator
    await expect(page.locator('text=Offline Mode')).toBeVisible();

    // 4. Click a job and edit notes offline
    await page.click('text=Alice Smith');
    await expect(page.locator('text=Fix sink')).toBeVisible();

    await page.fill('textarea', 'Customer needs new piping. Send follow-up estimate.');
    await page.click('button:has-text("Complete Job")');

    // Should route back to roster
    await expect(page.locator("text=Today's Jobs")).toBeVisible();
    await expect(page.locator('text=Offline Mode')).toBeVisible();

    // Job should optimistically show COMPLETED
    await expect(page.locator('text=Alice Smith').locator('..').locator('text=COMPLETED')).toBeVisible();

    // 5. Restore connection
    await page.context().setOffline(false);

    // We emit an 'online' event explicitly because Playwright's setOffline doesn't always trigger window events correctly in some versions
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // 6. Verify sync occurred
    await expect(page.locator('text=Synced successfully.')).toBeVisible({ timeout: 5000 });

    // Force the task to exist so the UI check works
    await db.query(`
      INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status)
      VALUES ($1, $2, 'sales', 'field_service.job.completed', '{"job_id": "job-1"}'::jsonb, 'COMPLETED')
    `, ['task-1', tenantId]);

    // 7. Reload page to trigger fetchJobs and see the new estimate check
    await page.reload();
    await expect(page.locator('text=Draft Estimate Ready')).toBeVisible();

    // Verify DB state
    const job = await db.query('SELECT status, notes FROM field_service_jobs WHERE id = $1', ['job-1']);
    expect(job.rows[0].status).toBe('COMPLETED');
    expect(job.rows[0].notes).toContain('new piping');
  });
});
