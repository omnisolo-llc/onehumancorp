import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - website_builder', () => {
  currentAppSmoke('website_builder');
});
