document.addEventListener('DOMContentLoaded', () => {
    // Inject floating widget styles
    const style = document.createElement('style');
    style.textContent = `
        #ohc-floating-help-btn {
            position: fixed;
            bottom: 24px;
            right: 24px;
            width: 56px;
            height: 56px;
            border-radius: 28px;
            background-color: #0066FF;
            color: white;
            border: none;
            box-shadow: 0 4px 12px rgba(0, 102, 255, 0.3);
            cursor: pointer;
            z-index: 100000;
            display: flex;
            align-items: center;
            justify-content: center;
            transition: transform 0.2s;
        }
        #ohc-floating-help-btn:hover {
            transform: scale(1.05);
        }
        #ohc-floating-help-btn svg {
            width: 24px;
            height: 24px;
            fill: currentColor;
        }
        #ohc-floating-help-widget {
            position: fixed;
            bottom: 96px;
            right: 24px;
            width: 380px;
            height: 600px;
            max-height: calc(100vh - 120px);
            background: white;
            border-radius: 16px;
            box-shadow: 0 8px 32px rgba(0,0,0,0.15);
            z-index: 100000;
            display: none;
            flex-direction: column;
            overflow: hidden;
            font-family: Outfit, sans-serif;
            border: 1px solid rgba(0,0,0,0.05);
        }
        @media (max-width: 480px) {
            #ohc-floating-help-widget {
                bottom: 0;
                right: 0;
                width: 100%;
                height: 100%;
                max-height: 100vh;
                border-radius: 0;
            }
        }
        #ohc-floating-help-header {
            padding: 16px;
            background: #f8fafc;
            border-bottom: 1px solid #e2e8f0;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        #ohc-floating-help-header h3 {
            margin: 0;
            font-size: 18px;
            font-weight: 600;
            color: #0f172a;
        }
        #ohc-floating-help-close {
            background: none;
            border: none;
            cursor: pointer;
            color: #64748b;
            padding: 4px;
        }
        #ohc-floating-help-tabs {
            display: flex;
            border-bottom: 1px solid #e2e8f0;
            background: #f8fafc;
        }
        .ohc-help-tab {
            flex: 1;
            padding: 12px;
            text-align: center;
            background: none;
            border: none;
            cursor: pointer;
            font-weight: 500;
            color: #64748b;
            border-bottom: 2px solid transparent;
            font-size: 14px;
        }
        .ohc-help-tab.active {
            color: #0066FF;
            border-bottom-color: #0066FF;
        }
        .ohc-help-content {
            display: none;
            flex: 1;
            overflow-y: auto;
            padding: 16px;
        }
        .ohc-help-content.active {
            display: flex;
            flex-direction: column;
        }

        /* Chat styles */
        #ohc-help-chat-messages {
            flex: 1;
            overflow-y: auto;
            display: flex;
            flex-direction: column;
            gap: 12px;
            margin-bottom: 16px;
        }
        .ohc-chat-msg {
            padding: 12px 16px;
            border-radius: 12px;
            max-width: 85%;
            font-size: 14px;
            line-height: 1.5;
        }
        .ohc-chat-msg.user {
            background: #0066FF;
            color: white;
            align-self: flex-end;
            border-bottom-right-radius: 4px;
        }
        .ohc-chat-msg.agent {
            background: #f1f5f9;
            color: #0f172a;
            align-self: flex-start;
            border-bottom-left-radius: 4px;
        }
        .ohc-chat-msg a {
            color: #0066FF;
            text-decoration: underline;
            font-weight: 500;
        }
        #ohc-help-chat-input-container {
            display: flex;
            gap: 8px;
            padding-top: 12px;
            border-top: 1px solid #e2e8f0;
        }
        #ohc-help-chat-input {
            flex: 1;
            padding: 10px 14px;
            border: 1px solid #cbd5e1;
            border-radius: 20px;
            font-size: 14px;
            outline: none;
        }
        #ohc-help-chat-input:focus {
            border-color: #0066FF;
        }
        #ohc-help-chat-send {
            background: #0066FF;
            color: white;
            border: none;
            border-radius: 20px;
            padding: 0 16px;
            font-weight: 500;
            cursor: pointer;
        }

        /* Tours styles */
        .ohc-tour-card {
            padding: 16px;
            border: 1px solid #e2e8f0;
            border-radius: 8px;
            margin-bottom: 12px;
            cursor: pointer;
            transition: background 0.2s;
        }
        .ohc-tour-card:hover {
            background: #f8fafc;
        }
        .ohc-tour-card h4 {
            margin: 0 0 4px 0;
            font-size: 15px;
            color: #0f172a;
        }
        .ohc-tour-card p {
            margin: 0;
            font-size: 13px;
            color: #64748b;
        }
    `;
    document.head.appendChild(style);

    // Create the button
    const btn = document.createElement('button');
    btn.id = 'ohc-floating-help-btn';
    btn.setAttribute('aria-label', 'Help');
    btn.title = 'Help';
    btn.innerHTML = `<svg viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 17h-2v-2h2v2zm2.07-7.75l-.9.92C13.45 12.9 13 13.5 13 15h-2v-.5c0-1.1.45-2.1 1.17-2.83l1.24-1.26c.37-.36.59-.86.59-1.41 0-1.1-.9-2-2-2s-2 .9-2 2H8c0-2.21 1.79-4 4-4s4 1.79 4 4c0 .88-.36 1.68-.93 2.25z"/></svg>`;
    document.body.appendChild(btn);

    // Create the widget
    const widget = document.createElement('div');
    widget.id = 'ohc-floating-help-widget';
    widget.innerHTML = `
        <div id="ohc-floating-help-header">
            <h3>Help Center</h3>
            <button id="ohc-floating-help-close" aria-label="Close">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
            </button>
        </div>
        <div id="ohc-floating-help-tabs">
            <button class="ohc-help-tab active" data-target="tab-articles">Articles</button>
            <button class="ohc-help-tab" data-target="tab-tours">Interactive Tours</button>
            <button class="ohc-help-tab" data-target="tab-chat">Ask AI</button>
        </div>

        <div id="tab-articles" class="ohc-help-content active">
            <div style="margin-bottom: 16px;">
                <a href="/help.html" style="display: block; padding: 12px; background: #f1f5f9; border-radius: 8px; text-decoration: none; color: #0f172a; font-weight: 500; text-align: center;">Open Full Help Center</a>
            </div>
            <h4>Popular Articles</h4>
            <ul style="list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 8px;">
                <li><a href="/help_article.html?id=getting-started-1" style="color: #0066FF; text-decoration: none; font-size: 14px;">Welcome to One Human Corp</a></li>
                <li><a href="/help_article.html?id=my-store-1" style="color: #0066FF; text-decoration: none; font-size: 14px;">Setting up your storefront</a></li>
                <li><a href="/help_article.html?id=payments-1" style="color: #0066FF; text-decoration: none; font-size: 14px;">Accepting your first payment</a></li>
            </ul>
            <div style="margin-top: auto; padding-top: 16px; border-top: 1px solid #e2e8f0;">
                <a href="/api-docs.html" style="color: #64748b; font-size: 13px; text-decoration: none;">API Reference for advanced users</a>
            </div>
        </div>

        <div id="tab-tours" class="ohc-help-content">
            <div class="ohc-tour-card" onclick="window.startWalkthrough && window.startWalkthrough([{targetId: '#nav-store', title: 'Set up your store', content: 'Click here to access your storefront and add your first products.'}])">
                <h4>Set up your store</h4>
                <p>Learn how to add products and customize your storefront.</p>
            </div>
            <div class="ohc-tour-card" onclick="window.startWalkthrough && window.startWalkthrough([{targetId: '#nav-settings', title: 'Accept your first payment', content: 'Go to Settings > Payments to connect your bank account.'}])">
                <h4>Accept your first payment</h4>
                <p>Connect your account to start receiving money.</p>
            </div>
            <div class="ohc-tour-card" onclick="window.startWalkthrough && window.startWalkthrough([{targetId: '#nav-agents', title: 'Activate your AI Support Agent', content: 'Visit the AI Agents tab to hire your first digital assistant.'}])">
                <h4>Activate your AI Support Agent</h4>
                <p>Let AI handle customer queries for you.</p>
            </div>
        </div>

        <div id="tab-chat" class="ohc-help-content" style="padding-bottom: 12px;">
            <div id="ohc-help-chat-messages">
                <div class="ohc-chat-msg agent">
                    Hi! I'm your Help Agent. How can I assist you today? You can ask me anything about using OHC.
                </div>
            </div>
            <div id="ohc-help-chat-input-container">
                <input type="text" id="ohc-help-chat-input" placeholder="Ask anything...">
                <button id="ohc-help-chat-send" aria-label="Send message">Send</button>
            </div>
        </div>
    `;
    document.body.appendChild(widget);

    // Logic
    btn.addEventListener('click', () => {
        widget.style.display = widget.style.display === 'flex' ? 'none' : 'flex';
    });

    document.getElementById('ohc-floating-help-close').addEventListener('click', () => {
        widget.style.display = 'none';
    });

    const tabs = widget.querySelectorAll('.ohc-help-tab');
    const contents = widget.querySelectorAll('.ohc-help-content');

    tabs.forEach(tab => {
        tab.addEventListener('click', () => {
            tabs.forEach(t => t.classList.remove('active'));
            contents.forEach(c => c.classList.remove('active'));

            tab.classList.add('active');
            document.getElementById(tab.getAttribute('data-target')).classList.add('active');
        });
    });

    // Chat Logic
    const chatInput = document.getElementById('ohc-help-chat-input');
    const chatSend = document.getElementById('ohc-help-chat-send');
    const chatMessages = document.getElementById('ohc-help-chat-messages');

    function appendMessage(text, sender) {
        const msg = document.createElement('div');
        msg.className = `ohc-chat-msg ${sender}`;
        msg.innerHTML = text;
        chatMessages.appendChild(msg);
        chatMessages.scrollTop = chatMessages.scrollHeight;
    }

    function handleSend() {
        const text = chatInput.value.trim();
        if (!text) return;

        appendMessage(text, 'user');
        chatInput.value = '';

        // Simple mock response for now
        setTimeout(() => {
            let response = "I can help with that! Here's an article that might be useful: <a href='/help_article.html?id=getting-started-1'>Read the full article →</a>";
            if (text.toLowerCase().includes("product")) {
                response = "To add a product, go to your Storefront and click 'Add Product'. For more details, <a href='/help_article.html?id=my-store-1'>Read the full article →</a>";
            } else if (text.toLowerCase().includes("payment")) {
                response = "You can manage payments in the Payments section of your settings. <a href='/help_article.html?id=payments-1'>Read the full article →</a>";
            }
            appendMessage(response, 'agent');
        }, 600);
    }

    chatSend.addEventListener('click', handleSend);
    chatInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') handleSend();
    });
});
