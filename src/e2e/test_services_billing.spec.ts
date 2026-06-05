import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - test_services_billing', () => {
  currentAppSmoke('test_services_billing');
});
