<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('virtual_meeting_walkthrough', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'virtual_meeting_walkthrough');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('virtual_meeting_walkthrough');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
