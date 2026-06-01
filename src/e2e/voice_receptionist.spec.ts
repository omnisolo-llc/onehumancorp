import { expect, test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('business owner can view AI receptionist status and voice call logs', async ({ page, request }) => {
  // 1. Mock the backend webhook invocation
  const payload = {
      to_number: "test_business_number",
      tenant_id: "e2e",
      caller_id: "+1 (555) 123-4567",
      call_id: "test-call-1",
      status: "completed",
      transcript: "Can I get a vegan cake? Yes, we do. I will send you a link to order.",
      actions_taken: ["Checked Catalog", "Order SMS Sent"],
      detected_language: "English"
  };

  const response = await request.post('/api/agents/voice_webhook', {
      data: payload,
      headers: {
          'x-vapi-signature': 'test-sig'
      }
  });

  expect(response.status()).toBe(200);

  // 2. Owner navigates to dashboard
  await page.goto('/dashboard');

  // 3. Expect to see AI Receptionist Active status card
  await expect(page.getByRole('heading', { name: 'AI Receptionist Active' })).toBeVisible();

  // 4. Click through to Inbox
  await page.click('text=View Log');

  // 5. Verify routing to Inbox
  await expect(page.getByRole('heading', { name: 'Customer Inbox' })).toBeVisible();

  // 6. Check if the voice call is rendered with appropriate transcript and action chips
  await expect(page.getByText('+1 (555) 123-4567').first()).toBeVisible();
  await expect(page.getByText('Call summarized: Can I get a vegan cake? Yes, we do. I will send you a link to order.').first()).toBeVisible();
  await expect(page.getByText('Order SMS Sent').first()).toBeVisible();
});
