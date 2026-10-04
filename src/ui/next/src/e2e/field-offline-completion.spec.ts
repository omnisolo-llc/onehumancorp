import { test, expect } from '../../../../e2e/fixtures';
import { e2eDbQuery } from '../../../../e2e/db_utils';

test('offline completion survives closing its page and keeps unfulfilled invoice work held after reload', async ({ page, context, loginAs, adminUser, seedData }) => {
  const suffix = `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
  const jobId = `offline-handoff-${suffix}`, templateId = `offline-template-${suffix}`;
  const tenantId = seedData.tenant.id;
  await e2eDbQuery("INSERT INTO job_templates (id, tenant_id, name) VALUES ($1, $2, 'Isolated offline handoff')", [templateId, tenantId]);
  await e2eDbQuery(`INSERT INTO appointments (id, tenant_id, customer_id, job_template_id, status, notes, scheduled_start_time, scheduled_end_time, updated_at)
    VALUES ($1, $2, $3, $4, 'In-Progress', '', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP + INTERVAL '1 hour', CURRENT_TIMESTAMP)`, [jobId, tenantId, seedData.customer.id, templateId]);
  await loginAs(page, adminUser); await page.goto('/field-ops/jobs');
  const card = page.getByTestId(`job-card-${jobId}`);
  await expect(card.getByRole('button', { name: 'Job Done', exact: true })).toBeVisible();
  const requests: { path: string; key: string | undefined }[] = [];
  context.on('request', request => {
    const path = new URL(request.url()).pathname;
    if (request.method() === 'POST' && ['/api/v1/sync/events', '/api/v1/invoices/generate'].includes(path)) requests.push({ path, key: request.headers()['idempotency-key'] });
  });
  await context.setOffline(true);
  await expect(page.getByText(/Offline Mode/)).toBeVisible();
  await card.getByRole('button', { name: 'Job Done', exact: true }).click();
  await expect(card.getByText(/Saved locally, awaiting server confirmation/)).toBeVisible();
  // Destroy the view and its closures before any server acknowledgement exists.
  await page.close(); await context.setOffline(false);
  const restored = await context.newPage();
  const invoiceResponse = context.waitForEvent('response', response => new URL(response.url()).pathname === '/api/v1/invoices/generate');
  await restored.goto('/dashboard');
  const invoice = await invoiceResponse;
  // This existing endpoint requires approved billable items. A queued request is
  // deliberately not asserted as an issued invoice or a successful payment.
  expect(invoice.status()).toBe(501);
  expect(await invoice.json()).toMatchObject({ success: false, code: 'approved_line_items_required' });
  await expect.poll(async () => (await e2eDbQuery('SELECT status FROM appointments WHERE id = $1 AND tenant_id = $2', [jobId, tenantId]))[0]?.status).toBe('Completed');
  await expect(restored.getByTestId('offline-queue-readiness')).toHaveAttribute('data-state', 'ready');
  await expect(restored.getByText(/Reconciliation: 1/)).toBeVisible();
  const event = requests.find(request => request.path === '/api/v1/sync/events');
  expect(event?.key).toBeTruthy();
  expect(requests.filter(request => request.path === '/api/v1/invoices/generate')).toEqual([{ path: '/api/v1/invoices/generate', key: `field-completion-${event!.key}-invoice` }]);
  await restored.reload();
  await expect(restored.getByTestId('offline-queue-readiness')).toHaveAttribute('data-state', 'ready');
  await expect(restored.getByText(/Reconciliation: 1/)).toBeVisible();
  expect(requests.filter(request => request.path === '/api/v1/sync/events')).toHaveLength(1);
  expect(requests.filter(request => request.path === '/api/v1/invoices/generate')).toHaveLength(1);
  await restored.close();
});
