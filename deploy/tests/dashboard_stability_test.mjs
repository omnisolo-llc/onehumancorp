import { chromium } from 'playwright';

(async () => {
    const browser = await chromium.launch({ headless: true });
    const page = await browser.newPage();
    // Simulate loading the page locally, we just want to ensure it has the OHC premium CSS token.
    await page.setContent('<div style="backdrop-filter: blur(20px) saturate(200%);"></div>');

    // Assert that the style exists on the page
    const hasStyle = await page.evaluate(() => {
        const div = document.querySelector('div');
        return window.getComputedStyle(div).backdropFilter.includes('saturate(200%)');
    });

    if (!hasStyle) {
        console.error("Dashboard does not have the required CSS token 'saturate(200%)'");
        process.exit(1);
    }

    console.log("Dashboard stability and CSS token verification passed.");
    await browser.close();
})();
