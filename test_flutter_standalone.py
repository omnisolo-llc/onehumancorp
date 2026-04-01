import subprocess
import time
import os
import signal
from playwright.sync_api import sync_playwright

print("Starting tests for Standalone Mode...")

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

os.makedirs("docs/research/ux/screenshots/2025-04-01", exist_ok=True)

port = 8081
process = start_server(port, {"OHC_STANDALONE": "true"})

try:
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()
        page.goto(f"http://127.0.0.1:{port}/#/login")

        page.wait_for_timeout(10000)
        page.screenshot(path="docs/research/ux/screenshots/2025-04-01/standalone_login.png")
        print("Standalone mode login screenshot saved.")

        # Click Floating Action button for remote settings
        page.mouse.click(750, 550) # Assuming a typical 800x600 resolution default
        page.wait_for_timeout(2000)
        page.screenshot(path="docs/research/ux/screenshots/2025-04-01/standalone_settings.png")
        print("Standalone settings screenshot saved.")

        browser.close()
finally:
    stop_server(process, port)

print("Tests completed.")
