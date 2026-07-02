import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';
import { db } from './e2e-seed';

test.describe('Department Handoff Protocol', () => {
    test('Owner Feed correctly displays and allows approval of Task Envelopes', async ({ adminPage: page }) => {
        // 1. Arrange: Seed a TaskEnvelope directly into the database to simulate background agent work
        const tenantId = 'e2e-tenant';
        const envelopeId = `env-${Date.now()}`;
        const initialPayload = JSON.stringify({
            title: "New Custom Cake Inquiry",
            body: "Customer Service replied. Sales drafted a $150 quote. Ops confirmed delivery date.",
            cost: 15000,
            button_text: "Approve & Send Quote"
        });
        const routingHistory = JSON.stringify([
            { department: "Triage", timestamp: new Date().toISOString() },
            { department: "Sales", timestamp: new Date().toISOString() }
        ]);

        await db.query(`
            INSERT INTO task_envelopes (id, tenant_id, current_department, status, payload, routing_history)
            VALUES ($1, $2, 'Sales', 'PENDING', $3::jsonb, $4::jsonb)
        `, [envelopeId, tenantId, initialPayload, routingHistory]);

        // 2. Act: Owner navigates to the Work Triage feed
        await page.goto('/ui/triage.html');
        await page.waitForLoadState('networkidle');

        // 3. Assert: The task envelope is displayed correctly
        await expect(page.locator('text=New Custom Cake Inquiry')).toBeVisible();
        await expect(page.locator('text=Customer Service replied. Sales drafted a $150 quote. Ops confirmed delivery date.')).toBeVisible();
        await expect(page.locator('text=$150.00')).toBeVisible();
        await expect(page.locator('text=$50.00')).toBeVisible(); // 33% deposit

        // 4. Act: Owner clicks "Approve & Send Quote"
        const approveButton = page.locator('button:has-text("Approve & Send Quote")');
        await expect(approveButton).toBeVisible();
        await approveButton.click();

        // 5. Assert: The item is removed and marked completed
        await page.waitForResponse(response => response.url().includes('/api/ui/triage/action') && response.status() === 200);
        await expect(page.locator('text=New Custom Cake Inquiry')).not.toBeVisible();

        // Clean up the DB
        await db.query(`DELETE FROM task_envelopes WHERE id = $1`, [envelopeId]);
    });
});
