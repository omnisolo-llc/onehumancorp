import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('virtual_meeting_walkthrough', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'virtual_meeting_walkthrough');
});
