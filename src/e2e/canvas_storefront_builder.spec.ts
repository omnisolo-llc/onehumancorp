import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - canvas_storefront_builder', () => {
  currentAppSmoke('canvas_storefront_builder');
});
