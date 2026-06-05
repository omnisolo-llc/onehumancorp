import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - unified_catalog', () => {
  currentAppSmoke('unified_catalog');
});
