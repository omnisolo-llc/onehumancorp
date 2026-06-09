document.addEventListener('DOMContentLoaded', () => {
    // Tooltips
    const tooltipEl = document.createElement('div');
    tooltipEl.className = 'ohc-tooltip';
    document.body.appendChild(tooltipEl);

    let hideTimeout;

    // Implement Tooltip Registry: fetch tooltips from backend
    let tooltipRegistry = {};
    async function loadTooltipRegistry() {
        try {
            const baseUrl = window.__TAURI__ ? 'http://127.0.0.1:18789' : '';
            const res = await fetch(`${baseUrl}/api/tooltips`);
            if (res.ok) {
                tooltipRegistry = await res.json();
            }
        } catch (e) {
            console.error('Failed to load tooltip registry', e);
        }
    }
    loadTooltipRegistry();

    function showTooltip(e, defaultText, id) {
        clearTimeout(hideTimeout);
        let text = defaultText;
        if (id && tooltipRegistry[id]) {
            text = tooltipRegistry[id];
        }
        tooltipEl.textContent = text;

        const targetRect = e.target.getBoundingClientRect();

        let top = targetRect.bottom + 10;
        let left = targetRect.left + (targetRect.width / 2) - (tooltipEl.offsetWidth / 2);

        if (left < 10) left = 10;
        if (left + tooltipEl.offsetWidth > window.innerWidth - 10) {
            left = window.innerWidth - tooltipEl.offsetWidth - 10;
        }

        if (top + tooltipEl.offsetHeight > window.innerHeight - 10) {
            top = targetRect.top - tooltipEl.offsetHeight - 10;
        }

        tooltipEl.style.top = `${top}px`;
        tooltipEl.style.left = `${left}px`;
        tooltipEl.classList.add('visible');
    }

    function hideTooltip() {
        hideTimeout = setTimeout(() => {
            tooltipEl.classList.remove('visible');
        }, 100);
    }

    document.addEventListener('mouseover', (e) => {
        const target = e.target.closest('[data-tooltip]');
        if (target) {
            showTooltip(e, target.getAttribute('data-tooltip'), target.id || target.getAttribute('data-tooltip-id'));
        }
    });

    document.addEventListener('mouseout', (e) => {
        if (e.target.closest('[data-tooltip]')) {
            hideTooltip();
        }
    });

    let touchTimeout;
    document.addEventListener('touchstart', (e) => {
        const target = e.target.closest('[data-tooltip]');
        if (target) {
            touchTimeout = setTimeout(() => {
                showTooltip(e, target.getAttribute('data-tooltip'), target.id || target.getAttribute('data-tooltip-id'));
            }, 500);
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

    // Chat Interface
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

        const uMsg = document.createElement('div');
        uMsg.className = 'msg msg-user';
        uMsg.textContent = text;
        messages.appendChild(uMsg);
        input.value = '';
        messages.scrollTop = messages.scrollHeight;

        try {
            const baseUrl = window.__TAURI__ ? 'http://127.0.0.1:18789' : '';
            const response = await fetch(`${baseUrl}/api/chat`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ message: text })
            });

            const data = await response.json();
            const aiMsg = document.createElement('div');
            aiMsg.className = 'msg msg-ai';

            let htmlContent = data.reply || "I'm having trouble connecting right now.";
            if (data.link) {
                htmlContent += `<br/><a href="${data.link.url}">${data.link.title || 'Read the full article →'}</a>`;
            }

            aiMsg.innerHTML = htmlContent;
            messages.appendChild(aiMsg);
            messages.scrollTop = messages.scrollHeight;
        } catch (error) {
            console.error('Chat error:', error);
            const errorMsg = document.createElement('div');
            errorMsg.className = 'msg msg-ai';
            errorMsg.textContent = "Sorry, I'm having trouble connecting right now.";
            messages.appendChild(errorMsg);
            messages.scrollTop = messages.scrollHeight;
        }
    }

    sendBtn.addEventListener('click', sendMessage);
    input.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') sendMessage();
    });

    // Walkthrough logic
    let walkthroughs = {
        'store_setup': [
            { target: 'generate-link-btn', text: 'Step 1: Set up your store.' },
            { target: 'share-whatsapp-btn', text: 'Step 2: Share it with your friends.' }
        ]
    };

    // Attempt to load walkthroughs from backend
    async function loadWalkthroughs() {
        try {
            const baseUrl = window.__TAURI__ ? 'http://127.0.0.1:18789' : '';
            const res = await fetch(`${baseUrl}/api/walkthroughs`);
            if (res.ok) {
                walkthroughs = await res.json();
            }
        } catch (e) {
            console.error('Failed to load walkthroughs', e);
        }
    }
    loadWalkthroughs();

    let currentWalkthrough = null;
    let currentStep = 0;

    window.startWalkthrough = function(name) {
        if (!walkthroughs[name]) return;
        currentWalkthrough = name;
        currentStep = 0;
        renderWalkthroughStep();
    }

    function renderWalkthroughStep() {
        const oldBubble = document.getElementById('ohc-walkthrough-bubble');
        if (oldBubble) oldBubble.remove();

        const steps = walkthroughs[currentWalkthrough];
        if (currentStep >= steps.length) {
            currentWalkthrough = null;
            return; // done
        }

        const step = steps[currentStep];
        const target = document.getElementById(step.target) || document.querySelector(step.target);
        if (!target) {
            currentStep++;
            renderWalkthroughStep();
            return;
        }

        const rect = target.getBoundingClientRect();

        const bubble = document.createElement('div');
        bubble.id = 'ohc-walkthrough-bubble';
        bubble.style.position = 'fixed';
        bubble.style.top = (rect.bottom + 10) + 'px';
        bubble.style.left = rect.left + 'px';
        bubble.style.background = '#0066FF';
        bubble.style.color = 'white';
        bubble.style.padding = '15px';
        bubble.style.borderRadius = '10px';
        bubble.style.zIndex = '10000';
        bubble.style.boxShadow = '0 4px 14px rgba(0, 102, 255, 0.4)';
        bubble.innerHTML = `
            <div style="margin-bottom: 10px;">${step.text}</div>
            <button id="ohc-walkthrough-next" style="background: white; color: #0066FF; border: none; padding: 5px 10px; border-radius: 5px; cursor: pointer;">Next</button>
        `;
        document.body.appendChild(bubble);

        document.getElementById('ohc-walkthrough-next').addEventListener('click', () => {
            currentStep++;
            renderWalkthroughStep();
        });
    }
});
