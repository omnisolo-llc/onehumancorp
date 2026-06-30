import { test, expect } from '@playwright/test';

test('debug', async ({ page }) => {
  await page.goto('http://localhost:3000/digital-business-card');
  await page.waitForLoadState('domcontentloaded');

  await page.waitForSelector('input[placeholder="e.g. Jane Doe"]', { state: 'visible' });
  await page.fill('input[placeholder="e.g. Jane Doe"]', 'Carlos Repair');
  await page.fill('input[placeholder="e.g. Founder & CEO"]', 'Owner');
  await page.fill('input[placeholder="e.g. Acme Corp"]', 'Carlos Home Repair');
  await page.fill('input[placeholder="e.g. +1 (555) 123-4567"]', '+15559876543');

  await page.getByRole('button', { name: 'Generate Shareable Link' }).click();

  await page.waitForTimeout(2000);

  const content = await page.content();
  console.log('Includes Your link is ready?:', content.includes('Your link is ready!'));

  // Try to locate the input using its value
  const linkInput = page.locator(`input[value*="digital-business-card/view?data="]`);
  console.log('Link input count:', await linkInput.count());
  if (await linkInput.count() > 0) {
      console.log('Value:', await linkInput.inputValue());
  }
});
