import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - agent_audit_dashboard', () => {
  currentAppSmoke('agent_audit_dashboard');
});
