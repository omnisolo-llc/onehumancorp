import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: twilio_omnichannel', async ({ page, request }) => { await currentAppSmoke(page, request, 'twilio_omnichannel'); });
