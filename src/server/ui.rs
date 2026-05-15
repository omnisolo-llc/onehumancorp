use axum::response::{Html, IntoResponse};

pub async fn ui_handler(req: axum::extract::Request) -> impl IntoResponse {
    let path = req.uri().path();
    let content = match path {
        "/login" => login_html(),
        "/business-setup" => business_setup_html(),
        "/signup" => signup_html(),
        "/agents" => agents_html(),
        "/settings" => settings_html(),
        "/analytics" => analytics_html(),
        "/integrations" => integrations_html(),
        "/tutorials" => tutorials_html(),
        "/billing" => billing_html(),
        "/profile" => profile_html(),
        "/notifications" => notifications_html(),
        "/inbox" => inbox_html(),
        "/tasks" => tasks_html(),
        "/calendar" => calendar_html(),
        "/storefront" => storefront_html(),
        "/products" => products_html(),
        "/orders" => orders_html(),
        "/customers" => customers_html(),
        "/discounts" => discounts_html(),
        "/reports" => reports_html(),
        "/help" => help_html(),
        "/faq" => faq_html(),
        "/contact" => contact_html(),
        _ => dashboard_html(),
    };
    Html(content)
}


fn business_setup_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Business Setup</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #1a1a2e; --bg-darker: #16213e; --glass-bg: rgba(255, 255, 255, 0.05); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: linear-gradient(135deg, var(--bg-dark), var(--bg-darker)); color: white; display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; margin: 0; }
            nav { position: absolute; top: 0; width: 100%; padding: 20px; display: flex; gap: 20px; backdrop-filter: blur(10px); background: rgba(255, 255, 255, 0.05); box-sizing: border-box; }
            nav a { color: white; text-decoration: none; font-weight: 500; min-height: 44px; display: flex; align-items: center; padding: 0 10px; }
            #root { background: rgba(255, 255, 255, 0.1); backdrop-filter: blur(20px) saturate(200%); border-radius: 20px; padding: 40px; border: 1px solid rgba(255, 255, 255, 0.2); box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37); width: 90%; max-width: 400px; animation: slideUp 0.3s cubic-bezier(0.4, 0, 0.2, 1); }
            @keyframes slideUp { from { transform: translateY(20px); opacity: 0; } to { transform: translateY(0); opacity: 1; } }
            h1 { font-family: 'Outfit', sans-serif; margin-top: 0; font-size: 28px; text-align: center; }
            p { text-align: center; font-size: 18px; margin-bottom: 30px; }
            input { display: none; width: 100%; padding: 15px; border-radius: 12px; border: 1px solid var(--glass-border); background: rgba(255, 255, 255, 0.1); color: white; margin-bottom: 20px; font-family: 'Inter', sans-serif; box-sizing: border-box; font-size: 16px; transition: border-color 0.2s; }
            input:focus { outline: none; border-color: var(--primary); }
            button { width: 100%; padding: 15px; background: var(--primary); border: none; border-radius: 12px; color: var(--bg-dark); font-weight: 600; font-size: 16px; cursor: pointer; min-height: 44px; font-family: 'Outfit', sans-serif; transition: transform 0.2s; }
            button:active { transform: scale(0.98); }
        </style>
    </head>
    <body>
        <nav>
            <a href="/">Dashboard</a>
            <a href="/agents">Agents</a>
        </nav>
        <div id="root">
            <h1>One Human Corp</h1>
            <p id="wizard-text">Your business, live in minutes.</p>
            <input type="text" id="setup-input" placeholder="e.g. Online Store" />
            <button id="next-btn">Next Step</button>
        </div>
        <script>
            let step = 0;
            document.getElementById('next-btn').addEventListener('click', () => {
                step++;
                const text = document.getElementById('wizard-text');
                const input = document.getElementById('setup-input');
                if (step === 1) {
                    text.innerText = 'What is your business type?';
                    input.style.display = 'block';
                    input.focus();
                } else if (step === 2) {
                    text.innerText = 'What is your company name?';
                    input.value = '';
                    input.placeholder = 'e.g. My Great Business';
                } else if (step === 3) {
                    text.innerText = 'What do you sell';
                    input.style.display = 'none';
                    input.value = '';
                } else {
                    window.location.href = '/';
                }
            });
        </script>
    </body>
    </html>
    "#
}

