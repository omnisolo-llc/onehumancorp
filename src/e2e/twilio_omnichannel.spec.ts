import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test twilio_omnichannel', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'twilio_omnichannel');
});
