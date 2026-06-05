import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - team_chat', () => {
  currentAppSmoke('team_chat');
});
