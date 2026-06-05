import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - echo_ux_friction', () => {
  currentAppSmoke('echo_ux_friction');
});
