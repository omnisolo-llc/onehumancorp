import { test, expect } from './fixtures';

test.describe('Settings Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/settings');
    await expect(page.locator('#settings-screen')).toBeVisible();

  test('toggles and provisions AI Voice Receptionist', async ({ page, request }) => {
    await expect(page.getByText('Enable AI Voice Receptionist')).toBeVisible();

    await page.getByLabel('Enable AI Voice Receptionist').check();
    await expect(page.getByLabel('Enable AI Voice Receptionist')).toBeChecked();

    await expect(page.getByText('Voice Persona')).toBeVisible();
    await expect(page.getByText('Assigned Number')).toBeVisible();

    await page.getByRole('button', { name: 'Get Number' }).click();

    await expect(page.getByRole('textbox', { name: 'Assigned Phone Number' })).not.toHaveValue('');

    const voiceRes = await request.post('/api/v1/webhooks/twilio/voice', { data: 'To=%2B15551234567&From=%2B19998887777', headers: { 'Content-Type': 'application/x-www-form-urlencoded' } });
    expect(voiceRes.ok()).toBeTruthy();
    const text = await voiceRes.text();
    expect(text).toContain('<Gather input="speech" action="/api/v1/webhooks/twilio/voice/gather"');

    const gatherRes = await request.post('/api/v1/webhooks/twilio/voice/gather', { data: 'SpeechResult=What%20are%20your%20hours%3F', headers: { 'Content-Type': 'application/x-www-form-urlencoded' } });
    expect(gatherRes.ok()).toBeTruthy();
    expect(await gatherRes.text()).toContain('<Record action="/api/v1/webhooks/twilio/voice/record"');

    const recordRes = await request.post('/api/v1/webhooks/twilio/voice/record', { data: 'RecordingUrl=http%3A%2F%2Ffoo.com%2Fbar.mp3&TranscriptionText=Hello%20world', headers: { 'Content-Type': 'application/x-www-form-urlencoded' } });
    expect(recordRes.ok()).toBeTruthy();
  });
});

  test('shows general notification settings', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Settings' })).toBeVisible();
    await expect(page.getByText('Enable Email Notifications')).toBeVisible();
    await expect(page.getByText('Enable Push Notifications')).toBeVisible();
    await expect(page.getByText('Timezone')).toBeVisible();
    await expect(page.getByText('Language', { exact: true })).toBeVisible();

  test('toggles and provisions AI Voice Receptionist', async ({ page, request }) => {
    await expect(page.getByText('Enable AI Voice Receptionist')).toBeVisible();

    await page.getByLabel('Enable AI Voice Receptionist').check();
    await expect(page.getByLabel('Enable AI Voice Receptionist')).toBeChecked();

    await expect(page.getByText('Voice Persona')).toBeVisible();
    await expect(page.getByText('Assigned Number')).toBeVisible();

    await page.getByRole('button', { name: 'Get Number' }).click();

    await expect(page.getByRole('textbox', { name: 'Assigned Phone Number' })).not.toHaveValue('');

    const voiceRes = await request.post('/api/v1/webhooks/twilio/voice', { data: 'To=%2B15551234567&From=%2B19998887777', headers: { 'Content-Type': 'application/x-www-form-urlencoded' } });
    expect(voiceRes.ok()).toBeTruthy();
    const text = await voiceRes.text();
    expect(text).toContain('<Gather input="speech" action="/api/v1/webhooks/twilio/voice/gather"');

    const gatherRes = await request.post('/api/v1/webhooks/twilio/voice/gather', { data: 'SpeechResult=What%20are%20your%20hours%3F', headers: { 'Content-Type': 'application/x-www-form-urlencoded' } });
    expect(gatherRes.ok()).toBeTruthy();
    expect(await gatherRes.text()).toContain('<Record action="/api/v1/webhooks/twilio/voice/record"');

    const recordRes = await request.post('/api/v1/webhooks/twilio/voice/record', { data: 'RecordingUrl=http%3A%2F%2Ffoo.com%2Fbar.mp3&TranscriptionText=Hello%20world', headers: { 'Content-Type': 'application/x-www-form-urlencoded' } });
    expect(recordRes.ok()).toBeTruthy();
  });
});

  test('toggles delivery settings', async ({ page }) => {
    await page.getByLabel('Enable Email Notifications').check();
    await page.getByLabel('Enable Push Notifications').check();
    await page.getByLabel('Enable Local Delivery').check();

    await expect(page.getByLabel('Enable Email Notifications')).toBeChecked();
    await expect(page.getByLabel('Enable Push Notifications')).toBeChecked();
    await expect(page.getByLabel('Enable Local Delivery')).toBeChecked();
    await expect(page.getByLabel('Delivery Radius (miles)')).toBeEnabled();
    await expect(page.getByLabel('Flat Delivery Fee ($)')).toBeEnabled();

  test('toggles and provisions AI Voice Receptionist', async ({ page, request }) => {
    await expect(page.getByText('Enable AI Voice Receptionist')).toBeVisible();

    await page.getByLabel('Enable AI Voice Receptionist').check();
    await expect(page.getByLabel('Enable AI Voice Receptionist')).toBeChecked();

    await expect(page.getByText('Voice Persona')).toBeVisible();
    await expect(page.getByText('Assigned Number')).toBeVisible();

    await page.getByRole('button', { name: 'Get Number' }).click();

    await expect(page.getByRole('textbox', { name: 'Assigned Phone Number' })).not.toHaveValue('');

    const voiceRes = await request.post('/api/v1/webhooks/twilio/voice', { data: 'To=%2B15551234567&From=%2B19998887777', headers: { 'Content-Type': 'application/x-www-form-urlencoded' } });
    expect(voiceRes.ok()).toBeTruthy();
    const text = await voiceRes.text();
    expect(text).toContain('<Gather input="speech" action="/api/v1/webhooks/twilio/voice/gather"');

    const gatherRes = await request.post('/api/v1/webhooks/twilio/voice/gather', { data: 'SpeechResult=What%20are%20your%20hours%3F', headers: { 'Content-Type': 'application/x-www-form-urlencoded' } });
    expect(gatherRes.ok()).toBeTruthy();
    expect(await gatherRes.text()).toContain('<Record action="/api/v1/webhooks/twilio/voice/record"');

    const recordRes = await request.post('/api/v1/webhooks/twilio/voice/record', { data: 'RecordingUrl=http%3A%2F%2Ffoo.com%2Fbar.mp3&TranscriptionText=Hello%20world', headers: { 'Content-Type': 'application/x-www-form-urlencoded' } });
    expect(recordRes.ok()).toBeTruthy();
  });
});

  test('shows SMS alert and delivery settings fields', async ({ page }) => {
    await expect(page.getByText('Critical SMS Alerts')).toBeVisible();
    await expect(page.getByPlaceholder('Mobile Phone Number (e.g. +1234567890)')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Verify Number' })).toBeVisible();
    await expect(page.getByText('Urgent Bookings')).toBeVisible();
    await expect(page.getByText('Failed Payments')).toBeVisible();
    await expect(page.getByText('New Orders')).toBeVisible();
    await expect(page.getByText('Local Delivery (DoorDash Drive)')).toBeVisible();
    await expect(page.getByText('Enable Local Delivery')).toBeVisible();
    await expect(page.getByText('Delivery Radius (miles)')).toBeVisible();
    await expect(page.getByText('Flat Delivery Fee ($)')).toBeVisible();

  test('toggles and provisions AI Voice Receptionist', async ({ page, request }) => {
    await expect(page.getByText('Enable AI Voice Receptionist')).toBeVisible();

    await page.getByLabel('Enable AI Voice Receptionist').check();
    await expect(page.getByLabel('Enable AI Voice Receptionist')).toBeChecked();

    await expect(page.getByText('Voice Persona')).toBeVisible();
    await expect(page.getByText('Assigned Number')).toBeVisible();

    await page.getByRole('button', { name: 'Get Number' }).click();

    await expect(page.getByRole('textbox', { name: 'Assigned Phone Number' })).not.toHaveValue('');

    const voiceRes = await request.post('/api/v1/webhooks/twilio/voice', { data: 'To=%2B15551234567&From=%2B19998887777', headers: { 'Content-Type': 'application/x-www-form-urlencoded' } });
    expect(voiceRes.ok()).toBeTruthy();
    const text = await voiceRes.text();
    expect(text).toContain('<Gather input="speech" action="/api/v1/webhooks/twilio/voice/gather"');

    const gatherRes = await request.post('/api/v1/webhooks/twilio/voice/gather', { data: 'SpeechResult=What%20are%20your%20hours%3F', headers: { 'Content-Type': 'application/x-www-form-urlencoded' } });
    expect(gatherRes.ok()).toBeTruthy();
    expect(await gatherRes.text()).toContain('<Record action="/api/v1/webhooks/twilio/voice/record"');

    const recordRes = await request.post('/api/v1/webhooks/twilio/voice/record', { data: 'RecordingUrl=http%3A%2F%2Ffoo.com%2Fbar.mp3&TranscriptionText=Hello%20world', headers: { 'Content-Type': 'application/x-www-form-urlencoded' } });
    expect(recordRes.ok()).toBeTruthy();
  });
});

  test('toggles and provisions AI Voice Receptionist', async ({ page, request }) => {
    await expect(page.getByText('Enable AI Voice Receptionist')).toBeVisible();

    await page.getByLabel('Enable AI Voice Receptionist').check();
    await expect(page.getByLabel('Enable AI Voice Receptionist')).toBeChecked();

    await expect(page.getByText('Voice Persona')).toBeVisible();
    await expect(page.getByText('Assigned Number')).toBeVisible();

    await page.getByRole('button', { name: 'Get Number' }).click();

    await expect(page.getByRole('textbox', { name: 'Assigned Phone Number' })).not.toHaveValue('');

    const voiceRes = await request.post('/api/v1/webhooks/twilio/voice', { data: 'To=%2B15551234567&From=%2B19998887777', headers: { 'Content-Type': 'application/x-www-form-urlencoded' } });
    expect(voiceRes.ok()).toBeTruthy();
    const text = await voiceRes.text();
    expect(text).toContain('<Gather input="speech" action="/api/v1/webhooks/twilio/voice/gather"');

    const gatherRes = await request.post('/api/v1/webhooks/twilio/voice/gather', { data: 'SpeechResult=What%20are%20your%20hours%3F', headers: { 'Content-Type': 'application/x-www-form-urlencoded' } });
    expect(gatherRes.ok()).toBeTruthy();
    expect(await gatherRes.text()).toContain('<Record action="/api/v1/webhooks/twilio/voice/record"');

    const recordRes = await request.post('/api/v1/webhooks/twilio/voice/record', { data: 'RecordingUrl=http%3A%2F%2Ffoo.com%2Fbar.mp3&TranscriptionText=Hello%20world', headers: { 'Content-Type': 'application/x-www-form-urlencoded' } });
    expect(recordRes.ok()).toBeTruthy();
  });
});
