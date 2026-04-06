import asyncio
from playwright.async_api import async_playwright

async def main():
    async with async_playwright() as p:
        browser = await p.chromium.launch()
        page = await browser.new_page(viewport={"width": 1280, "height": 1024})

        try:
            print("Navigating to app...")
            await page.goto("http://127.0.0.1:8081", wait_until="networkidle", timeout=30000)

            print("Waiting for app to load...")
            # Wait a bit for flutter initialization
            await page.wait_for_timeout(5000)

            # Print page content / errors to diagnose blank screen
            content = await page.content()
            if "flutter-view" not in content and "flt-glass-pane" not in content:
                print("HTML content does not seem to have Flutter tags.")
                print(content[:1000])

            # Wait for any flutter element
            await page.wait_for_selector('flutter-view, flt-glass-pane, flt-scene', timeout=10000)
            print("Flutter view detected.")
            await page.wait_for_timeout(3000)

            # Screenshot
            await page.screenshot(path="debug_blank.png")
            print("Screenshot saved to debug_blank.png")

        except Exception as e:
            print(f"Error: {e}")
        finally:
            await browser.close()

asyncio.run(main())
