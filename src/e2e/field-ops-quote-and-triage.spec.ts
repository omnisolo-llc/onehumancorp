import { test, expect } from '@playwright/test';

test.describe('Agentic Missed-Lead Recovery & Voice-to-Quote Generation', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Voice-to-Quote Generation CUJ', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');

    // Seed an appointment
    await page.request.post(`/api/v1/field-ops/appointments`, {
      headers: { 'x-tenant-id': tenantId },
      data: {
        id: 'job-1234',
        customer_id: 'cust-1',
        customer_name: 'John Doe',
        job_template_id: 'temp-1',
        job_name: 'Plumbing Fix',
        status: 'Scheduled',
        scheduled_start_time: new Date().toISOString(),
        scheduled_end_time: new Date().toISOString(),
        location_address: '123 Main St',
        notes: ''
      }
    });

    // Navigate to field ops jobs
    await page.goto('/field-ops/jobs');
    await expect(page.locator('h1', { hasText: 'Today\'s Route' })).toBeVisible({ timeout: 15000 });

    // Open Voice Quote Modal
    await page.getByTestId('voice-quote-btn-job-1234').click();

    // Verify modal is visible
    await expect(page.getByText('Voice-to-Quote')).toBeVisible();

    // Fill transcript
    await page.getByTestId('voice-transcript-input').fill('Needs 2 hours labor for pipe repair, $50 in parts');

    // Generate quote
    await page.getByTestId('generate-quote-btn').click();

    // Verify results appear
    await expect(page.getByTestId('draft-quote-result')).toBeVisible({ timeout: 15000 });
    await expect(page.getByText('Calculated Total')).toBeVisible();

    // Approve and Send
    await page.getByRole('button', { name: 'Approve & Send' }).click();

    // Modal should close
    await expect(page.getByText('Voice-to-Quote')).not.toBeVisible();
  });

  test('Agentic Missed-Lead Recovery (Work Triage) CUJ', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Navigate to Triage Feed
    await page.goto('/triage');
    await expect(page.locator('body')).toContainText(/Work Triage/, { timeout: 15000 });

    // Click "Simulate Missed Call" button
    await page.getByTestId('simulate-missed-lead-btn').click();

    // Wait for optimistic/backend update to show the new card
    const cardContext = page.getByText('Leaky pipe under sink, can you fix?');
    await expect(cardContext).toBeVisible({ timeout: 15000 });

    // Find the drafted reply section
    await expect(page.getByText('Hi! I am currently on a job but can fix this today. Can you send a photo of the leak?')).toBeVisible();

    // Find the approve button and click
    const approveBtn = page.locator('button', { hasText: /Approve & Send/i }).first();
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Verify card disappears (Wait for processing to finish)
    await expect(cardContext).not.toBeVisible({ timeout: 10000 });
  });
});
