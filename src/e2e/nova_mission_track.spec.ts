import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test nova_mission_track', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'nova_mission_track');
});
