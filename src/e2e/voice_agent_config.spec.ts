import { expect, test } from './fixtures';

test.describe('Autonomous Voice AI Phone Attendant Engine', () => {

    test('Carlos activates and configures AI Voice Attendant, and it persists', async ({ page, request }) => {
        // We will test the API directly first to ensure the backend functionality is intact

        // 1. Get initial config (should be disabled by default)
        const getInitialResponse = await request.get('/api/v1/voice/config', {
            headers: {
                'Authorization': 'Bearer test-token',
                'X-Tenant-ID': 'default'
            }
        });
        expect(getInitialResponse.ok()).toBeTruthy();

        const initialData = await getInitialResponse.json();
        expect(initialData.phone_number).toBeDefined();

        // 2. Update config
        const updatePayload = {
            phone_number: initialData.phone_number,
            is_enabled: true,
            primary_language: 'Spanish',
            custom_instructions: 'Tell callers that estimates are free.'
        };

        const updateResponse = await request.post('/api/v1/voice/config', {
            headers: {
                'Authorization': 'Bearer test-token',
                'X-Tenant-ID': 'default'
            },
            data: updatePayload
        });
        expect(updateResponse.ok()).toBeTruthy();

        // 3. Verify it persisted
        const getUpdatedResponse = await request.get('/api/v1/voice/config', {
            headers: {
                'Authorization': 'Bearer test-token',
                'X-Tenant-ID': 'default'
            }
        });
        expect(getUpdatedResponse.ok()).toBeTruthy();

        const updatedData = await getUpdatedResponse.json();
        expect(updatedData.is_enabled).toBe(true);
        expect(updatedData.primary_language).toBe('Spanish');
        expect(updatedData.custom_instructions).toBe('Tell callers that estimates are free.');
    });

    test('Twilio incoming call webhook generates TwiML correctly for configured tenant', async ({ request }) => {
        // We simulate a Twilio incoming call webhook
        const formData = {
            CallSid: 'CA1234567890abcdef1234567890abcdef',
            From: '+19876543210',
            To: '(555) 123-4567',
        };

        const response = await request.post('/api/v1/webhooks/voice/incoming', {
            form: formData,
            headers: {
                'X-Twilio-Signature': 'fake-signature',
            }
        });

        expect(response.ok()).toBeTruthy();

        const responseText = await response.text();

        // We expect it to respond with a TwiML Connect Stream since it matches the default mocked tenant phone
        expect(responseText).toContain('<Response>');
        expect(responseText).toContain('<Connect>');
        expect(responseText).toContain('<Stream url="wss://localhost/api/v1/voice/stream">');
        expect(responseText).toContain('<Parameter name="tenant_id"');
        expect(responseText).toContain('<Parameter name="session_id" value="CA1234567890abcdef1234567890abcdef" />');
    });
});
