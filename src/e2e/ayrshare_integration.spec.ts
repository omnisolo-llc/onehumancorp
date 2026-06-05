import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - ayrshare_integration', () => {
  currentAppSmoke('ayrshare_integration');
});
