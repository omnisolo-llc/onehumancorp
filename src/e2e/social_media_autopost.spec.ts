import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test social_media_autopost', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'social_media_autopost');
});
