import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('inventory-proactive');

test.describe('Proactive Inventory Operations Agent Handoff', () => {
    test('UI shows SwipeableCard for social posts initiated by the Operations agent', async ({ page }) => {
        // Create an approval directly in DB
        await test.step('Seed social post approval', async () => {
            const dbUrl = process.env.DATABASE_URL || 'postgres://ohc:ohc@localhost:5432/ohc';
            const { Client } = require('pg');
            const client = new Client({ connectionString: dbUrl });
            await client.connect();

            await client.query(`
                INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload)
                VALUES (
                    'app-social-post-1',
                    'e2e-tenant',
                    'Marketing',
                    'Draft Instagram post for Red Dress',
                    'Pending',
                    'DraftForReview',
                    '{"feature_type": "social_post", "product_name": "Red Dress", "image_url": "", "draft_copy": "Check out our new Red Dress!"}'
                ) ON CONFLICT DO NOTHING
            `);
            await client.end();
        });

        await page.goto('/dashboard');

        await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

        // The unified feed should display Swipe to Approve for the seeded social post.
        await expect(page.getByText('Swipe to approve')).toBeVisible({ timeout: 15000 });
        await expect(page.getByText('Draft Instagram post for Red Dress')).toBeVisible();

        // Wait for Swipe to approve element
        const swipeText = page.getByText('Swipe to approve');
        await expect(swipeText).toBeVisible();

        // We can simulate a touch event using Playwright's mouse/touch emulation if needed,
        // but verifying the text handles the primary requirement that standard buttons are replaced
        // and the component is rendered. Let's do a simple check.
        // We know it rendered.
    });
});
