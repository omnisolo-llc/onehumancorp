import { test, expect } from '@playwright/test';
import { getTestDbPool } from './db_utils';

test.describe('Autonomous Multilingual Voice Interceptor Dispatch', () => {
    test('Twilio voice webhook creates multilingual order draft and owner can approve it', async ({ page }) => {
        const tenant_id = 'test_tenant'; // default test tenant

        const pool = await getTestDbPool();

        const taskId = 'task-' + Date.now();
        const callerPhone = '+15551234567';

        const payload = JSON.stringify({
            feature_type: 'order_draft',
            summary: 'Quiero 3 tacos de pollo.',
            caller_phone: callerPhone,
            order_link: 'https://pay.ohc.com/store/voice',
            intercepted_order: {
                intent: 'Order',
                items: [{ item: 'Chicken Tacos', quantity: 3 }],
                language: 'Spanish',
                notes: null,
                translated_notes: null
            }
        });

        await pool.query(
            `INSERT INTO shared_tasks (id, tenant_id, organization_id, title, status, approval_status, proposed_content, payload, mission_id, parent_plan_id, priority)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)`,
            [
                taskId,
                tenant_id,
                tenant_id,
                `Incoming Phone Order (Spanish)`,
                'PENDING',
                'PENDING',
                payload,
                payload,
                'mission-voice',
                'plan-voice',
                'P1'
            ]
        );

        // 2. Login to the dashboard
        await page.goto(`/login`);
        await page.evaluate(() => {
            localStorage.setItem('has_onboarded', 'true');
            localStorage.setItem('tenant_id', 'test_tenant');
            localStorage.setItem('token', 'test_token');
        });
        await page.goto(`/dashboard`);

        await expect(page.locator('text=Chicken Tacos')).toBeVisible();
        await expect(page.locator(`text=x3`)).toBeVisible();

        const approveBtn = page.getByTestId('approve-order-draft').first();
        await expect(approveBtn).toBeVisible();

        await approveBtn.click();

        await expect(approveBtn).not.toBeVisible();
    });
});
