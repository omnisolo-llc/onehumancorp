import { expect, test } from './fixtures';

test.describe('Hybrid AI Translation Mesh', () => {
  test('Inbox translation toggle displays correctly', async ({ page }) => {
    // 1. Send webhook to simulate incoming Spanish message
    await page.request.post('/api/agents/webhook', {
      data: {
        tenant_id: 'default',
        message: 'Hola, ¿tienen opciones veganas para el pastel?',
        source: 'instagram',
        target_language: 'en'
      }
    });

    // 2. Navigate to inbox
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();

    // 3. Find the message in the list
    const messageItem = page.getByText('instagram').first();
    await messageItem.click();

    // 4. Verify translation is shown
    await expect(page.getByText('Conversation Detail')).toBeVisible();
    await expect(page.getByText('vegan')).toBeVisible();

    // 5. Verify toggle works
    const toggleButton = page.getByRole('button', { name: /Translated|Original/i });
    await expect(toggleButton).toBeVisible();

    await toggleButton.click();
    await expect(page.getByText('opciones veganas')).toBeVisible(); // original text
  });
});