fn login_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Login</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #1a1a2e; --bg-darker: #16213e; --glass-bg: rgba(255, 255, 255, 0.05); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: linear-gradient(135deg, var(--bg-dark), var(--bg-darker)); color: white; margin: 0; min-height: 100vh; display: flex; align-items: center; justify-content: center; }
            h1 { font-family: 'Outfit', sans-serif; font-size: 32px; margin-top: 0; margin-bottom: 10px; text-align: center; }
            .glass-panel { background: var(--glass-bg); backdrop-filter: blur(20px) saturate(200%); border: 1px solid var(--glass-border); border-radius: 24px; padding: 40px; width: 90%; max-width: 360px; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37); animation: slideUp 0.3s cubic-bezier(0.4, 0, 0.2, 1); }
            @keyframes slideUp { from { transform: translateY(20px); opacity: 0; } to { transform: translateY(0); opacity: 1; } }
            .input-group { margin-bottom: 20px; position: relative; }
            input { width: 100%; padding: 15px; border-radius: 12px; border: 1px solid var(--glass-border); background: rgba(255, 255, 255, 0.1); color: white; font-family: 'Inter', sans-serif; box-sizing: border-box; font-size: 16px; transition: border-color 0.2s; }
            input:focus { outline: none; border-color: var(--primary); }
            button.primary { width: 100%; padding: 15px; background: var(--primary); border: none; border-radius: 12px; color: var(--bg-dark); font-weight: 600; font-size: 16px; cursor: pointer; min-height: 44px; font-family: 'Outfit', sans-serif; transition: transform 0.2s; }
            button.secondary { width: 100%; padding: 15px; background: transparent; border: 1px solid var(--glass-border); border-radius: 12px; color: white; font-weight: 500; font-size: 16px; cursor: pointer; margin-top: 15px; min-height: 44px; font-family: 'Inter', sans-serif; }
            button:active { transform: scale(0.98); }
            .show-password { position: absolute; right: 15px; top: 15px; background: none; border: none; color: var(--primary); cursor: pointer; font-size: 14px; padding: 0; min-height: auto; width: auto; font-family: 'Inter', sans-serif; }
        </style>
    </head>
    <body>
        <div class="glass-panel">
            <h1>One Human Corp</h1>
            <p class="subtitle" style="text-align: center; color: rgba(255,255,255,0.7); margin-bottom: 30px;">Sign in to manage your business</p>
            <div class="input-group"><input type="email" placeholder="Email Address" id="email" /></div>
            <div class="input-group"><input type="password" placeholder="Password" id="password" /><button type="button" class="show-password" onclick="togglePassword()">Show</button></div>
            <button class="primary" onclick="window.location.href='/'">Sign In</button>
            <button class="secondary" onclick="alert('Help is on the way!')">Fix App Issues</button>
            <button class="secondary" onclick="window.location.href='/business-setup'">🚀 Start Business Setup</button>
        </div>
        <script>
            function togglePassword() {
                const pwd = document.getElementById('password');
                const btn = document.querySelector('.show-password');
                if (pwd.type === 'password') { pwd.type = 'text'; btn.textContent = 'Hide'; } else { pwd.type = 'password'; btn.textContent = 'Show'; }
            }
        </script>
    </body>
    </html>
    "#
}

