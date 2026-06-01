import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

// CUJ Workflow
// Persona: Priya - Boutique Owner
// Concept: Needs to embed positive customer reviews on her new storefront.
// Operating Plan: Use the new Wall of Love generator widget.
//
// 1. Logs into dashboard.
// 2. Sees the "Wall of Love Widget" section.
// 3. Clicks "Generate Widget".
// 4. Widget generates the code.
// 5. Clicks "Refresh" (or interacts with widget UI).
// 6. Copies the script and ensures it contains the expected content.

test.describe('Wall of Love Growth Loop', () => {
    test('User can generate and interact with the Wall of Love widget', async ({ memberPage }) => {
        const page = memberPage;

        // 1. Go to dashboard
        await page.goto('/dashboard');

        // 2. Find the Wall of Love section
        const section = page.locator('h2:has-text("Wall of Love Widget")');
        await expect(section).toBeVisible();

        // 3. Click Generate Widget
        await page.getByRole('button', { name: 'Generate Widget' }).click();

        // 4. Ensure textarea is populated with the embed code
        const textArea = page.locator('textarea:has-text("ohc.app/api/v1/growth/widgets/wall-of-love")');
        await expect(textArea).toBeVisible();

        // 5. Click the Refresh button
        const refreshBtn = page.getByRole('button', { name: 'Refresh' });
        await refreshBtn.click();

        // Verify button text changed to refreshing
        await expect(page.getByRole('button', { name: 'Refreshing...' })).toBeVisible();
    });
});

currentAppSmoke('viral_wall_of_love');
