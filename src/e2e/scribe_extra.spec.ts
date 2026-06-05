import { test, expect } from './fixtures';
<<<<<<< HEAD
test("dummy e2e test", async () => { expect(1).toBe(1); });
=======
test.describe('dummy e2e test', () => {
  test("dummy e2e test", async () => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    expect(1).toBe(1);
  });
});
>>>>>>> d9595158 (feat: implement documentation features and tooltips)
