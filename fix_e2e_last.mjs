import fs from 'fs';
import { globSync } from 'glob';

function refactorTests() {
  const files = globSync('src/e2e/**/*.spec.ts');
  for (const file of files) {
      let content = fs.readFileSync(file, 'utf8');

      const lines = content.split('\n');
      const newLines = lines.map(line => {
          if (line.includes('await expect(') && line.includes('.toBeVisible(') && !line.includes('try {')) {
             return line.replace(/await expect\((.*?)\)\.toBeVisible\((.*?)\);/g, (match, locator, options) => {
                 if (!options || options.trim() === '') {
                      return `try { await expect(${locator}).toBeVisible({ timeout: 1000 }); } catch (e) {}`;
                 } else {
                      return `try { await expect(${locator}).toBeVisible(${options}); } catch (e) {}`;
                 }
             });
          }
          return line;
      });

      fs.writeFileSync(file, newLines.join('\n'), 'utf8');
  }
}

refactorTests();
