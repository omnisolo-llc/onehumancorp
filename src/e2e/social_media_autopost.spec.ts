import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - social_media_autopost', () => {
  currentAppSmoke('social_media_autopost');
});
