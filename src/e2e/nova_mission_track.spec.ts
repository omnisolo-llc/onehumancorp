import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('nova_mission_track', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'nova_mission_track');
});
