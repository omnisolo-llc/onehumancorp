import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('twilio_omnichannel smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'twilio_omnichannel'); });
