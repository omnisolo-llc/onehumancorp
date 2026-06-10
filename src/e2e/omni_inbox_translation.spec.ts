import { test, expect } from '@playwright/test';
import { supabase } from '../ui/next/src/lib/supabaseClient';

test.describe('Omni Inbox Translation E2E', () => {
  test('Owner configures translation and views translated message in inbox', async ({ page, request }) => {
    // 1. Log in as an owner (we assume the standard local login flow via /auth/login or a pre-seeded test user)
    // For local e2e, we login with standard mock credentials or a test fixture
    await page.goto('http://localhost:3000/auth/login');

    // Fallback logic if unauthenticated
    const loginEmailInput = page.locator('input[name="email"]');
    if (await loginEmailInput.isVisible()) {
      await loginEmailInput.fill('test-owner@example.com');
      await page.locator('input[name="password"]').fill('password123');
      await page.locator('button:has-text("Log in")').click();
    }

    // 2. Navigate to Settings and configure the "Global Audience" translation settings
    await page.goto('http://localhost:3000/settings');
    await expect(page.locator('h1')).toContainText('Settings');

    // Turn on auto-translate and set target language
    const autoTranslateToggle = page.locator('input[name="auto_translate"]');
    await autoTranslateToggle.check();

    const targetLanguagesInput = page.locator('input[name="target_languages"]');
    await targetLanguagesInput.fill('es, fr');

    await page.locator('button:has-text("Save Settings")').click();

    // 3. Dispatch a mock webhook request to the backend Meta webhook endpoint
    const webhookPayload = {
      object: "page",
      entry: [{
        id: "mock_page_id",
        time: Date.now(),
        messaging: [{
          sender: { id: "mock_customer_id" },
          recipient: { id: "mock_page_id" },
          timestamp: Date.now(),
          message: {
            mid: "mock_mid",
            text: "Hola, me gustaría pedir un pastel para el viernes."
          }
        }]
      }]
    };

    const webhookResponse = await request.post('http://localhost:8000/api/meta/webhook', {
      data: webhookPayload,
    });

    // The webhook might return 200 OK immediately and process async
    expect(webhookResponse.ok()).toBeTruthy();

    // 4. Navigate to the Inbox and verify the translated message is visible
    await page.goto('http://localhost:3000/inbox');
    await expect(page.locator('h1')).toContainText('Inbox');

    // Wait for the message to appear (polling or waiting for specific text)
    // Since the message was "Hola...", we expect the LLM to translate it to English "Hello, I would like to order a cake for Friday."
    await page.waitForSelector('text=Hello, I would like to order a cake for Friday.');
    const translatedMessage = page.locator('text=Hello, I would like to order a cake for Friday.');
    await expect(translatedMessage).toBeVisible();

    // Optional: Check for the translation badge
    const badge = page.locator('text=Translated from Spanish');
    await expect(badge).toBeVisible();
  });
});
