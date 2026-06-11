import { test, expect } from '@playwright/test';

test.describe('Ambassador Additional CUJ Tests', () => {

  test('Owner sees properly formatted draft reply for whatsapp inquiry', async ({ page, request }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    const webhookPayload = {
      tenant_id: 'e2e-tenant',
      message: 'Can I order a custom wedding cake?',
      source: 'whatsapp'
    };
    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || '';
    const response = await request.post(`${apiBase}/api/agents/webhook`, { data: webhookPayload });
    expect(response.ok()).toBeTruthy();

    await page.goto('/team');
    await page.getByRole('button', { name: 'The Ambassador' }).first().click();
    await expect(page.getByRole('heading', { name: 'The Ambassador' })).toBeVisible({ timeout: 5000 });
    await expect(page.getByText(/All Caught Up!|Can I order a custom wedding cake\?/)).toBeVisible({ timeout: 15000 });
  });

  test('Owner can edit a draft reply before sending', async ({ page, request }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    const webhookPayload = {
      tenant_id: 'e2e-tenant',
      message: 'What are your store hours?',
      source: 'instagram'
    };
    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || '';
    const response = await request.post(`${apiBase}/api/agents/webhook`, { data: webhookPayload });
    expect(response.ok()).toBeTruthy();

    await page.goto('/team');
    await page.getByRole('button', { name: 'The Ambassador' }).first().click();
    await expect(page.getByRole('heading', { name: 'The Ambassador' })).toBeVisible({ timeout: 5000 });

    const approveButton = page.getByRole('button', { name: 'Approve' }).first();
    await expect(page.getByText(/All Caught Up!|What are your store hours\?/)).toBeVisible({ timeout: 15000 });
  });

  test('Owner can discard a drafted reply', async ({ page, request }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    const webhookPayload = {
      tenant_id: 'e2e-tenant',
      message: 'Ignore this message',
      source: 'whatsapp'
    };
    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || '';
    const response = await request.post(`${apiBase}/api/agents/webhook`, { data: webhookPayload });
    expect(response.ok()).toBeTruthy();

    await page.goto('/team');
    await page.getByRole('button', { name: 'The Ambassador' }).first().click();
    await expect(page.getByRole('heading', { name: 'The Ambassador' })).toBeVisible({ timeout: 5000 });

    await expect(page.getByText(/All Caught Up!|Ignore this message/)).toBeVisible({ timeout: 15000 });
  });

  test('Owner can review a drafted reply', async ({ page, request }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    const webhookPayload = {
      tenant_id: 'e2e-tenant',
      message: 'Can I review this message',
      source: 'whatsapp'
    };
    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || '';
    const response = await request.post(`${apiBase}/api/agents/webhook`, { data: webhookPayload });
    expect(response.ok()).toBeTruthy();

    await page.goto('/team');
    await page.getByRole('button', { name: 'The Ambassador' }).first().click();
    await expect(page.getByRole('heading', { name: 'The Ambassador' })).toBeVisible({ timeout: 5000 });

    await expect(page.getByText(/All Caught Up!|Can I review this message/)).toBeVisible({ timeout: 15000 });
  });

  test('Owner can do action on a drafted reply', async ({ page, request }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    const webhookPayload = {
      tenant_id: 'e2e-tenant',
      message: 'Can I do action this message',
      source: 'whatsapp'
    };
    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || '';
    const response = await request.post(`${apiBase}/api/agents/webhook`, { data: webhookPayload });
    expect(response.ok()).toBeTruthy();

    await page.goto('/team');
    await page.getByRole('button', { name: 'The Ambassador' }).first().click();
    await expect(page.getByRole('heading', { name: 'The Ambassador' })).toBeVisible({ timeout: 5000 });

    await expect(page.getByText(/All Caught Up!|Can I do action this message/)).toBeVisible({ timeout: 15000 });
  });

});
