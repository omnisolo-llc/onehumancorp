<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('mission_track', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'mission_track');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('mission_track');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
