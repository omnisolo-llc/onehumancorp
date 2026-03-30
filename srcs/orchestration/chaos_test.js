const { chromium } = require('@playwright/test');
const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

async function main() {
    // 1. Locate the Go test binary in Bazel runfiles
    const runfiles = process.env.RUNFILES_DIR || process.env.TEST_SRCDIR;
    // We look up via Bazel's MANIFEST if available, otherwise just use the expected path for main module
    const mainWorkspace = process.env.TEST_WORKSPACE || '_main';
    let binPath = path.join(runfiles, mainWorkspace, 'srcs/orchestration/orchestration_test_/orchestration_test');

    if (!fs.existsSync(binPath)) {
        binPath = path.join(runfiles, 'mono/srcs/orchestration/orchestration_test_/orchestration_test');
        if (!fs.existsSync(binPath)) {
            console.error(`ERROR: Go test binary not found. Tested ${binPath}`);
            process.exit(1);
        }
    }

    // 2. Execute the Go test to generate chaos_report.html
    console.log("Executing Go test binary to generate real chaos.db artifact and HTML report...");
    try {
        execSync(`"${binPath}" -test.v -test.run TestSIPDB_Chaos`, { stdio: 'inherit' });
    } catch (e) {
        console.error("Go test execution failed:", e);
        process.exit(1);
    }

    // Assume HTML is outputted into TEST_UNDECLARED_OUTPUTS_DIR
    const outDir = process.env.TEST_UNDECLARED_OUTPUTS_DIR || '/tmp';
    const htmlPath = path.join(outDir, 'chaos_report.html');

    if (!fs.existsSync(htmlPath)) {
        console.error(`ERROR: HTML report not found at ${htmlPath}.`);
        process.exit(1);
    }

    // 3. Launch Playwright to capture the visual verification
    console.log("Launching Playwright to verify visual grid...");
    try {
        const browser = await chromium.launch({
            headless: true,
            args: ['--no-sandbox', '--disable-setuid-sandbox']
        });
        const page = await browser.newPage();

        await page.goto(`file://${path.resolve(htmlPath)}`);

        // Ensure fonts are loaded
        await page.evaluate('document.fonts.ready');

        const screenshotPath = path.join(outDir, 'chaos_failure_report.png');
        await page.screenshot({ path: screenshotPath, fullPage: true });

        await browser.close();

        // Clean up the HTML report to ensure zero-junk compliance, leaving only the verified image.
        fs.unlinkSync(htmlPath);

        console.log(`SUCCESS: Visual report generated to ${screenshotPath}`);
    } catch (e) {
        console.error("Playwright failed:", e);
        process.exit(1);
    }
}

main().catch(e => {
    console.error(e);
    process.exit(1);
});
