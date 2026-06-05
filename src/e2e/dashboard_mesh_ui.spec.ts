import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - dashboard_mesh_ui', () => {
  currentAppSmoke('dashboard_mesh_ui');
});
