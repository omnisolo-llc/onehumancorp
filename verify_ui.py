import time
from playwright.sync_api import sync_playwright

def verify():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()

        # Give flutter time to load
        page.goto("http://localhost:3000")
        page.wait_for_timeout(5000)

        page.screenshot(path="/home/jules/verification.png")

        browser.close()

if __name__ == "__main__":
    verify()
