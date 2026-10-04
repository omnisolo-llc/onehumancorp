'use strict';
const path = require('node:path');
const fs = require('node:fs');
// Discovery only: use Playwright's public test identities and title paths.
module.exports = class BrowserInventoryReporter {
  onBegin(config, suite) {
    this.tests = suite.allTests().map(test => ({
      id: test.id, title: test.title,
      file: path.relative(process.cwd(), test.location.file).split(path.sep).join('/'),
      selector: `[${test.parent.project().name}] › ${path.relative(config.rootDir, test.location.file).split(path.sep).join('/')} › ${test.titlePath().slice(3).join(' › ')}`,
    }));
  }
  onEnd(result) {
    if (result.status !== 'passed' || !this.tests?.length) throw new Error('Browser discovery failed or was empty');
    const data = JSON.stringify(this.tests) + '\n';
    if (process.env.OHC_BROWSER_INVENTORY_OUTPUT) {
      // Native command stdout is a bounded diagnostic tail, not an artifact.
      fs.writeFileSync(process.env.OHC_BROWSER_INVENTORY_OUTPUT, data, { mode: 0o600, flag: 'wx' });
    } else process.stdout.write(data);
  }
};
