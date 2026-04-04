from playwright.sync_api import sync_playwright
import time
import os

def test_dashboard_visuals():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()

        # Navigate to the app
        print("Navigating to Flutter web app...")
        for _ in range(30):
            try:
                page.goto("http://localhost:3000")
                break
            except:
                time.sleep(1)
        else:
            print("Failed to connect to Flutter web server")
            return

        print("Waiting for dashboard to load...")
        time.sleep(10) # Allow flutter engine to render

        # In canvaskit/html mode we can try to hover the canvas directly in the center to trigger *something* if text matching fails
        try:
            print("Attempting to find text...")
            # We can't select text in canvaskit easily via playwright unless semantics are enabled.
            # We will just click and move mouse to trigger some hover.
            page.mouse.move(400, 400)
            time.sleep(1)
        except Exception as e:
            print(f"Error: {e}")

        os.makedirs('docs/research/ux/screenshots/verification', exist_ok=True)
        screenshot_path = 'docs/research/ux/screenshots/verification/dashboard_hover.png'
        page.screenshot(path=screenshot_path)
        print(f"Screenshot saved to {screenshot_path}")

        browser.close()

if __name__ == "__main__":
    test_dashboard_visuals()