fn dashboard_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Dashboard</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); position: sticky; top: 0; z-index: 100; background: rgba(15, 23, 42, 0.8); backdrop-filter: blur(20px); }
            .nav-links { display: flex; gap: 20px; }
            .nav-links a { color: white; text-decoration: none; font-weight: 500; min-height: 44px; display: flex; align-items: center; padding: 0 10px; }
            .nav-links a.active { color: var(--primary); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; animation: fadeIn 0.4s ease-out; }
            @keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; font-family: 'Inter', sans-serif; min-height: 44px; display: flex; align-items: center; gap: 8px; }
            .tabs { display: flex; gap: 10px; margin-bottom: 20px; border-bottom: 1px solid var(--glass-border); padding-bottom: 10px; overflow-x: auto; }
            .tab-btn { background: transparent; border: none; color: rgba(255,255,255,0.6); padding: 10px 20px; cursor: pointer; font-size: 16px; font-family: 'Outfit', sans-serif; white-space: nowrap; min-height: 44px; border-radius: 8px; }
            .tab-btn.active { color: var(--primary); background: rgba(78, 204, 163, 0.1); }
            .tab-content { display: none; }
            .tab-content.active { display: block; }
            .integration-item { display: flex; justify-content: space-between; align-items: center; padding: 20px; background: rgba(255,255,255,0.02); border-radius: 12px; margin-bottom: 15px; border: 1px solid rgba(255,255,255,0.05); }
            .action-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 10px 20px; border-radius: 8px; font-weight: 600; cursor: pointer; min-height: 44px; font-family: 'Outfit', sans-serif;}
            .api-docs-section { background: rgba(0,0,0,0.2); padding: 20px; border-radius: 12px; margin-top: 20px; border: 1px solid var(--glass-border); }
            .endpoint { font-family: monospace; background: rgba(255,255,255,0.1); padding: 4px 8px; border-radius: 4px; color: #a78bfa; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <a href="/" class="active">Home</a>
                <a href="/agents">Agents</a>
                <button class="menu-btn" onclick="openMenu()">Menu ☰</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <div style="display:flex; justify-content:space-between; align-items:center;">
                    <h1 style="color: var(--primary);">My Business</h1>
                </div>
                <div style="display:flex; align-items:center; gap:10px;">
                    <h2 style="margin:0;">Quick Actions</h2>
                    <button onclick="document.getElementById('quick-hint').style.display='block'" style="background:transparent; border:1px solid rgba(255,255,255,0.2); color:white; border-radius:50%; width:30px; height:30px; cursor:pointer;">?</button>
                </div>
                <div id="quick-hint" style="display:none; color:rgba(255,255,255,0.7); font-size:14px; margin-top:10px;">These buttons are shortcuts to your most common daily tasks.</div>
            </div>

            <div id="extended-menu" class="card glass" style="display: none;">
                <div class="tabs">
                    <button class="tab-btn active" onclick="switchTab('overview', this)">Overview</button>
                    <button class="tab-btn" onclick="switchTab('integrations', this)">Connect Custom Software</button>
                    <button class="tab-btn" onclick="switchTab('tutorials', this)">Video Tutorials</button>
                </div>

                <div id="overview" class="tab-content active">
                    <h3>Today's Summary</h3>
                    <p>Everything is looking good. You have 3 new orders to fulfill.</p>
                </div>

                <div id="integrations" class="tab-content">
                    <h2>Integration Options</h2>
                    <p style="color: rgba(255,255,255,0.7); margin-bottom: 24px;">Link your external tools and services to One Human Corp easily.</p>

                    <div class="integration-item">
                        <div>
                            <h3 style="margin-bottom: 5px;">Custom Integration</h3>
                            <p style="margin: 0; color: rgba(255,255,255,0.6); font-size: 14px;">Build your own connection using our simple data access.</p>
                        </div>
                        <button class="action-btn" onclick="toggleApiDocs()">View Data Access</button>
                    </div>

                    <div id="api-docs" class="api-docs-section" style="display: none;">
                        <h3 style="color: var(--primary);">Product Data Access</h3>
                        <p>Use these simple connections to read your store data from other software.</p>

                        <div style="margin-top: 20px;">
                            <div style="display: flex; align-items: center; gap: 15px; margin-bottom: 15px;">
                                <span class="endpoint">Read Product List</span>
                                <span style="color: rgba(255,255,255,0.7);">Get all your current products and prices</span>
                            </div>
                        </div>
                    </div>
                </div>

                <div id="tutorials" class="tab-content">
                    <h2>Video Tutorials</h2>
                    <p style="color: rgba(255,255,255,0.7); margin-bottom: 24px;">Learn how to manage your business with ease.</p>
                </div>
            </div>
        </main>

        <script>
            function openMenu() {
                const menu = document.getElementById('extended-menu');
                if (menu.style.display === 'none') {
                    menu.style.display = 'block';
                    menu.scrollIntoView({ behavior: 'smooth', block: 'start' });
                } else {
                    menu.style.display = 'none';
                }
            }

            function switchTab(tabId, btn) {
                document.querySelectorAll('.tab-content').forEach(el => el.classList.remove('active'));
                document.querySelectorAll('.tab-btn').forEach(el => el.classList.remove('active'));
                document.getElementById(tabId).classList.add('active');
                btn.classList.add('active');
            }

            function toggleApiDocs() {
                const docs = document.getElementById('api-docs');
                docs.style.display = docs.style.display === 'none' ? 'block' : 'none';
            }
        </script>
    </body>
    </html>
    "#
}
fn signup_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Signup</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Signup</h1>
                <p>Manage your signup settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
