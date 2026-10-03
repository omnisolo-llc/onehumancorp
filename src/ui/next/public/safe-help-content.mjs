// Remote help content is data, never HTML, inline handlers or executable URLs.
function safeUrl(value, document) {
  if (typeof value !== 'string' || !value.trim() || value.length > 4096) return null;
  try {
    const url = new URL(value, document.baseURI);
    const origin = new URL(document.baseURI).origin;
    if (url.username || url.password) return null;
    if (url.protocol !== 'https:' && !(url.protocol === 'http:' && url.origin === origin)) return null;
    return url.href;
  } catch { return null; }
}

export function renderHelpMessage(node, text, link) {
  const document = node.ownerDocument;
  node.textContent = typeof text === 'string' ? text : 'Help response unavailable.';
  const url = safeUrl(link?.url, document);
  if (!url || typeof link?.title !== 'string' || !link.title.trim()) return;
  const wrapper = document.createElement('div');
  wrapper.style.marginTop = '8px';
  const anchor = document.createElement('a');
  anchor.href = url;
  anchor.rel = 'noopener noreferrer';
  const title = link.title.trim();
  anchor.textContent = title.startsWith('Read the full article') || title.includes('→') ? title : `Read the full article: ${title} →`;
  anchor.style.cssText = 'display:inline-block;padding:8px 12px;min-height:44px;color:#0066FF;border-radius:8px;font-size:13px;';
  wrapper.appendChild(anchor);
  node.appendChild(wrapper);
}

// Walkthrough content shares the same trust boundary as remote help messages.
// Return the actual controls, rather than resolving potentially duplicated IDs
// elsewhere in the document. Only our own fixed markup is parsed as HTML.
export function renderWalkthroughStep(bubble, step, index, count) {
  if (!step || typeof step !== 'object' || !Number.isInteger(index)
      || !Number.isInteger(count) || index < 0 || index >= count) {
    throw new TypeError('Invalid walkthrough step');
  }
  const title = typeof step.title === 'string' ? step.title.slice(0, 500) : 'Tour';
  const text = typeof step.content === 'string' ? step.content : step.text;
  bubble.setAttribute('aria-label', `${title} walkthrough step`);
  bubble.innerHTML = `
    <div style="display:flex;justify-content:space-between;align-items:center;border-bottom:1px solid #eee;padding-bottom:8px;margin-bottom:8px;">
      <h4 style="margin:0;font-size:16px;font-weight:bold;"></h4>
      <button type="button" id="wt-close" class="omnisolo-walkthrough-close" aria-label="Close walkthrough" style="background:none;border:none;cursor:pointer;font-size:18px;">&times;</button>
    </div>
    <p style="margin:0;font-size:14px;color:#333;"></p>
    <div data-controls style="display:flex;justify-content:flex-end;gap:8px;margin-top:8px;">
      <button type="button" id="wt-next" style="min-height:44px;display:inline-flex;align-items:center;justify-content:center;padding:6px 12px;border:none;border-radius:8px;background:#2563eb;color:white;"></button>
    </div>`;
  bubble.querySelector('h4').textContent = title;
  bubble.querySelector('p').textContent = typeof text === 'string' ? text.slice(0, 10000) : '';
  const next = bubble.querySelector('button[id="wt-next"]');
  next.textContent = index === count - 1 ? 'Finish' : 'Next';
  let previous = null;
  if (index > 0) {
    previous = bubble.ownerDocument.createElement('button');
    previous.id = 'wt-prev';
    previous.type = 'button';
    previous.className = 'glassmorphism';
    previous.textContent = 'Back';
    previous.style.cssText = 'min-height:44px;min-width:80px;padding:6px 12px;border-radius:8px;cursor:pointer;';
    next.before(previous);
  }
  return { close: bubble.querySelector('button[id="wt-close"]'), previous, next };
}

export function renderHelpVideos(container, entries) {
  if (!container) return;
  container.replaceChildren();
  if (!Array.isArray(entries)) throw new TypeError('Video list is unavailable');
  const document = container.ownerDocument;
  for (const entry of entries.slice(0, 100)) {
    const url = safeUrl(entry?.video_url, document);
    if (!url || typeof entry?.title !== 'string' || !entry.title.trim()) continue;
    const title = entry.title.slice(0, 500);
    const card = document.createElement('div');
    card.style.cssText = 'border:1px solid rgba(255,255,255,.4);border-radius:8px;padding:12px;';
    const heading = document.createElement('h4');
    heading.textContent = title;
    const play = document.createElement('button');
    play.type = 'button';
    play.textContent = `Play ${title}`;
    play.style.minHeight = '44px';
    play.addEventListener('click', () => {
      const video = document.createElement('video');
      video.controls = true;
      video.preload = 'metadata';
      video.src = url;
      video.setAttribute('aria-label', title);
      video.style.width = '100%';
      card.replaceChildren(heading, video);
    });
    card.append(heading, play);
    if (typeof entry.duration === 'string') {
      const duration = document.createElement('span');
      duration.textContent = entry.duration.slice(0, 80);
      card.appendChild(duration);
    }
    container.appendChild(card);
  }
}
