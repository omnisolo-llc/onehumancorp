import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - viral_storefront', () => {
  currentAppSmoke('viral_storefront');
});
