from playwright.sync_api import sync_playwright

def run_cuj(page):
    page.goto("http://127.0.0.1:8081/#/dashboard")
    page.wait_for_timeout(10000) # Wait for web server and flutter canvas rendering
    page.mouse.click(10, 10) # Dismiss any overlays
    page.screenshot(path="/home/jules/verification/screenshots/flutter3.png")
    page.wait_for_timeout(1000)

if __name__ == "__main__":
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        context = browser.new_context(
            record_video_dir="/home/jules/verification/videos",
            viewport={'width': 1200, 'height': 800}
        )
        page = context.new_page()
        try:
            run_cuj(page)
        finally:
            context.close()
            browser.close()
