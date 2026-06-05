import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - ux_friction_audit', () => {
  currentAppSmoke('ux_friction_audit');
});
