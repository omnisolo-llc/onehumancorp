import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test.describe('Twilio Webhook and Omnichannel integration', () => {

    test('twilio_voice_webhook_regression_check', async ({ page, request, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
        await currentAppSmoke(page, request, 'twilio_voice_webhook_regression_check');
    });

    test('voice_attendant_regression_check', async ({ page, request, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
        await currentAppSmoke(page, request, 'voice_attendant_regression_check');
    });

    test('twilio_webhook_regression_check', async ({ page, request, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
        await currentAppSmoke(page, request, 'twilio_webhook_regression_check');
    });
});
