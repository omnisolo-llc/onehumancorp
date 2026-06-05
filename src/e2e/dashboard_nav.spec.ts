import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - dashboard_nav', () => {
  currentAppSmoke('dashboard_nav');
});
