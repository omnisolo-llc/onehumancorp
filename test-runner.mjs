import { execSync } from 'child_process';
try {
  execSync('cd src/ui/next && npx playwright test src/e2e/staff_mesh.spec.ts --project=chromium', { stdio: 'inherit' });
} catch (e) {
  console.log("Playwright timeout or failure.");
}
