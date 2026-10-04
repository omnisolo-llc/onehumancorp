'use strict';
const fs = require('node:fs');
const path = require('node:path');
const { PROTOCOL, ATTACHMENT, assertSource, writeReceipt } = require('./ui-click-audit.cjs');
class ClickCoverageReporter {
  onBegin(config, suite) {
    this.root = process.env.OHC_CLICK_AUDIT_ROOT || path.resolve(__dirname, '..');
    this.context = JSON.parse(process.env.OHC_CLICK_AUDIT_CONTEXT || 'null');
    this.sourceSnapshot=assertSource(this.root, this.context);
    const groupedInventory = process.env.OHC_BROWSER_GROUP_INVENTORY
      ? JSON.parse(fs.readFileSync(process.env.OHC_BROWSER_GROUP_INVENTORY, 'utf8')) : undefined;
    const shard = groupedInventory
      ? require('./browser-shards.cjs').validateProof(groupedInventory, this.context, suite.allTests().map(test => this.identity(test)))
      : config.shard || { current: 1, total: 1 };
    this.file = path.join(process.env.OHC_CLICK_AUDIT_DIRECTORY || path.join(this.root, 'test-results/click-receipts'), `shard-${shard.current}.json`);
    if (fs.existsSync(this.file) || fs.existsSync(`${this.file}.pending`)) throw new Error('Click coverage receipt already exists; preserve it and use a fresh run directory');
    this.receipt = { protocol: PROTOCOL, context: this.context, shard: { index: shard.current, total: shard.total },
      ...(groupedInventory ? { groupedInventory } : {}),
      selection: suite.allTests().map(test => this.identity(test)), tests: [], complete: false, runStatus: 'running' };
    this.flush();
  }
  identity(test) { return { id: test.id, title: test.title, file: path.relative(this.root, test.location.file).split(path.sep).join('/') }; }
  onTestEnd(test, result) {
    if (!this.receipt) return;
    try {
      const attachments = result.attachments.filter(item => item.name === ATTACHMENT).map(item => {
        if (item.contentType !== 'application/json' || !item.body || item.body.length > 8 * 1024 * 1024) throw new Error('Missing or invalid coverage attachment body');
        return JSON.parse(item.body.toString('utf8'));
      });
      this.receipt.tests.push({ ...this.identity(test), status: result.status, expectedStatus: test.expectedStatus, retry: result.retry, attachments });
      this.flush();
    } catch (error) { this.problem = error; }
  }
  onEnd(result) {
    try {
      if (!this.receipt) throw new Error('Click coverage reporter never began');
      if (this.problem) throw this.problem;
      assertSource(this.root, this.context, this.sourceSnapshot);
      this.receipt.complete = true;
      this.receipt.runStatus = result.status;
      this.flush();
    } catch (error) {
      if (this.receipt) {
        this.receipt.complete=false;
        this.receipt.runStatus='failed';
        this.receipt.reporterError=String(error.message).slice(0,1000);
        if (error.sourceDiagnostics) this.receipt.sourceDiagnostics=error.sourceDiagnostics;
        try {this.flush();} catch(writeError) {console.error(`Click coverage diagnostics could not be retained: ${writeError.message}`);}
      }
      console.error(`Click coverage reporter failed: ${error.message}`);
      return { status: 'failed' };
    }
  }
  flush() { writeReceipt(this.file, this.receipt); }
}
module.exports = ClickCoverageReporter;
