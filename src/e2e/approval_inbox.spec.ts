import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - approval_inbox', () => {
  currentAppSmoke('approval_inbox');
});
