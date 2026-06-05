import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - free_tier', () => {
  currentAppSmoke('free_tier');
});
