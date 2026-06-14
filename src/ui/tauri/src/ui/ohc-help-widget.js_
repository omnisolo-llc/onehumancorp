document.addEventListener('DOMContentLoaded', () => {
    // Inject tooltip CSS
    const style = document.createElement('style');
    style.textContent = `
        .ohc-tooltip {
            position: fixed;
            background: rgba(255, 255, 255, 0.85);
            backdrop-filter: blur(30px) saturate(210%); -webkit-backdrop-filter: blur(30px) saturate(210%);
            -webkit-backdrop-filter: blur(30px) saturate(210%); -webkit-backdrop-filter: blur(30px) saturate(210%);
            border: 1px solid rgba(255, 255, 255, 0.6);
            border-radius: 16px;
            padding: 12px 16px;
            color: #1d1d1f;
            font-family: "Outfit", -apple-system, BlinkMacSystemFont, sans-serif;
            font-size: 14px;
            line-height: 1.4;
            max-width: 250px;
            box-shadow: 0 10px 30px rgba(0, 0, 0, 0.1);
            z-index: 10000;
            pointer-events: none;
            opacity: 0;
            transform: translateY(10px);
            transition: opacity 0.2s ease, transform 0.2s ease;
            text-align: center;
        }
        .ohc-tooltip.visible {
            opacity: 1;
            transform: translateY(0);
        }
        @media (prefers-color-scheme: dark) {
            .ohc-tooltip {
                background: rgba(22, 22, 26, 0.7);
                border-color: rgba(255, 255, 255, 0.1);
                color: #f5f5f7;
            }
        }
    `;
    document.head.appendChild(style);

    if (!window.OHC_TOOLTIPS) {
        window.OHC_TOOLTIPS = {};
        fetch("/api/tooltips").then(r => r.json()).then(data => { window.OHC_TOOLTIPS = data; }).catch(e => {
            //
            //
            //
            //
            //
            //
            //
            console.error(e);
        });
    }

    const tooltipEl = document.createElement('div');
    tooltipEl.className = 'ohc-tooltip';
    document.body.appendChild(tooltipEl);

    function showTooltip(e, text) {
        if (!text) return;
        tooltipEl.textContent = text;
        const rect = e.target.getBoundingClientRect();

        // Position above the element
        let top = rect.top - tooltipEl.offsetHeight - 10;
        let left = rect.left + (rect.width / 2) - (tooltipEl.offsetWidth / 2);

        // Adjust if out of bounds
        if (top < 10) {
            top = rect.bottom + 10; // show below if not enough space above
        }
        if (left < 10) left = 10;
        if (left + tooltipEl.offsetWidth > window.innerWidth - 10) {
            left = window.innerWidth - tooltipEl.offsetWidth - 10;
        }

        tooltipEl.style.top = top + 'px';
        tooltipEl.style.left = left + 'px';
        tooltipEl.classList.add('visible');
    }

    function hideTooltip() {
        tooltipEl.classList.remove('visible');
    }

    // Hover (desktop)
    document.addEventListener('mouseover', (e) => {
        const t = e.target;
        const target = t && typeof t.closest === 'function' ? t.closest('[data-tooltip]') : null;
        if (target) {
            showTooltip(e, (window.OHC_TOOLTIPS && window.OHC_TOOLTIPS[target.id]) || target.getAttribute('data-tooltip'));
        }
    });

    document.addEventListener('mouseout', (e) => {
        const target = e.target;
        if (target && typeof target.closest === 'function' && target.closest('[data-tooltip]')) {
            hideTooltip();
        }
    });

    // Mobile long press (contextmenu) or touchstart
    let touchTimeout;
    document.addEventListener('touchstart', (e) => {
        const t = e.target;
        const target = t && typeof t.closest === 'function' ? t.closest('[data-tooltip]') : null;
        if (target) {
            touchTimeout = setTimeout(() => {
                showTooltip(e, (window.OHC_TOOLTIPS && window.OHC_TOOLTIPS[target.id]) || target.getAttribute('data-tooltip'));
            }, 500); // 500ms long press
        }
    });

    document.addEventListener('touchend', () => {
        clearTimeout(touchTimeout);
        hideTooltip();
    });

    document.addEventListener('touchmove', () => {
        clearTimeout(touchTimeout);
        hideTooltip();
    });
});
</script>
document.addEventListener('DOMContentLoaded', () => {
    // Inject chat CSS
    const style = document.createElement('style');
    style.textContent = `
        #ohc-help-btn {
            position: fixed;
            bottom: 20px;
            right: 20px;
            width: 56px;
            height: 56px;
            border-radius: 16px;
            background: #0066FF;
            color: white;
            border: none;
            box-shadow: 0 4px 14px rgba(0, 102, 255, 0.4);
            display: flex;
            align-items: center;
            justify-content: center;
            cursor: pointer;
            z-index: 9999;
            transition: transform 0.2s;
        }
        #ohc-help-btn:hover {
            transform: scale(1.05);
        }
        #ohc-help-btn svg {
            width: 28px;
            height: 28px;
            fill: none;
            stroke: currentColor;
            stroke-width: 2;
        }
        #ohc-help-chat-overlay {
            position: fixed;
            bottom: 90px;
            right: 20px;
            width: 350px;
            height: 500px;
            max-height: calc(100vh - 120px);
            max-width: calc(100vw - 40px);
            background: rgba(255, 255, 255, 0.85);
            backdrop-filter: blur(30px) saturate(2.1);
            -webkit-backdrop-filter: blur(30px) saturate(2.1);
            border: 1px solid rgba(255, 255, 255, 0.6);
            border-radius: 16px;
            box-shadow: 0 10px 40px rgba(0, 0, 0, 0.15);
            display: none;
            flex-direction: column;
            z-index: 9998;
            overflow: hidden;
            font-family: "Outfit", sans-serif;
            transform-origin: bottom right;
            animation: popIn 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
        }
        @keyframes popIn {
            0% { transform: scale(0.5); opacity: 0; }
            100% { transform: scale(1); opacity: 1; }
        }
        #ohc-help-chat-header {
            padding: 15px 20px;
            border-bottom: 1px solid rgba(0, 0, 0, 0.1);
            display: flex;
            justify-content: space-between;
            align-items: center;
            background: rgba(255, 255, 255, 0.5);
        }
        #ohc-help-chat-header h3 {
            margin: 0;
            font-size: 16px;
            color: #1d1d1f;
        }
        #ohc-help-close {
            background: none;
            border: none;
            color: #86868b;
            cursor: pointer;
            padding: 5px;
        }
        #ohc-help-close:hover { color: #1d1d1f; }
        #ohc-help-messages {
            flex-grow: 1;
            padding: 20px;
            overflow-y: auto;
            display: flex;
            flex-direction: column;
            gap: 15px;
        }
        .msg {
            max-width: 85%;
            padding: 12px 16px;
            border-radius: 16px;
            font-size: 14px;
            line-height: 1.4;
        }
        .msg-user {
            align-self: flex-end;
            background: #0066FF;
            color: white;
            border-bottom-right-radius: 4px;
        }
        .msg-ai {
            align-self: flex-start;
            background: rgba(0, 0, 0, 0.05);
            color: #1d1d1f;
            border-bottom-left-radius: 4px;
        }
        .msg-ai a {
            color: #0066FF;
            font-weight: 600;
            text-decoration: none;
            display: block;
            margin-top: 5px;
        }
        #ohc-help-input-area {
            padding: 15px;
            border-top: 1px solid rgba(0, 0, 0, 0.1);
            display: flex;
            gap: 10px;
            background: rgba(255, 255, 255, 0.5);
        }
        #ohc-help-input {
            flex-grow: 1;
            border: 1px solid rgba(0, 0, 0, 0.2);
            border-radius: 16px;
            padding: 10px 15px;
            outline: none;
            background: rgba(255, 255, 255, 0.8);
            font-size: 14px;
        }
        #ohc-help-input:focus { border-color: #0066FF; }
        #ohc-help-send {
            background: #0066FF;
            color: white;
            border: none;
            width: 40px;
            height: 40px;
            border-radius: 16px;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
        }

        @media (prefers-color-scheme: dark) {
            #ohc-help-chat-overlay {
                background: rgba(22, 22, 26, 0.7);
                border-color: rgba(255, 255, 255, 0.1);
            }
            #ohc-help-chat-header, #ohc-help-input-area {
                background: rgba(0, 0, 0, 0.2);
                border-color: rgba(255, 255, 255, 0.1);
            }
            #ohc-help-chat-header h3 { color: #f5f5f7; }
            .msg-ai {
                background: rgba(255, 255, 255, 0.1);
                color: #f5f5f7;
            }
            #ohc-help-input {
                background: rgba(0, 0, 0, 0.3);
                border-color: rgba(255, 255, 255, 0.2);
                color: white;
            }
        }
    `;
    document.head.appendChild(style);

    // Inject HTML
    const btn = document.createElement('button');
    btn.id = 'ohc-help-btn';
    btn.setAttribute('aria-label', 'Open help chat');
    btn.innerHTML = `<svg viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>`;
    document.body.appendChild(btn);

    const overlay = document.createElement('div');
    overlay.id = 'ohc-help-chat-overlay';
    overlay.innerHTML = `
        <div id="ohc-help-chat-header">
            <h3>Ask AI Help</h3>
            <button id="ohc-help-close" aria-label="Close help chat">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"></path></svg>
            </button>
        </div>
        <div id="ohc-help-messages">
            <div class="msg msg-ai">Hi! I am your AI Help Agent! How can I assist you with OHC today?</div>
        </div>

        <div id="ohc-help-walkthroughs" style="margin-top: 15px; border-top: 1px solid rgba(0,0,0,0.1); padding-top: 15px;">
            <h4 style="margin: 0 0 10px 0; font-size: 14px;">Interactive Tours</h4>
            <button onclick="window.startWalkthrough && window.startWalkthrough([{targetId: 'nav-store', title: 'Set up your store', content: 'Click here to go to your storefront builder.'}])" style="width: 100%; text-align: left; padding: 8px; border: 1px solid rgba(0,0,0,0.1); border-radius: 8px; background: white; margin-bottom: 8px; cursor: pointer; font-size: 13px;">Tour: Set up your store</button>
            <button onclick="window.startWalkthrough && window.startWalkthrough([{targetId: 'nav-agents', title: 'Activate your AI Support Agent', content: 'Activate your AI agent to help with tasks.'}])" style="width: 100%; text-align: left; padding: 8px; border: 1px solid rgba(0,0,0,0.1); border-radius: 8px; background: white; margin-bottom: 8px; cursor: pointer; font-size: 13px;">Tour: Activate your AI Support Agent</button>
        </div>

<div id="ohc-help-input-area">
            <input type="text" id="ohc-help-input" placeholder="Ask anything..." />
            <button id="ohc-help-send" aria-label="Send message">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"></path></svg>
            </button>
        </div>
    `;
    document.body.appendChild(overlay);

    const closeBtn = document.getElementById('ohc-help-close');
    const messages = document.getElementById('ohc-help-messages');
    const input = document.getElementById('ohc-help-input');
    const sendBtn = document.getElementById('ohc-help-send');

    btn.addEventListener('click', () => {
        overlay.style.display = 'flex';
        input.focus();
    });

    closeBtn.addEventListener('click', () => {
        overlay.style.display = 'none';
    });

        async function sendMessage() {
        const text = input.value.trim();
        if (!text) return;

        // Add user msg
        const uMsg = document.createElement('div');
        uMsg.className = 'msg msg-user';
        uMsg.textContent = text;
        messages.appendChild(uMsg);
        input.value = '';
        messages.scrollTop = messages.scrollHeight;

        try {
            let url = '/api/chat';
            if (window.__TAURI__ && window.__TAURI__.core) {
                url = 'http://127.0.0.1:18789/api/chat';
            }
            const response = await fetch(url, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ message: text })
            });
            const data = await response.json();

            const aiMsg = document.createElement('div');
            aiMsg.className = 'msg msg-ai';
            if (data.link && data.link.url) {
                aiMsg.innerHTML = `${data.reply}<br/><a href="${data.link.url}">${data.link.title}</a>`;
            } else {
                aiMsg.textContent = data.reply;
            }
            messages.appendChild(aiMsg);
            messages.scrollTop = messages.scrollHeight;
        } catch (e) {
            console.error(e);
            const aiMsg = document.createElement('div');
            aiMsg.className = 'msg msg-ai';
            aiMsg.textContent = 'Sorry, there was an error connecting to the help assistant.';
            messages.appendChild(aiMsg);
            messages.scrollTop = messages.scrollHeight;
        }
    }

    sendBtn.addEventListener('click', sendMessage);
    input.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') sendMessage();
    });
});

