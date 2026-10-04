import { randomUUID } from 'node:crypto';
import { e2eDbQuery } from '../db_utils';

/** Uses only the runner-verified isolated database; each journey owns its rows. */
export async function seedRoutingJobs(tenantId: string, customerId: string) {
  const suffix = randomUUID();
  const routeId = `html-route-${suffix}`;
  const ids = [1, 2].map(index => `html-job-${suffix}-${index}`);
  await e2eDbQuery("INSERT INTO service_routes (id, tenant_id, route_date, status) VALUES ($1, $2, CURRENT_DATE, 'active')", [routeId, tenantId]);
  for (const [index, id] of ids.entries()) {
    const appointmentId = `html-appointment-${suffix}-${index}`;
    await e2eDbQuery(`INSERT INTO appointments (id, tenant_id, customer_id, job_template_id, status, scheduled_start_time, scheduled_end_time, location_address)
      VALUES ($1, $2, $3, 'e2e-template-1', 'Scheduled', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP + INTERVAL '1 hour', '123 Main St, Austin, TX')`, [appointmentId, tenantId, customerId]);
    await e2eDbQuery(`INSERT INTO job_locations (id, tenant_id, service_route_id, appointment_id, sequence_order, status)
      VALUES ($1, $2, $3, $4, $5, 'pending')`, [id, tenantId, routeId, appointmentId, index]);
  }
  return ids;
}
