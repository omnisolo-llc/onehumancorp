<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('kairos_walkthrough', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'kairos_walkthrough');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('kairos_walkthrough');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
