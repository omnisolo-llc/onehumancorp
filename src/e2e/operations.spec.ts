import { randomUUID } from 'node:crypto';
import { test, expect } from './fixtures';
import { createGrowthOwner } from './growth_owner';
import { e2eDbQuery } from './db_utils';
test.describe('Operations recorded appointments', () => {
    test('loads a real owned schedule and excludes another business', async ({ page, baseURL }) => {
        if (!baseURL)
            throw new Error('A local browser fixture URL is required');
        const origin = new URL(baseURL);
        expect(['http:', 'https:']).toContain(origin.protocol);
        expect(['127.0.0.1', 'localhost', '[::1]']).toContain(origin.hostname);
        const owner = await createGrowthOwner(page, baseURL);
        const suffix = randomUUID();
        const foreign = `e2e-operations-foreign-${suffix}`;
        await e2eDbQuery("INSERT INTO tenants(id,name)VALUES($1,'Foreign owned test fixture')", [foreign]);
        const expected = [] as {
            id: string;
            name: string;
            status: string;
            start: string;
        }[];
        for (const [index, offset] of [-2, 0, 2].entries()) {
            const id = `owned-appointment-${suffix}-${index}`;
            const customer = `owned-customer-${suffix}-${index}`;
            const template = `owned-template-${suffix}-${index}`;
            const start = new Date(Date.now() + offset * 3600000).toISOString();
            const end = new Date(Date.parse(start) + 3600000).toISOString();
            const status = ['Completed', 'Confirmed', 'Requested'][index];
            const name = `Recorded service ${index} ${suffix}`;
            await e2eDbQuery('INSERT INTO customers(id,tenant_id,name)VALUES($1,$2,$3)', [customer, owner.tenantId, `Recorded customer ${index}`]);
            await e2eDbQuery('INSERT INTO job_templates(id,tenant_id,name)VALUES($1,$2,$3)', [template, owner.tenantId, name]);
            await e2eDbQuery('INSERT INTO appointments(id,tenant_id,customer_id,job_template_id,status,scheduled_start_time,scheduled_end_time,notes)VALUES($1,$2,$3,$4,$5,$6,$7,$8)', [id, owner.tenantId, customer, template, status, start, end, `Owner-entered appointment note ${index}`]);
            expected.push({ id, name, status, start });
        }
        await e2eDbQuery("INSERT INTO job_templates(id,tenant_id,name)VALUES($1,$2,'Foreign private service')", [`foreign-template-${suffix}`, foreign]);
        await e2eDbQuery("INSERT INTO appointments(id,tenant_id,job_template_id,status)VALUES($1,$2,$3,'Requested')", [`foreign-appointment-${suffix}`, foreign, `foreign-template-${suffix}`]);
        const reading = page.waitForResponse(response => new URL(response.url()).origin === origin.origin && new URL(response.url()).pathname === '/api/v1/field-ops/appointments' && response.request().method() === 'GET').then(async (response) => ({ status: response.status(), body: await response.json() }));
        await page.goto('/operations');
        const receipt = await reading;
        expect(receipt.status).toBe(200);
        expect(receipt.body.appointments.map((row: {
            id: string;
        }) => row.id).sort()).toEqual(expected.map(row => row.id).sort());
        await expect(page.getByRole('heading', { name: 'Appointment schedule', exact: true })).toBeVisible();
        await expect(page.getByRole('heading', { name: 'Schedule overview', exact: true })).toBeVisible();
        await expect(page.getByText('3 recorded appointments.')).toBeVisible();
        for (const row of expected) {
            const card = page.getByTestId(`appointment-${row.id}`);
            await expect(card.getByRole('heading', { name: row.name, exact: true })).toBeVisible();
            await expect(card.getByText(row.status, { exact: true })).toBeVisible();
            expect(Date.parse((await card.locator('time').first().getAttribute('datetime'))!)).toBe(Date.parse(row.start));
        }
        await expect(page.getByText('Foreign private service')).toHaveCount(0);
        await expect(page.getByText('AI Summary:', { exact: true })).toHaveCount(0);
        await expect(page.getByText('Deposit Required', { exact: true })).toHaveCount(0);
        await expect(page.getByText('Messaging and reminder dispatch are not available from this schedule.')).toBeVisible();
    });
    test('shows the actual empty-business state without sample appointments', async ({ page, baseURL }) => {
        if (!baseURL)
            throw new Error('A local browser fixture URL is required');
        const origin = new URL(baseURL);
        expect(['http:', 'https:']).toContain(origin.protocol);
        expect(['127.0.0.1', 'localhost', '[::1]']).toContain(origin.hostname);
        await createGrowthOwner(page, baseURL);
        const reading = page.waitForResponse(response => new URL(response.url()).origin === origin.origin && new URL(response.url()).pathname === '/api/v1/field-ops/appointments' && response.request().method() === 'GET').then(async (response) => ({ status: response.status(), body: await response.json() }));
        await page.goto('/operations');
        const receipt = await reading;
        expect(receipt.status).toBe(200);
        expect(receipt.body).toEqual({ appointments: [] });
        await expect(page.getByText('No appointments recorded for this business.')).toBeVisible();
        await expect(page.getByText('Alice Smith', { exact: true })).toHaveCount(0);
        await expect(page.getByRole('list', { name: 'Recorded appointments' })).toHaveCount(0);
    });
});
