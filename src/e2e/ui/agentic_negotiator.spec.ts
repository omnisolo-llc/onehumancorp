import { test, expect } from '@playwright/test';
import * as crypto from 'crypto';

test.describe('Agentic Negotiator & Booker CUJ', () => {

  test.beforeEach(async ({ page }) => {
    // Generate a valid tenant identity via the standard authentication route
    // to avoid IDOR vulnerabilities in the backend.
    const tenantId = `tenant-${crypto.randomBytes(4).toString('hex')}`;
    const agentId = `owner-${crypto.randomBytes(4).toString('hex')}@test.com`;

    // Go to login to obtain proper session/token auth context per repo guidelines
    await page.goto('/login');
    await page.getByPlaceholder('Email address').fill(agentId);
    await page.getByPlaceholder('Password').fill('Password123!');
    await page.getByRole('button', { name: 'Sign in' }).click();

    await expect(page).toHaveURL('/dashboard');
  });

  test('Owner can toggle Agentic Negotiator and see intercepted leads', async ({ page }) => {
    // Navigate to Inbox as a real owner
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();

    // In the real app, the owner reviews the AI's drafts/quotes before sending or toggles it to auto-send
    await page.getByRole('button', { name: 'AI Settings' }).click();

    // Toggle the negotiator on
    const negotiatorToggle = page.getByRole('switch', { name: 'Agentic Negotiator' });
    if (!(await negotiatorToggle.isChecked())) {
        await negotiatorToggle.click();
    }

    // Ensure the settings are saved
    await expect(page.getByText('Settings saved')).toBeVisible();

    // Verify the AI has placed a draft quote on an inbound message
    // (Assuming a webhook or background process seeded a message)
    await page.goto('/inbox');
    const firstThread = page.locator('.inbox-thread').first();
    await firstThread.click();

    // Expect to see the AI's generated response proposing a booking and quote
    await expect(page.getByText('AI Draft: I can help with that!')).toBeVisible({ timeout: 5000 });
    await expect(page.getByRole('button', { name: 'Approve & Send Quote' })).toBeVisible();
  });
});
