import asyncio
from playwright.async_api import async_playwright
import os

async def run():
    async with async_playwright() as p:
        browser = await p.chromium.launch(headless=True)
        page = await browser.new_page()

        url = "http://127.0.0.1:8080"
        print(f"Navigating to {url}")
        await page.goto(url)
        await page.wait_for_timeout(2000)

        os.makedirs("docs/research/ux/screenshots/2026-03-30", exist_ok=True)
        os.makedirs("docs/research/ux/screenshots/2026-04-01", exist_ok=True)

        print("Taking login screenshot")
        await page.screenshot(path="docs/research/ux/screenshots/2026-04-01/01_login.png", full_page=True)

        print("Logging in")
        # Flutter web apps often don't have standard HTML elements like input when rendered with CanvasKit
        # Using exact coordinates to interact

        # Click on the email input field (approx coordinates)
        await page.mouse.click(100, 300)
        await page.keyboard.type("test@test.com")

        # Click on the password field
        await page.mouse.click(100, 400)
        await page.keyboard.type("password")

        # Click on the Sign In button
        await page.mouse.click(200, 500)

        await page.wait_for_timeout(2000)

        print("Taking dashboard screenshot")
        await page.screenshot(path="docs/research/ux/screenshots/2026-04-01/02_dashboard.png", full_page=True)

        print("Navigating to wizard")
        await page.goto(f"{url}/wizard")
        await page.wait_for_timeout(1000)
        await page.screenshot(path="docs/research/ux/screenshots/2026-04-01/03_wizard_1.png", full_page=True)

        print("Next step")
        # await page.click("button:has-text('Next')")
        await page.wait_for_timeout(500)
        await page.screenshot(path="docs/research/ux/screenshots/2026-04-01/04_wizard_2.png", full_page=True)

        print("Next step")
        # await page.click("button:has-text('Next')")
        await page.wait_for_timeout(500)
        await page.screenshot(path="docs/research/ux/screenshots/2026-04-01/05_wizard_3.png", full_page=True)

        print("Navigating to AI config")
        await page.goto(f"{url}/ai-config")
        await page.wait_for_timeout(1000)
        await page.screenshot(path="docs/research/ux/screenshots/2026-04-01/06_ai_config.png", full_page=True)

        await browser.close()

if __name__ == "__main__":
    asyncio.run(run())
