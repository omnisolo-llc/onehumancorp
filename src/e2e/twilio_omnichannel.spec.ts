<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('twilio_omnichannel', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'twilio_omnichannel');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('twilio_omnichannel');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
