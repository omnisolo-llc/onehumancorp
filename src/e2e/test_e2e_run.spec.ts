import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - test_e2e_run', () => {
  currentAppSmoke('test_e2e_run');
});