</script>
document.addEventListener('DOMContentLoaded', () => {
    const btn = document.getElementById('dashboard-invite-btn');
    const container = document.getElementById('dashboard-invite-container');
    const linkInput = document.getElementById('dashboard-invite-link');
    const copyBtn = document.getElementById('dashboard-copy-btn');
    const shareXBtn = document.getElementById('dashboard-share-x-btn');

    if(btn) {
        btn.addEventListener('click', async () => {
            btn.innerText = "Generating...";
            if (window.__TAURI__ && window.__TAURI__.core) {
              window.__TAURI__.core.invoke("generate_cloud_bridge_invite").then((link) => {
                linkInput.value = link;
                btn.style.display = 'none';
                container.style.display = 'block';
              }).catch((err) => {
                console.error(err);
                linkInput.value = "https://cloud.ohc.network/invite/fallback";
                btn.style.display = 'none';
                container.style.display = 'block';
              });
            } else {
              try {
                  const tenantId = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'default';
                  const res = await fetch('/api/v1/growth/cloud-bridge/invite', {
                      method: 'POST',
                      headers: { 'Content-Type': 'application/json' },
                      body: JSON.stringify({ team_id: tenantId, inviter_id: "owner", invitee_id: "pending" })
                  });
                  const data = await res.json();
                  linkInput.value = data.invite_link || "https://cloud.ohc.network/invite/fallback";
              } catch(e) {
                  console.error(e);
                  linkInput.value = "https://cloud.ohc.network/invite/fallback";
              }
              btn.style.display = 'none';
              container.style.display = 'block';
            }
        });

        copyBtn.addEventListener('click', () => {
            navigator.clipboard.writeText(linkInput.value);
            copyBtn.innerText = 'Copied!';
            setTimeout(() => { copyBtn.innerText = 'Copy'; }, 2000);
        });

        shareXBtn.addEventListener('click', () => {
            const text = `I manage my business with OHC! Join using my invite link and get 1 month free: ${linkInput.value}\n\n⚡ Powered by OHC`;
            window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent(text)}`, '_blank');
                localStorage.setItem(dismissedKey, 'true');
                document.getElementById('milestone-container').style.display = 'none';
        });
    }
});
</script>
document.addEventListener('DOMContentLoaded', () => {
    if (!window.OHC_TOOLTIPS) {
        window.OHC_TOOLTIPS = {};
        fetch("/api/tooltips").then(r => r.json()).then(data => { window.OHC_TOOLTIPS = data; }).catch(e => {
            //
            //
            //
            //
            //
            //
            //
            console.error(e);
        });
    }
    const tooltipEl = document.createElement('div');
    tooltipEl.className = 'ohc-tooltip';
    document.body.appendChild(tooltipEl);

    function showTooltip(e, text) {
        if (!text) return;
        tooltipEl.textContent = text;
        const targetRect = e.target.closest('[id]') ? e.target.closest('[id]').getBoundingClientRect() : e.target.getBoundingClientRect();

        let left = targetRect.left + (targetRect.width / 2) - (tooltipEl.offsetWidth / 2);
        let top = targetRect.bottom + 10;

        if (left + tooltipEl.offsetWidth > window.innerWidth - 10) {
            left = window.innerWidth - tooltipEl.offsetWidth - 10;
        } else if (left < 10) {
            left = 10;
        }

        if (top + tooltipEl.offsetHeight > window.innerHeight - 10) {
            top = targetRect.top - tooltipEl.offsetHeight - 10;
        }

        tooltipEl.style.top = `${top}px`;
        tooltipEl.style.left = `${left}px`;
        tooltipEl.classList.add('visible');
    }

    function hideTooltip() {
        tooltipEl.classList.remove('visible');
    }

    document.addEventListener('mouseover', (e) => {
        const target = e.target.closest('[id]');
        if (target && target.id && window.OHC_TOOLTIPS && window.OHC_TOOLTIPS[target.id]) {
            showTooltip(e, window.OHC_TOOLTIPS[target.id]);
        }
    });

    document.addEventListener('mouseout', (e) => {
        const target = e.target.closest('[id]');
        if (target && target.id && window.OHC_TOOLTIPS && window.OHC_TOOLTIPS[target.id]) {
            hideTooltip();
        }
    });
});
</script>
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
            background-color: rgba(0, 102, 255, 0.9);
            backdrop-filter: blur(10px);
            color: white;
            border: 1px solid rgba(255,255,255,0.2);
            box-shadow: 0 4px 16px rgba(0, 102, 255, 0.3);
            cursor: pointer;
            z-index: 99990;
            display: flex;
            align-items: center;
            justify-content: center;
            transition: transform 0.2s;
        }
        #ohc-floating-help-btn:hover {
            transform: scale(1.05);
        }
        #ohc-floating-help-btn svg {
            width: 28px;
            height: 28px;
            fill: currentColor;
        }
        #ohc-floating-help-widget {
            position: fixed;
            bottom: 96px;
            right: 24px;
            width: 380px;
            height: 600px;
            max-height: calc(100vh - 120px);
            background: rgba(255, 255, 255, 0.85);
            backdrop-filter: blur(20px);
            border-radius: 16px;
            box-shadow: 0 8px 32px rgba(0,0,0,0.15);
            z-index: 99990;
            display: none;
            flex-direction: column;
            overflow: hidden;
            font-family: Outfit, -apple-system, sans-serif;
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
            background: rgba(248, 250, 252, 0.6);
            border-bottom: 1px solid rgba(226, 232, 240, 0.5);
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
            border-bottom: 1px solid rgba(226, 232, 240, 0.5);
            background: rgba(248, 250, 252, 0.6);
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
            background: rgba(0, 102, 255, 0.9);
            color: white;
            align-self: flex-end;
            border-bottom-right-radius: 4px;
        }
        .ohc-chat-msg.agent {
            background: rgba(241, 245, 249, 0.8);
            color: #0f172a;
            align-self: flex-start;
            border-bottom-left-radius: 4px;
        }
        .ohc-chat-msg a {
            color: #0066FF;
            text-decoration: underline;
            font-weight: 500;
            display: block;
            margin-top: 4px;
        }
        #ohc-help-chat-input-container {
            display: flex;
            gap: 8px;
            padding-top: 12px;
            border-top: 1px solid rgba(226, 232, 240, 0.5);
        }
        #ohc-help-chat-input {
            flex: 1;
            padding: 10px 14px;
            border: 1px solid #cbd5e1;
            border-radius: 20px;
            font-size: 14px;
            outline: none;
            background: rgba(255, 255, 255, 0.8);
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
            border: 1px solid rgba(226, 232, 240, 0.5);
            border-radius: 8px;
            margin-bottom: 12px;
            cursor: pointer;
            transition: background 0.2s;
            background: rgba(255, 255, 255, 0.5);
        }
        .ohc-tour-card:hover {
            background: rgba(248, 250, 252, 0.8);
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

        /* Tooltip Styles */
        .ohc-tooltip {
            position: fixed;
            background: #333;
            color: white;
            padding: 8px 12px;
            border-radius: 6px;
            font-size: 13px;
            font-family: Outfit, sans-serif;
            z-index: 100000;
            pointer-events: none;
            opacity: 0;
            transition: opacity 0.2s;
            box-shadow: 0 4px 12px rgba(0,0,0,0.15);
            max-width: 250px;
            line-height: 1.4;
        }
        .ohc-tooltip.visible {
            opacity: 1;
        }
    `;
    document.head.appendChild(style);

    // Create the button
    const btn = document.createElement('button');
    btn.id = 'ohc-floating-help-btn';
    btn.setAttribute('data-tooltip', 'Help Center');
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
            <button class="ohc-help-tab" data-target="tab-chat" aria-label="Ask AI">Ask AI</button>
        </div>

        <div id="tab-articles" class="ohc-help-content active">
            <div style="margin-bottom: 16px;">
                <a href="/help.html" style="display: block; padding: 12px; background: rgba(241, 245, 249, 0.8); border-radius: 8px; text-decoration: none; color: #0f172a; font-weight: 500; text-align: center;">Open Full Help Center</a>
            </div>
            <h4>Popular Articles</h4>
            <ul style="list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 8px;" id="ohc-help-articles-list">
                <li><a href="/help_article.html?id=getting-started-1" style="color: #0066FF; text-decoration: none; font-size: 14px;">Welcome to One Human Corp</a></li>
                <li><a href="/help_article.html?id=my-store-1" style="color: #0066FF; text-decoration: none; font-size: 14px;">Setting up your storefront</a></li>
                <li><a href="/help_article.html?id=payments-1" style="color: #0066FF; text-decoration: none; font-size: 14px;">Accepting your first payment</a></li>
            </ul>
            <div style="margin-top: auto; padding-top: 16px; border-top: 1px solid rgba(226, 232, 240, 0.5);">
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

    function appendMessage(text, sender, link = null) {
        const msg = document.createElement('div');
        msg.className = `ohc-chat-msg ${sender}`;
        msg.innerHTML = text;
        if (link && link.url && link.title) {
            msg.innerHTML += `<a href="${link.url}">${link.title}</a>`;
        }
        chatMessages.appendChild(msg);
        chatMessages.scrollTop = chatMessages.scrollHeight;
    }

    async function handleSend() {
        const text = chatInput.value.trim();
        if (!text) return;

        appendMessage(text, 'user');
        chatInput.value = '';

        try {
            const res = await fetch('/api/chat', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ message: text })
            });
            const data = await res.json();

            if (data.reply) {
                appendMessage(data.reply.text || data.reply, 'agent', data.reply.link);
            } else if (data.text) {
                appendMessage(data.text, 'agent', data.link);
            } else {
                appendMessage(data, 'agent');
            }
        } catch (e) {
            appendMessage("I'm sorry, I'm having trouble connecting right now.", 'agent');
        }
    }

    chatSend.addEventListener('click', handleSend);
    chatInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') handleSend();
    });

    // --- Global Tooltip & Walkthrough Logic ---

    // Tooltips
    if (!window.OHC_TOOLTIPS) {
        window.OHC_TOOLTIPS = {};
        fetch("/api/tooltips").then(r => r.json()).then(data => { window.OHC_TOOLTIPS = data; }).catch(e => {
            //
            //
            //
            //
            //
            //
            console.error(e);
        });
    }
    const tooltipEl = document.createElement('div');
    tooltipEl.className = 'ohc-tooltip';
    document.body.appendChild(tooltipEl);

    function showTooltip(e, text) {
        if (!text) return;
        tooltipEl.textContent = text;
        const targetRect = e.target.closest('[id]') ? e.target.closest('[id]').getBoundingClientRect() : e.target.getBoundingClientRect();

        let left = targetRect.left + (targetRect.width / 2) - (tooltipEl.offsetWidth / 2);
        let top = targetRect.bottom + 10;

        if (left + tooltipEl.offsetWidth > window.innerWidth - 10) {
            left = window.innerWidth - tooltipEl.offsetWidth - 10;
        } else if (left < 10) {
            left = 10;
        }

        if (top + tooltipEl.offsetHeight > window.innerHeight - 10) {
            top = targetRect.top - tooltipEl.offsetHeight - 10;
        }

        tooltipEl.style.top = `${top}px`;
        tooltipEl.style.left = `${left}px`;
        tooltipEl.classList.add('visible');
    }

    function hideTooltip() {
        tooltipEl.classList.remove('visible');
    }

    document.addEventListener('mouseover', (e) => {
        const target = e.target.closest('[id]');
        if (target && target.id && window.OHC_TOOLTIPS && window.OHC_TOOLTIPS[target.id]) {
            showTooltip(e, window.OHC_TOOLTIPS[target.id]);
        }
    });

    document.addEventListener('mouseout', (e) => {
        const target = e.target.closest('[id]');
        if (target && target.id && window.OHC_TOOLTIPS && window.OHC_TOOLTIPS[target.id]) {
            hideTooltip();
        }
    });

    // Walkthroughs
    if (!window.startWalkthrough) {
        window.startWalkthrough = function(steps) {
            if (!steps || steps.length === 0) return;

            let currentStep = 0;

            const overlay = document.createElement('div');
            overlay.className = 'ohc-walkthrough-overlay';
            overlay.style.cssText = 'position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); z-index: 99998;';
            document.body.appendChild(overlay);

            const bubble = document.createElement('div');
            bubble.className = 'ohc-walkthrough-bubble';
            bubble.setAttribute('role', 'dialog');
            bubble.style.cssText = 'position: fixed; background: white; border-radius: 8px; padding: 16px; box-shadow: 0 4px 12px rgba(0,0,0,0.15); z-index: 99999; max-width: 300px; display: flex; flex-direction: column; gap: 8px; font-family: Outfit, sans-serif;';
            document.body.appendChild(bubble);

            function renderStep() {
                const step = steps[currentStep];

                document.querySelectorAll('.walkthrough-highlight, .ohc-walkthrough-highlight').forEach(el => {
                    el.classList.remove('walkthrough-highlight', 'ohc-walkthrough-highlight');
                    el.style.position = '';
                    el.style.zIndex = '';
                    el.style.background = '';
                });

                const target = document.getElementById(step.targetId) || document.querySelector(step.targetId || step.selector);

                bubble.innerHTML = `
                    <div style="display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid #eee; padding-bottom: 8px; margin-bottom: 8px;">
                        <h4 style="margin: 0; font-size: 16px; font-weight: bold;">${step.title || 'Tour'}</h4>
                        <button id="wt-close" class="ohc-walkthrough-close" style="background: none; border: none; cursor: pointer; font-size: 18px;">&times;</button>
                    </div>
                    <p style="margin: 0; font-size: 14px; color: #333;">${step.content || step.text}</p>
                    <div style="display: flex; justify-content: flex-end; gap: 8px; margin-top: 8px;">
                        ${currentStep > 0 ? '<button id="wt-prev" style="padding: 6px 12px; border: 1px solid #ccc; border-radius: 4px; background: white; cursor: pointer;">Back</button>' : ''}
                        <button id="wt-next" style="padding: 6px 12px; border: none; border-radius: 4px; background: #0066FF; color: white; cursor: pointer;">${currentStep === steps.length - 1 ? 'Finish' : 'Next'}</button>
                    </div>
                `;

                document.getElementById('wt-close').onclick = closeWalkthrough;
                if (document.getElementById('wt-prev')) document.getElementById('wt-prev').onclick = () => { currentStep--; renderStep(); };
                document.getElementById('wt-next').onclick = () => {
                    if (currentStep === steps.length - 1) {
                        closeWalkthrough();
                    } else {
                        currentStep++;
                        renderStep();
                    }
                };

                if (target) {
                    target.classList.add('walkthrough-highlight');
                    target.style.position = 'relative';
                    target.style.zIndex = '99999';
                    target.style.background = 'white';

                    const rect = target.getBoundingClientRect();
                    if (rect.bottom + 200 < window.innerHeight) {
                        bubble.style.top = (rect.bottom + 10) + 'px';
                    } else {
                        bubble.style.top = (rect.top - bubble.offsetHeight - 10) + 'px';
                    }
                    bubble.style.left = Math.max(10, Math.min(rect.left, window.innerWidth - 320)) + 'px';
                } else {
                    bubble.style.top = '50%';
                    bubble.style.left = '50%';
                    bubble.style.transform = 'translate(-50%, -50%)';
                }
            }

            function closeWalkthrough() {
                document.querySelectorAll('.walkthrough-highlight, .ohc-walkthrough-highlight').forEach(el => {
                    el.classList.remove('walkthrough-highlight', 'ohc-walkthrough-highlight');
                    el.style.position = '';
                    el.style.zIndex = '';
                    el.style.background = '';
                });
                if (overlay.parentNode) overlay.parentNode.removeChild(overlay);
                if (bubble.parentNode) bubble.parentNode.removeChild(bubble);
            }

            renderStep();
        };
    }
});
</script>
