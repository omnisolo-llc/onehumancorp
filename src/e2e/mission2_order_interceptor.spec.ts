import { test, expect } from './fixtures';

test.describe('Multilingual Order Interceptor E2E', () => {

  test('Persona: Fatima (Food Cart) receives a translated order interception', async ({ page, request, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    const tenantId = adminUser.organizationId;

    // Simulate an incoming Spanish message that should be intercepted as an order
    // 1. Send the message to the inbox
    await request.post('/api/agents/webhook', {
        data: {
            tenant_id: tenantId,
            source: 'whatsapp',
            sender_id: 'customer-fatima-1',
            message: 'Hola, me gustaría pedir dos tacos de pastor y una horchata para recoger a las 2pm.',
            target_language: 'English'
        }
    });

    // 2. Wait for the translation and interception agents to work
    // In a real environment, this happens via events. In E2E, we might need a small delay or poll.
    await page.goto('/dashboard.html');

    // 3. Verify the "Pending AI Negotiations" or activity feed shows the order
    // Based on Mission 2 requirements, it should show up in the owner's feed
    await expect(page.getByText(/Incoming Multilingual Order/i)).toBeVisible({ timeout: 15000 });
    await expect(page.getByText(/two pastor tacos and one horchata/i)).toBeVisible();

    // 4. Verify the translated message is shown
    await expect(page.getByText(/me gustaría pedir dos tacos/i)).toBeVisible();

    // 5. Check if a draft reply was generated
    // This would typically be in the Inbox or as part of the action card
    await page.goto('/triage.html');
    await expect(page.getByText(/Sure! I've received your order/i).or(page.getByText(/¡Claro! He recibido su pedido/i))).toBeVisible();
  });
});
