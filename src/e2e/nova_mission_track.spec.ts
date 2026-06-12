<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('nova_mission_track', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'nova_mission_track');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('nova_mission_track');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
