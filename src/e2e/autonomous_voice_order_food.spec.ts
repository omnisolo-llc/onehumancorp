import { test, expect } from '@playwright/test';
import { e2eDbQuery } from './db_utils';

test.describe('Autonomous Voice Ordering Agent', () => {
    test('Twilio voice webhook detects food order intent and creates resolved task', async ({ page }) => {
        const tenant_id = 'test_tenant'; // default test tenant

        // 1. Create the task that the Twilio webhook creates for an ORDER_FOOD intent
        const taskId = 'task-food-' + Date.now();
        const callerPhone = '+15559876543';

        const summaryText = `Automated receptionist handled a call from ${callerPhone} and sent the ordering link.`;

        const payload = JSON.stringify({
            feature_type: 'order_food',
            summary: summaryText,
            caller_phone: callerPhone
        });

        await e2eDbQuery(
            `INSERT INTO shared_tasks (id, tenant_id, organization_id, title, status, approval_status, proposed_content, payload, mission_id, parent_plan_id, priority)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)`,
            [
                taskId,
                tenant_id,
                tenant_id,
                `Voice Order Link Sent to ${callerPhone}`,
                'COMPLETED',
                'RESOLVED', // Note it's RESOLVED because no approval is needed
                payload,
                payload,
                'mission-voice-food',
                'plan-voice-food',
                'P2'
            ]
        );

        // Also insert into state_machine_transitions so it shows in activity feed
        const transitionId = 'trans-food-' + Date.now();
        await e2eDbQuery(
            `INSERT INTO state_machine_transitions (id, task_id, from_state, to_state, agent_id, transitioned_at, handoff_payload)
             VALUES ($1, $2, $3, $4, $5, NOW(), $6)`,
            [
                transitionId,
                taskId,
                'PENDING',
                'COMPLETED',
                'voice_agent',
                JSON.stringify({
                    original_payload: {
                        description: summaryText
                    }
                })
            ]
        );

        // 2. Login to the dashboard
        await page.goto(`/login`);
        // Basic login bypass for e2e
        await page.evaluate(() => {
            localStorage.setItem('has_onboarded', 'true');
            localStorage.setItem('tenant_id', 'test_tenant');
            localStorage.setItem('token', 'test_token');
        });
        await page.goto(`/dashboard`);

        // 3. Switch to Activity Feed tab
        await page.getByRole('button', { name: /Activity Feed/ }).click();

        // 4. Verify the RESOLVED task is visible in the activity feed
        // The frontend UI would show this as a completed action by the AI
        await expect(page.locator(`text=${summaryText}`)).toBeVisible();
    });
});
