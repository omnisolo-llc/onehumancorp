document.addEventListener('DOMContentLoaded', () => {
    // Inject Styles
    const style = document.createElement('style');
    style.id = 'ohc-help-style';
    style.innerHTML = `
        #ohc-help-btn {
            position: fixed;
            bottom: 30px;
            right: 30px;
            width: 50px;
            height: 50px;
            border-radius: 25px;
            background: #0066FF;
            color: white;
            border: none;
            box-shadow: 0 4px 15px rgba(0, 102, 255, 0.4);
            cursor: pointer;
            z-index: 9999;
            display: flex;
            align-items: center;
            justify-content: center;
            transition: transform 0.2s cubic-bezier(0.175, 0.885, 0.32, 1.275);
        }
        #ohc-help-btn:hover {
            transform: scale(1.1);
        }
        #ohc-help-btn svg {
            width: 24px;
            height: 24px;
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
            backdrop-filter: blur(30px) saturate(210%);
            -webkit-backdrop-filter: blur(30px) saturate(210%);
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
        <div id="ohc-help-input-area">
            <input type="text" id="ohc-help-input" placeholder="Ask me anything..." />
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
