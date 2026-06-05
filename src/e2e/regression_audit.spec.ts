import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - regression_audit', () => {
  currentAppSmoke('regression_audit');
});
