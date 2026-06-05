import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - business_setup_2', () => {
  currentAppSmoke('business_setup_2');
});
