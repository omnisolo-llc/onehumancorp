import { Reporter, TestCase, TestResult, FullResult } from '@playwright/test/reporter';
import fs from 'fs';
import path from 'path';

class OHCVisualReporter implements Reporter {
  private results: { title: string; status: string; duration: number; error?: string }[] = [];

  onTestEnd(test: TestCase, result: TestResult) {
    this.results.push({
      title: test.title,
      status: result.status,
      duration: result.duration,
      error: result.error?.message,
    });
  }

  onEnd(result: FullResult) {
    const isSuccess = result.status === 'passed';

    let gridHtml = this.results.map(r => {
      const isPass = r.status === 'passed';
      const color = isPass ? 'rgba(0, 255, 128, 0.2)' : 'rgba(255, 64, 64, 0.2)';
      const border = isPass ? 'rgba(0, 255, 128, 0.5)' : 'rgba(255, 64, 64, 0.5)';
      const icon = isPass ? '✅' : '❌';

      return `
        <div style="
          background: ${color};
          border: 1px solid ${border};
          border-radius: 12px;
          padding: 16px;
          margin-bottom: 12px;
          backdrop-filter: blur(20px) saturate(200%);
          -webkit-backdrop-filter: blur(20px) saturate(200%);
        ">
          <div style="display: flex; justify-content: space-between; align-items: center;">
            <strong style="font-size: 1.1em; color: white;">${icon} ${r.title}</strong>
            <span style="color: rgba(255,255,255,0.7); font-size: 0.9em;">${r.duration}ms</span>
          </div>
          ${r.error ? `<pre style="color: #ff8888; font-size: 0.85em; margin-top: 10px; background: rgba(0,0,0,0.3); padding: 8px; border-radius: 6px; overflow-x: auto;">${r.error}</pre>` : ''}
        </div>
      `;
    }).join('\n');

    const html = `
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <title>OHC Swarm Verification</title>
      <style>
        @import url('https://fonts.googleapis.com/css2?family=Outfit:wght@300;400;600&display=swap');
        body {
          font-family: 'Outfit', sans-serif;
          background: linear-gradient(135deg, #0f172a 0%, #1e1b4b 100%);
          color: white;
          margin: 0;
          padding: 40px;
          min-height: 100vh;
        }
        .container {
          max-width: 800px;
          margin: 0 auto;
          background: rgba(255, 255, 255, 0.05);
          backdrop-filter: blur(20px) saturate(200%);
          -webkit-backdrop-filter: blur(20px) saturate(200%);
          border: 1px solid rgba(255, 255, 255, 0.1);
          border-radius: 24px;
          padding: 32px;
          box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
        }
        h1 {
          font-weight: 600;
          margin-top: 0;
          border-bottom: 1px solid rgba(255,255,255,0.1);
          padding-bottom: 16px;
        }
        .summary {
          display: flex;
          justify-content: space-between;
          margin-bottom: 30px;
          font-size: 1.2em;
        }
        .status-passed { color: #4ade80; }
        .status-failed { color: #f87171; }
      </style>
    </head>
    <body>
      <div class="container">
        <h1>🛡️ OHC Swarm E2E Verification</h1>
        <div class="summary">
          <span>Status: <strong class="${isSuccess ? 'status-passed' : 'status-failed'}">${result.status.toUpperCase()}</strong></span>
          <span>Total Tests: ${this.results.length}</span>
        </div>
        <div class="grid">
          ${gridHtml}
        </div>
      </div>
    </body>
    </html>
    `;

    const reportDir = path.join(process.cwd(), 'playwright-report');
    if (!fs.existsSync(reportDir)) {
      fs.mkdirSync(reportDir, { recursive: true });
    }
    fs.writeFileSync(path.join(reportDir, 'index.html'), html);
    console.log('Visual report generated at playwright-report/index.html');
  }
}

export default OHCVisualReporter;
