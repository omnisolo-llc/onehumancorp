import { test, expect } from './fixtures';
test("dummy e2e test", async () => { expect(1).toBe(1); });
<<<<<<< HEAD
=======
  test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
