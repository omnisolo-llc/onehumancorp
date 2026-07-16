import { test, expect } from '@playwright/test';
import { Pool } from 'pg';

const OHC_DATABASE_URL = process.env.OHC_DATABASE_URL || "postgres://postgres:postgres@127.0.0.1:5432/ohc";

test.describe('Universal Event Bus and Async Queue (ohc_async_jobs)', () => {
    let pool: Pool;

    test.beforeAll(async () => {
        pool = new Pool({ connectionString: OHC_DATABASE_URL });
    });

    test.afterAll(async () => {
        await pool.end();
    });

    test('Simulate concurrent unified inbox webhooks pushing to ohc_async_jobs', async ({ request }) => {
        const testTenant = 'test-async-tenant';

        // Ensure test environment is somewhat clean
        await pool.query('DELETE FROM ohc_async_jobs WHERE tenant_id = $1', [testTenant]);

        // 1. Simulate 3 concurrent webhooks arriving at the same time
        const webhookPromises = [
            request.post('/api/v1/webhooks/unified_inbox', {
                data: {
                    tenant_id: testTenant,
                    source: 'instagram',
                    identifier: 'ig-user-1',
                    message: 'I want a custom birthday cake!'
                }
            }),
            request.post('/api/v1/webhooks/unified_inbox', {
                data: {
                    tenant_id: testTenant,
                    source: 'instagram',
                    identifier: 'ig-user-2',
                    message: 'How much for a wedding cake?'
                }
            }),
            request.post('/api/v1/webhooks/unified_inbox', {
                data: {
                    tenant_id: testTenant,
                    source: 'instagram',
                    identifier: 'ig-user-3',
                    message: 'Do you make vegan cakes?'
                }
            })
        ];

        const responses = await Promise.all(webhookPromises);
        for (const r of responses) {
            expect(r.ok()).toBeTruthy();
        }

        // 2. Wait a moment to ensure async writes occur
        await new Promise(res => setTimeout(res, 2000));

        // 3. Verify that we have 3 pending/completed events in ohc_async_jobs
        const queueRes = await pool.query('SELECT * FROM ohc_async_jobs WHERE tenant_id = $1 AND job_type = $2', [testTenant, 'customer_support']);
        expect(queueRes.rowCount).toBeGreaterThanOrEqual(3);

        const payloads = queueRes.rows.map(r => typeof r.payload === 'string' ? JSON.parse(r.payload) : r.payload);

        expect(payloads.some(p => p.message === 'I want a custom birthday cake!')).toBeTruthy();
        expect(payloads.some(p => p.message === 'How much for a wedding cake?')).toBeTruthy();
        expect(payloads.some(p => p.message === 'Do you make vegan cakes?')).toBeTruthy();

        // 4. Ideally, EventRouterWorker picks this up and pushes to work_intents or ohc_job_queue (legacy triage queue).
        // Let's verify if work_intents has it. Wait a bit more for worker to process.
        await new Promise(res => setTimeout(res, 2000));

        const intentsRes = await pool.query('SELECT * FROM work_intents WHERE tenant_id = $1 AND intent_type = $2', [testTenant, 'customer_inquiry']);
        expect(intentsRes.rowCount).toBeGreaterThanOrEqual(3);

        // Clean up
        await pool.query('DELETE FROM ohc_async_jobs WHERE tenant_id = $1', [testTenant]);
        await pool.query('DELETE FROM work_intents WHERE tenant_id = $1', [testTenant]);
    });
});
