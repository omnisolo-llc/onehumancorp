import { test, expect } from '@playwright/test';

test('business setup onboarding flow', async ({ page }) => {
  // 1. Start from login
  await page.goto('/');
  await expect(page.getByText('One Human Corp')).toBeVisible();

  await page.getByPlaceholder('Email or Username').fill('test@example.com');
  await page.getByPlaceholder('Password').fill('password123');
  await page.getByText('Sign In').click();

  // 2. Go to business setup
  await expect(page.getByText('Launch Setup Wizard (New User)')).toBeVisible();
  await page.getByText('Launch Setup Wizard (New User)').click();

  // Step 0: Welcome
  await expect(page.getByText('Your business,')).toBeVisible();
  await page.getByText('Get Started →').click();

  // Step 1: Business type
  await expect(page.getByText('What kind of business are you building?')).toBeVisible();
  await page.getByText('🛒 Online Store').click();

  // Step 2: Company name
  await expect(page.getByText('Give your business a name')).toBeVisible();
  await page.getByPlaceholder('e.g. Maya\'s Cakes').fill('Test Company');
  await page.getByText('Next →').click();

  // Step 3: What do you sell
  await expect(page.getByText('What do you sell?')).toBeVisible();
  await page.getByText('📦 Physical products').click();
  await page.getByText('Next →').click();

  // Step 4: Payments
  await expect(page.getByText('How do you want to receive payments?')).toBeVisible();
  await page.getByText('🌐 Online only').click();

  // Step 5: Admin Account
  await expect(page.getByText('Create your account')).toBeVisible();
  await page.getByPlaceholder('e.g. Maya Smith').fill('Admin User');
  await page.getByPlaceholder('you@email.com').fill('admin@testcompany.com');
  await page.getByPlaceholder('Password').fill('securepassword123');
  await page.getByText('Next →').click();

  // Step 6: Choose a Template
  await expect(page.getByText('Choose a Template')).toBeVisible();
  await page.getByText('✨ Modern').click();

  // Step 7: Add your first product
  await expect(page.getByText('Add your first product or service')).toBeVisible();
  await page.getByPlaceholder('e.g. Custom Birthday Cake').fill('Test Product');
  await page.getByPlaceholder('e.g. 50.00').fill('50.00');
  await page.getByText('Next →').click();

  // Step 8: Choose a Domain
  await expect(page.getByText('Choose a Domain')).toBeVisible();
  await page.getByText('🌐 Free OHC Domain').click();

  // Step 9: Launch
  await expect(page.getByText('Ready to launch!')).toBeVisible();
  await page.getByText('Launch My Business →').click();

  // Final check
  await expect(page.getByText('Go to Dashboard →')).toBeVisible();
});
