import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Smart Quotes & Deposit Architecture', () => {
  test('Voice command generates quote and creates Action Card', async ({ page }) => {

    await page.goto('/');

    const audioData = new Uint8Array([0, 1, 2, 3]); // dummy audio
    const boundary = '----WebKitFormBoundaryDummy';
    const body = Buffer.concat([
        Buffer.from(`--${boundary}\r\n`),
        Buffer.from('Content-Disposition: form-data; name="audio"; filename="audio.wav"\r\n'),
        Buffer.from('Content-Type: audio/wav\r\n\r\n'),
        audioData,
        Buffer.from(`\r\n--${boundary}--\r\n`)
    ]);

    const response = await page.request.post('/api/v1/voice/command', {
        headers: {
            'Content-Type': `multipart/form-data; boundary=${boundary}`,
        },
        data: body,
    });

    expect(response.status()).toBe(200);
    const json = await response.json();
    expect(json.department_assigned).toBe('Sales');
    expect(json.status).toBe('PROPOSED');

    // 2. Navigate to Agent Feed / Action Center to see the card
    await page.goto('/feed');

    // Look for the generated action card text
    await expect(page.locator('text=Drafted Quote for Voice Request')).toBeVisible();

    // Click the Approve button
    const approveButton = page.locator('button:has-text("Approve")').first();
    await approveButton.click();

    // Verify it updates or moves to completed
    await expect(page.locator('text=Drafted Quote for Voice Request')).toBeHidden({ timeout: 10000 });
  });
});
