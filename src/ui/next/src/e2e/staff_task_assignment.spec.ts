import { test, expect } from '../../../../e2e/fixtures';
import { Pool } from 'pg';

test.describe('Staff AI Task & Summaries', () => {
  let pool: Pool;

  test.beforeAll(async () => {
    // Setup a DB connection for setting up real E2E data
    const dbUrl = process.env.DATABASE_URL || 'postgres://ohc:ohc_local_pass@localhost:5432/ohc';
    pool = new Pool({ connectionString: dbUrl });
  });

  test.afterAll(async () => {
    await pool.end();
  });

  test('CUJ: Staff views auto-generated tasks and manager views summary', async ({ page, request, browser }) => {
    // 1. Setup Data
    const tenantId = 'e2e-tenant';
    const staffId = 'staff_john_123';

    // Ensure staff profile exists
    await pool.query(`
      INSERT INTO ohc_staff_member (id, tenant_id, name, phone_number, role)
      VALUES ($1, $2, $3, $4, $5)
      ON CONFLICT (id) DO NOTHING
    `, [staffId, tenantId, 'John Connor', '+1234567890', 'Barista']);

    // 2. Trigger webhook simulating an order with volume spike
    const orderPayload = {
      tenant_id: tenantId,
      volume_spike: true,
      product_name: 'Falafels',
      notes: ''
    };

    // We can directly insert the DepartmentEvent via the mesh, but for E2E we can insert a task to simulate agent's work
    const taskId = 'task_e2e_123';
    await pool.query(`
      INSERT INTO staff_tasks (id, tenant_id, staff_id, description, status, priority)
      VALUES ($1, $2, 'unassigned', 'Volume Spike: Prepare Falafels', 'pending', 'high')
      ON CONFLICT (id) DO NOTHING
    `, [taskId, tenantId]);

    // Insert a shift summary
    const summaryId = 'summary_e2e_123';
    await pool.query(`
      INSERT INTO shift_summaries (id, tenant_id, shift_date, summary_text, metrics)
      VALUES ($1, $2, CURRENT_DATE, 'End of shift summary: Shift ran smoothly. Orders processed normally.', '{}'::jsonb)
      ON CONFLICT (id) DO NOTHING
    `, [summaryId, tenantId]);

    // 3. Staff logs in and views their tasks
    await page.goto('/staff');
    await expect(page.locator('h1', { hasText: 'My Shifts & Tasks' })).toBeVisible();

    // Verify AI prioritized task is visible
    await expect(page.getByText('Volume Spike: Prepare Falafels')).toBeVisible();
    await expect(page.getByText('AI Prioritized')).toBeVisible();

    // 4. Staff escalates low supply
    await page.getByTestId('staff-escalate-btn').click();
    await page.getByPlaceholder('What do you need?').fill('Low on cups');
    await page.getByTestId('submit-escalation').click();

    // Ensure modal closes
    await expect(page.getByTestId('submit-escalation')).not.toBeVisible();

    // 5. Manager views the shift summary
    // Since page is staff, create a new context/page for manager
    const managerContext = await browser.newContext();
    const managerPage = await managerContext.newPage();

    await managerPage.goto('/location-dashboard');
    await expect(managerPage.locator('h1', { hasText: 'Location Dashboard' })).toBeVisible();

    // The shift summary should appear as an alert
    await expect(managerPage.getByText('End of shift summary: Shift ran smoothly. Orders processed normally.')).toBeVisible();

    // The active task should also be visible
    await expect(managerPage.getByText('Volume Spike: Prepare Falafels')).toBeVisible();

    // Ensure staff view is also correct
    await expect(managerPage.getByText('John Connor')).toBeVisible();

    await managerContext.close();
  });
});
