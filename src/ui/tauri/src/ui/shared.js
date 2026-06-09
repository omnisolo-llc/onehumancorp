function initTooltips() {
  const tooltipEl = document.createElement('div');
  tooltipEl.className = 'ohc-tooltip';
  document.body.appendChild(tooltipEl);

  let hideTimeout;

  function showTooltip(e, text) {
    clearTimeout(hideTimeout);
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
    if (target) showTooltip(e, target.getAttribute('data-tooltip'));
  });

  document.addEventListener('mouseout', (e) => {
    if (e.target.closest('[data-tooltip]')) hideTooltip();
  });

  let touchTimeout;
  document.addEventListener('touchstart', (e) => {
    const target = e.target.closest('[data-tooltip]');
    if (target) {
      touchTimeout = setTimeout(() => {
        showTooltip(e, target.getAttribute('data-tooltip'));
      }, 500);
    }
  });

  document.addEventListener('touchend', () => {
    clearTimeout(touchTimeout);
    hideTooltip();
  });
}

function initHelpChat() {
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

  btn.addEventListener('click', () => { overlay.style.display = 'flex'; input.focus(); });
  closeBtn.addEventListener('click', () => { overlay.style.display = 'none'; });

  async function sendMessage() {
    const text = input.value.trim();
    if (!text) return;
    const uMsg = document.createElement('div');
    uMsg.className = 'msg msg-user';
    uMsg.textContent = text;
    messages.appendChild(uMsg);
    input.value = '';
    messages.scrollTop = messages.scrollHeight;
    setTimeout(() => {
      const aiMsg = document.createElement('div');
      aiMsg.className = 'msg msg-ai';
      if (text.toLowerCase().includes('credit card') || text.toLowerCase().includes('pay')) {
        aiMsg.innerHTML = `You can accept credit cards by connecting your bank account in the Payments section.<br/><a href="help_article.html?id=payments">Read the full article →</a>`;
      } else {
        aiMsg.innerHTML = `I can help you with that! Check out our Help Center for a detailed guide.<br/><a href="help.html">Read the full article →</a>`;
      }
      messages.appendChild(aiMsg);
      messages.scrollTop = messages.scrollHeight;
    }, 800);
  }
  sendBtn.addEventListener('click', sendMessage);
  input.addEventListener('keydown', (e) => { if (e.key === 'Enter') sendMessage(); });
}

document.addEventListener('DOMContentLoaded', () => {
  initTooltips();
  initHelpChat();
});
