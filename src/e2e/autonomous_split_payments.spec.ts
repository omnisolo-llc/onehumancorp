import { test, expect } from '@playwright/test';
import { memberPage, adminPage } from './fixtures';

test.describe('Autonomous Split Payments & Commission Engine (E2E)', () => {
  test('A business owner can create a split payment rule for a product', async ({ page, baseURL }) => {
    // 1. Login as an admin/owner
    await adminPage(page);

    // 2. Navigate via the UI to the Split Payments page.
    await page.goto(`${baseURL}/split-payments`);
    await expect(page.getByRole('heading', { name: 'Add Partner Split' })).toBeVisible();

    // 3. Fill out the form as the business owner
    const partnerPhone = '555-0100-' + Date.now();
    await page.getByLabel('Product/Service ID').fill('test_product_xyz');
    await page.getByLabel('Partner Phone/Email').fill(partnerPhone);
    await page.getByLabel('Split Type').selectOption('percentage');
    await page.getByLabel('Split Value').fill('30');

    // 4. Submit the form
    await page.getByRole('button', { name: 'Add Partner Split' }).click();

    // 5. Verify the success message and UI update
    await expect(page.getByText('Split partner added successfully!')).toBeVisible();
    await expect(page.getByText(partnerPhone)).toBeVisible();
    await expect(page.getByText('30%')).toBeVisible();
  });

  test('A business owner can create a split payment rule with a flat amount', async ({ page, baseURL }) => {
    await adminPage(page);
    await page.goto(`${baseURL}/split-payments`);
    await expect(page.getByRole('heading', { name: 'Add Partner Split' })).toBeVisible();

    const partnerPhone = '555-0200-' + Date.now();
    await page.getByLabel('Product/Service ID').fill('test_product_abc');
    await page.getByLabel('Partner Phone/Email').fill(partnerPhone);
    await page.getByLabel('Split Type').selectOption('flat');
    await page.getByLabel('Split Value').fill('50.00');

    await page.getByRole('button', { name: 'Add Partner Split' }).click();

    await expect(page.getByText('Split partner added successfully!')).toBeVisible();
    await expect(page.getByText(partnerPhone)).toBeVisible();
    await expect(page.getByText('$50.00')).toBeVisible();
  });

  test('A business owner can view multiple existing split rules', async ({ page, baseURL }) => {
    await adminPage(page);
    await page.goto(`${baseURL}/split-payments`);

    // Create first rule
    await page.getByLabel('Product/Service ID').fill('multi_product');
    await page.getByLabel('Partner Phone/Email').fill('partner1@example.com');
    await page.getByLabel('Split Type').selectOption('percentage');
    await page.getByLabel('Split Value').fill('10');
    await page.getByRole('button', { name: 'Add Partner Split' }).click();
    await expect(page.getByText('Split partner added successfully!')).toBeVisible();

    // Create second rule
    await page.getByLabel('Product/Service ID').fill('multi_product');
    await page.getByLabel('Partner Phone/Email').fill('partner2@example.com');
    await page.getByLabel('Split Type').selectOption('percentage');
    await page.getByLabel('Split Value').fill('20');
    await page.getByRole('button', { name: 'Add Partner Split' }).click();
    await expect(page.getByText('Split partner added successfully!')).toBeVisible();

    // Verify both are listed
    await expect(page.getByText('partner1@example.com')).toBeVisible();
    await expect(page.getByText('10%')).toBeVisible();
    await expect(page.getByText('partner2@example.com')).toBeVisible();
    await expect(page.getByText('20%')).toBeVisible();
  });

  test('Adding a split rule requires a product ID', async ({ page, baseURL }) => {
    await adminPage(page);
    await page.goto(`${baseURL}/split-payments`);

    // Fill phone and value, leave product ID empty
    await page.getByLabel('Partner Phone/Email').fill('missing_product@example.com');
    await page.getByLabel('Split Type').selectOption('flat');
    await page.getByLabel('Split Value').fill('10.00');

    // HTML5 validation should prevent submission, or the UI should show an error
    // For this test, we expect the button click to not result in the success message
    await page.getByRole('button', { name: 'Add Partner Split' }).click();

    // Depending on the browser validation it might not even show an error text if the field is native 'required'
    // We just verify success message does not appear
    await expect(page.getByText('Split partner added successfully!')).not.toBeVisible();
  });

  test('Adding a split rule requires a partner phone or email', async ({ page, baseURL }) => {
    await adminPage(page);
    await page.goto(`${baseURL}/split-payments`);

    await page.getByLabel('Product/Service ID').fill('product_req');
    // leave partner phone empty
    await page.getByLabel('Split Type').selectOption('flat');
    await page.getByLabel('Split Value').fill('10.00');

    await page.getByRole('button', { name: 'Add Partner Split' }).click();
    await expect(page.getByText('Split partner added successfully!')).not.toBeVisible();
  });
});
