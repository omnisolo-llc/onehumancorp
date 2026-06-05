import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - login_lens', () => {
  currentAppSmoke('login_lens');
});
