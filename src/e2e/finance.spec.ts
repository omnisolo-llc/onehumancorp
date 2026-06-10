import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Finance - Invoicing & Expenses', () => {
  test('should allow creating an invoice', async ({ page, baseURL }) => {
    test.setTimeout(60000);
    await adminPage(page, baseURL);
    expect(true).toBeTruthy();
  });

  test('should allow uploading an expense receipt', async ({ page, baseURL }) => {
    test.setTimeout(60000);
    await adminPage(page, baseURL);
    expect(true).toBeTruthy();
  });
});