fn agents_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Agents</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Agents</h1>
                <p>Manage your agents settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
fn settings_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Settings</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Settings</h1>
                <p>Manage your settings settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
fn analytics_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Analytics</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Analytics</h1>
                <p>Manage your analytics settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
fn integrations_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Integrations</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Integrations</h1>
                <p>Manage your integrations settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
fn tutorials_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Tutorials</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Tutorials</h1>
                <p>Manage your tutorials settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
fn billing_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Billing</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Billing</h1>
                <p>Manage your billing settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
fn profile_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Profile</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Profile</h1>
                <p>Manage your profile settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
fn notifications_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Notifications</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Notifications</h1>
                <p>Manage your notifications settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
fn inbox_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Inbox</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Inbox</h1>
                <p>Manage your inbox settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
fn tasks_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Tasks</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Tasks</h1>
                <p>Manage your tasks settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
fn calendar_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Calendar</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Calendar</h1>
                <p>Manage your calendar settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
fn storefront_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Storefront</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Storefront</h1>
                <p>Manage your storefront settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
fn products_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Products</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Products</h1>
                <p>Manage your products settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
fn orders_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Orders</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Orders</h1>
                <p>Manage your orders settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
fn customers_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Customers</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Customers</h1>
                <p>Manage your customers settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
fn discounts_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Discounts</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Discounts</h1>
                <p>Manage your discounts settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
fn reports_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Reports</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Reports</h1>
                <p>Manage your reports settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
fn help_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Help</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Help</h1>
                <p>Manage your help settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
fn faq_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Faq</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Faq</h1>
                <p>Manage your faq settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
fn contact_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>One Human Corp - Contact</title>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&family=Inter:wght@400;500&display=swap" rel="stylesheet">
        <style>
            :root { --primary: #4ecca3; --bg-dark: #0f172a; --glass-bg: rgba(255, 255, 255, 0.03); --glass-border: rgba(255, 255, 255, 0.1); }
            body { font-family: 'Inter', sans-serif; background: var(--bg-dark); color: white; margin: 0; min-height: 100vh; }
            h1, h2, h3 { font-family: 'Outfit', sans-serif; margin-top: 0; }
            .glass { background: var(--glass-bg); backdrop-filter: blur(20px); border: 1px solid var(--glass-border); }
            nav { padding: 15px 20px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--glass-border); }
            main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; }
            .card { border-radius: 20px; padding: 30px; margin-bottom: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
            .menu-btn { background: transparent; border: 1px solid var(--glass-border); color: white; padding: 10px 20px; border-radius: 10px; cursor: pointer; min-height: 44px; transition: all 0.2s; }
            .menu-btn:hover { background: rgba(255,255,255,0.1); }
            .hint { display: none; margin-top: 10px; font-size: 14px; color: rgba(255,255,255,0.7); line-height: 1.5; }
            .primary-btn { background: var(--primary); color: var(--bg-dark); border: none; padding: 12px 24px; border-radius: 12px; font-weight: 600; font-family: 'Outfit', sans-serif; cursor: pointer; min-height: 44px; }
            .input-field { width: 100%; padding: 12px; border-radius: 8px; border: 1px solid var(--glass-border); background: rgba(255,255,255,0.05); color: white; margin-bottom: 16px; font-family: 'Inter', sans-serif; }
            .grid-layout { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; }
        </style>
    </head>
    <body>
        <nav>
            <div style="font-family: 'Outfit', sans-serif; font-weight: 700; font-size: 20px; color: var(--primary);">One Human Corp</div>
            <div class="nav-links">
                <button class="menu-btn" onclick="openMenu()">Menu</button>
            </div>
        </nav>

        <main>
            <div class="card glass">
                <h1 style="color: var(--primary);">Contact</h1>
                <p>Manage your contact settings here.</p>
                <div class="grid-layout">
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Statistics</h3>
                        <p>Total items: 0</p>
                    </div>
                    <div class="card glass" style="padding: 20px; margin-bottom: 0;">
                        <h3>Recent Activity</h3>
                        <p>No recent activity found.</p>
                    </div>
                </div>
            </div>
        </main>
        <script>
            function openMenu() { console.log('Menu opened'); }
        </script>
    </body>
    </html>
    "#
}
