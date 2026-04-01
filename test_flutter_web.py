import subprocess
import time
import os
import signal
from playwright.sync_api import sync_playwright

print("Starting tests for OHC Flutter Web...")

def start_server(port, env):
    print(f"Starting server on port {port} with env: {env}")
    process = subprocess.Popen(
        ["flutter", "run", "-d", "web-server", "--web-port", str(port), "--web-hostname", "127.0.0.1"],
        cwd="srcs/app",
        env={**os.environ, **env},
        preexec_fn=os.setsid,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE
    )
    # Give it time to start
    time.sleep(15)
    return process

def stop_server(process, port):
    print(f"Stopping server on port {port}...")
    try:
        os.killpg(os.getpgid(process.pid), signal.SIGTERM)
    except Exception as e:
        print(f"Error killing process: {e}")

    try:
        subprocess.run(f"kill -9 $(lsof -t -i :{port})", shell=True, check=False)
    except:
        pass
    print("Server stopped.")

os.makedirs("docs/research/ux/screenshots/2025-04-01", exist_ok=True)

# Start single flutter instance
port = 8080
process = start_server(port, {"OHC_MULTITENANT": "true"})

try:
    with sync_playwright() as p:
        print("Launching browser...")
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()
        page.goto(f"http://127.0.0.1:{port}/#/login")

        # Wait for the page to load
        page.wait_for_timeout(10000)
        page.screenshot(path="docs/research/ux/screenshots/2025-04-01/cloud_login.png")
        print("Cloud mode login screenshot saved.")

        # Click center of screen roughly to trigger anything
        page.mouse.click(500, 500)
        page.wait_for_timeout(2000)

        # Login
        # Use exact coordinates because Semantic query might fail in CanvasKit
        print("Attempting exact coordinate login...")
        page.mouse.click(400, 400) # This is a placeholder, might need adjustment
        page.wait_for_timeout(5000)
        page.screenshot(path="docs/research/ux/screenshots/2025-04-01/cloud_dashboard.png")
        print("Cloud mode dashboard screenshot saved.")

        # Try to open settings for standalone
        print("Attempting standalone settings view...")
        page.goto(f"http://127.0.0.1:{port}/#/login")
        page.wait_for_timeout(5000)
        page.mouse.click(750, 50) # Floating action button position roughly
        page.wait_for_timeout(2000)
        page.screenshot(path="docs/research/ux/screenshots/2025-04-01/standalone_settings.png")
        print("Standalone settings screenshot saved.")

        browser.close()
finally:
    stop_server(process, port)

print("Tests completed.")
