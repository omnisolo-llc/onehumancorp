'use strict';
const path = require('node:path');
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
    process.stdout.write(JSON.stringify(this.tests) + '\n');
  }
};
