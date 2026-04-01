import subprocess
import time
import os

print("Starting tests for OHC...")

# Install playwright
subprocess.run(["pip3", "install", "playwright"], check=True)
subprocess.run(["playwright", "install"], check=True)

# Generate playwright script
playwright_script = """
from playwright.sync_api import sync_playwright
import time
import os

# Ensure screenshots directory exists
os.makedirs("docs/research/ux/screenshots/2025-04-01", exist_ok=True)

def test_cloud_mode():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()
        page.goto("http://127.0.0.1:8080/#/login")

        # Wait for the page to load
        time.sleep(2)
        page.screenshot(path="docs/research/ux/screenshots/2025-04-01/cloud_login.png")
        print("Cloud mode login screenshot saved.")

        # Click SSO Login
        page.get_by_role("button", name="Sign in with SSO").click()

        # Wait for the dashboard to load after login delay
        time.sleep(3)
        page.screenshot(path="docs/research/ux/screenshots/2025-04-01/cloud_dashboard.png")
        print("Cloud mode dashboard screenshot saved.")

        browser.close()

def test_standalone_mode():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()
        page.goto("http://127.0.0.1:8081/#/login")

        # Wait for the page to load
        time.sleep(2)
        page.screenshot(path="docs/research/ux/screenshots/2025-04-01/standalone_login.png")
        print("Standalone mode login screenshot saved.")

        # In standalone mode, we might need to change settings
        page.get_by_label("Remote Connection Settings").click()
        time.sleep(1)
        page.screenshot(path="docs/research/ux/screenshots/2025-04-01/standalone_settings.png")
        print("Standalone settings screenshot saved.")

        browser.close()

if __name__ == "__main__":
    test_cloud_mode()
    test_standalone_mode()
"""

with open("test_ui.py", "w") as f:
    f.write(playwright_script)

print("Playwright script generated.")
