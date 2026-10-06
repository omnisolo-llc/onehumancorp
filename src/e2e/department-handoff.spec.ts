import { test, expect } from './fixtures';
import { db } from './db_utils';
import { seedDashboardAuditOwner } from './support/dashboard_audit_fixture';

test.describe('Department Handoff Protocol', () => {
    test('Owner Feed correctly displays and allows approval of Task Envelopes', async ({ anonymousPage: page, loginAs, baseURL }) => {
        if (!baseURL) throw new Error('The isolated local app URL is required');
        const owner = await seedDashboardAuditOwner(baseURL);
        const tenantId = owner.tenantId;
        const envelopeId = `${owner.namespace}-handoff`;
        const remainingId = `${owner.namespace}-remaining-handoff`;
        const ownedIds = [envelopeId, remainingId];
        const initialPayload = {
            title: 'New Custom Cake Inquiry',
            body: 'Customer Service replied. Sales drafted a $150 quote. Ops confirmed delivery date.',
            cost: 15000,
            button_text: 'Approve & Send Quote',
        };
        const remainingPayload = {
            title: 'Another pending departmental handoff',
            body: 'This separate proposal still needs the owner decision.',
            button_text: 'Review another handoff',
        };
        const routingHistory = [
            { department: 'Triage', timestamp: new Date().toISOString() },
            { department: 'Sales', timestamp: new Date().toISOString() },
        ];
        const readSource = (id: string) => db.query(
            'SELECT id, tenant_id, current_department, status, payload, routing_history FROM task_envelopes WHERE id = $1 AND tenant_id = $2',
            [id, tenantId],
        );
        const sourceRow = (id: string, payload: object, status = 'PENDING') => ({
            id, tenant_id: tenantId, current_department: 'Sales', status, payload, routing_history: routingHistory,
        });
        const readReceipts = () => db.query(`SELECT action_id, tenant_id, actor_id, approved, edited_payload, receipt::jsonb AS receipt
            FROM legacy_triage_decisions WHERE tenant_id = $1 AND action_id = ANY($2::text[])`, [tenantId, ownedIds]);

        try {
            // Both proposals belong to this case's verified owner before its first feed read.
            await db.query(`INSERT INTO task_envelopes (id, tenant_id, current_department, status, payload, routing_history)
                VALUES ($1, $3, 'Sales', 'PENDING', $4::jsonb, $6::jsonb),
                       ($2, $3, 'Sales', 'PENDING', $5::jsonb, $6::jsonb)`,
            [envelopeId, remainingId, tenantId, JSON.stringify(initialPayload), JSON.stringify(remainingPayload), JSON.stringify(routingHistory)]);
            expect(await readSource(envelopeId)).toEqual([sourceRow(envelopeId, initialPayload)]);
            expect(await readSource(remainingId)).toEqual([sourceRow(remainingId, remainingPayload)]);
            expect(await readReceipts()).toEqual([]);

            await loginAs(page, { email: owner.email, password: owner.password, role: 'ADMIN', organizationId: tenantId });
            await page.goto(`/ui/triage.html?tenant_id=${tenantId}&bypass_cache=true&t=${Date.now()}`);
            await page.waitForLoadState('networkidle');

            const card = page.locator('#triage-list .triage-card').filter({
                has: page.getByRole('heading', { name: initialPayload.title, exact: true }),
            });
            const remainingCard = page.locator('#triage-list .triage-card').filter({
                has: page.getByRole('heading', { name: remainingPayload.title, exact: true }),
            });
            await expect(card.locator('text=New Custom Cake Inquiry')).toBeVisible({ timeout: 15000 });
            await expect(card.locator('text=Customer Service replied. Sales drafted a $150 quote. Ops confirmed delivery date.')).toBeVisible();
            await expect(card.locator('text=$150.00')).toBeVisible();
            await expect(card.locator('text=$50.00')).toBeVisible(); // 33% deposit
            await expect(remainingCard).toBeVisible();

            const approveButton = card.getByRole('button', { name: 'Approve & Send Quote', exact: true });
            await expect(approveButton).toBeVisible();
            const origin = new URL(baseURL).origin;
            const decisionUrl = new URL(`/api/v1/ui/triage/action?tenant_id=${encodeURIComponent(tenantId)}`, origin).href;
            const submittedDecisions: unknown[] = [];
            page.on('request', request => {
                if (request.url() === decisionUrl && request.method() === 'POST') submittedDecisions.push(request.postDataJSON());
            });
            const [decision] = await Promise.all([
                page.waitForResponse(response => response.url() === decisionUrl && response.request().method() === 'POST'
                    && response.request().postDataJSON()?.triage_item_id === envelopeId),
                approveButton.click(),
            ]);
            expect(decision.status()).toBe(200);
            // The static button supplies an empty edit when there is no textarea.
            // This task has no executable action type; its label is not a delivery receipt.
            const savedPayload = { ...initialPayload, message: '' };
            const receipt = await decision.json();
            expect(receipt).toEqual({
                status: 'success', success: true, decision_recorded: true,
                item: { id: envelopeId, tenant_id: tenantId, lifecycle_state: 'APPROVED', edited_payload: '', proposed_action: savedPayload },
                dispatch: { status: 'NOT_REQUESTED', detail: 'Decision and draft saved; no execution was requested', receipt_id: null },
            });
            const assertStoredDecision = async () => {
                expect(await readSource(envelopeId)).toEqual([sourceRow(envelopeId, savedPayload, 'APPROVED')]);
                expect(await readSource(remainingId)).toEqual([sourceRow(remainingId, remainingPayload)]);
                expect(await readReceipts()).toEqual([{
                    action_id: envelopeId, tenant_id: tenantId, actor_id: owner.userId,
                    approved: true, edited_payload: '', receipt,
                }]);
            };
            await assertStoredDecision();
            await expect(page.locator('text=New Custom Cake Inquiry')).not.toBeVisible();
            await expect(remainingCard.getByRole('button', { name: remainingPayload.button_text, exact: true })).toBeEnabled();

            const assertPendingProjection = (items: { id: string }[]) => {
                expect(Array.isArray(items)).toBe(true);
                expect(items.map(item => item.id)).not.toContain(envelopeId);
                expect(items.find(item => item.id === remainingId)).toMatchObject({
                    id: remainingId, tenant_id: tenantId, intent: 'task_envelope', status: 'PENDING', suggested_actions: remainingPayload,
                });
            };
            const readback = await page.request.get(`/api/v1/ui/triage?tenant_id=${encodeURIComponent(tenantId)}&bypass_cache=true`);
            try {
                expect(readback.status()).toBe(200);
                assertPendingProjection(await readback.json());
            } finally { await readback.dispose(); }
            const [reloaded] = await Promise.all([
                page.waitForResponse(response => new URL(response.url()).origin === origin
                    && new URL(response.url()).pathname === '/api/v1/ui/triage' && response.request().method() === 'GET'),
                page.reload(),
            ]);
            expect(reloaded.status()).toBe(200);
            assertPendingProjection(await reloaded.json());
            await expect(remainingCard.getByRole('button', { name: remainingPayload.button_text, exact: true })).toBeEnabled();
            await expect(page.locator('text=New Custom Cake Inquiry')).not.toBeVisible();
            await assertStoredDecision();
            expect(submittedDecisions).toEqual([{ triage_item_id: envelopeId, approved: true, edited_payload: '' }]);
        } finally {
            await db.query('DELETE FROM legacy_triage_decisions WHERE tenant_id = $1 AND action_id = ANY($2::text[])', [tenantId, ownedIds]);
            await db.query('DELETE FROM task_envelopes WHERE tenant_id = $1 AND id = ANY($2::text[])', [tenantId, ownedIds]);
        }
    });
});
