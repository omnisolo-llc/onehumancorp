import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('social_media_autopost', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'social_media_autopost');
});
