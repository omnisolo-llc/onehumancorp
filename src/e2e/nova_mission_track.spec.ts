import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - nova_mission_track', () => {
  currentAppSmoke('nova_mission_track');
});
