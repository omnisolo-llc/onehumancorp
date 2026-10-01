const EPOCH_KEY = 'omnisolo_queue_identity_epoch_v2';
const PREFIX = 'omnisolo_invite_creation_v1:';
const sameOwner = (a, b) => a?.userId === b?.userId && a?.tenantId === b?.tenantId;
function confirmedLink(body) {
  if (!body || body.error != null || body.success === false || typeof body.invite_link !== 'string') return null;
  try {
    const url = new URL(body.invite_link);
    if (url.protocol !== 'https:' || !['omnisolo.co', 'cloud.omnisolo.co'].includes(url.hostname) || url.port || url.username || url.password || url.search || url.hash || !/^\/invite\/[^/]+$/.test(url.pathname) || /\/(fallback|default)$/.test(url.pathname)) return null;
    return body.invite_link;
  } catch { return null; }
}
export function installInviteBridge(view = window) {
  const document = view.document;
  const generate = document.getElementById('generate-link-btn');
  const container = document.getElementById('link-container');
  const input = document.getElementById('referral-link');
  const copy = document.getElementById('copy-btn');
  const whatsapp = document.getElementById('share-whatsapp-btn');
  const twitter = document.getElementById('share-x-btn') || document.getElementById('share-twitter-btn');
  if (!generate || !container || !input || !copy || generate.dataset.inviteBridgeBound) return;
  generate.dataset.inviteBridgeBound = 'true';
  let status = document.getElementById('invite-link-status');
  if (!status) {
    status = document.createElement('p'); status.id = 'invite-link-status'; status.setAttribute('role', 'status');
    generate.insertAdjacentElement('afterend', status);
  }
  let owner = null; let epoch = 0; let busy = false; let link = ''; let copyVersion = 0; let expiry;
  const controls = [copy, whatsapp, twitter].filter(Boolean);
  const keyFor = value => PREFIX + encodeURIComponent(JSON.stringify([value.userId, value.tenantId]));
  const note = message => { status.textContent = message; };
  const available = () => !!owner && owner.expiresAt > Date.now();
  const shareText = () => `Join my team on OmniSolo OneHumanCorp! Here is your invite link:\n\n${link}\n\n⚡ OmniSolo`;
  function clear() {
    epoch += 1; copyVersion += 1; view.clearTimeout(expiry); owner = null; link = ''; busy = false;
    input.value = ''; container.style.display = 'none'; generate.style.display = ''; generate.disabled = true;
    copy.textContent = 'Copy'; for (const control of controls) control.disabled = true;
  }
  function invalidate() { clear(); note('Your session changed. Reload to verify invitation access.'); }
  function readMarker(key, expected) {
    const bytes = view.localStorage.getItem(key);
    if (bytes === null) return null;
    const value = JSON.parse(bytes);
    if (value?.version !== 1 || !sameOwner(value.owner, expected) || typeof value.operation !== 'string' || !value.operation || !['pending', 'created'].includes(value.state)) throw new Error('Invite history is unavailable');
    return value;
  }
  async function identity() {
    if (!['https:', 'http:'].includes(view.location.protocol)) throw new Error('Signed-in application required');
    const response = await view.fetch('/api/v1/auth/session-identity', { credentials: 'same-origin', cache: 'no-store', redirect: 'error' });
    const value = await response.json();
    if (response.status !== 200 || value?.error != null || value?.success === false || typeof value?.userId !== 'string' || !value.userId || typeof value?.tenantId !== 'string' || !value.tenantId || !Number.isFinite(value.expiresAt) || value.expiresAt <= Date.now()) throw new Error('Identity unavailable');
    return value;
  }
  async function start() {
    const expected = epoch; note('Verifying invitation access…');
    try {
      const verified = await identity(); if (expected !== epoch) return;
      owner = verified; expiry = view.setTimeout(invalidate, Math.min(verified.expiresAt - Date.now(), 2147483647));
      if (!view.navigator.locks?.request) throw new Error('Coordinated invitation requests are unavailable');
      const marker = readMarker(keyFor(owner), owner);
      if (marker) { note(marker.state === 'created' ? 'An invitation was already created in this browser. Review existing invitations before creating another.' : 'A previous invitation request is unconfirmed. Review existing invitations before trying again.'); return; }
      generate.disabled = false; note('Create one invitation link for this verified account.');
    } catch { if (expected === epoch) { generate.disabled = true; note('Invitation access or local request history is unavailable. No invitation was requested.'); } }
  }
  clear();
  const create = async () => {
    if (!available() || busy || generate.disabled) return;
    const intended = { ...owner }; const expected = epoch; const key = keyFor(intended);
    busy = true; generate.disabled = true; note('Requesting an invitation…');
    let dispatched = false;
    try {
      await view.navigator.locks.request(key, { mode: 'exclusive', ifAvailable: true }, async lock => {
        if (!lock) throw new Error('Another invitation request is active');
        const verified = await identity();
        if (expected !== epoch || !sameOwner(verified, intended)) { invalidate(); return; }
        if (readMarker(key, intended)) throw new Error('Previous outcome must be reviewed');
        const headers = new view.Headers({ 'content-type': 'application/json', 'x-ohc-expected-user': intended.userId, 'x-ohc-expected-tenant': intended.tenantId });
        if (headers.get('x-ohc-expected-user') !== intended.userId || headers.get('x-ohc-expected-tenant') !== intended.tenantId) throw new Error('Identity cannot be bound');
        const marker = { version: 1, owner: { userId: intended.userId, tenantId: intended.tenantId }, operation: view.crypto.randomUUID(), state: 'pending' };
        view.localStorage.setItem(key, JSON.stringify(marker));
        if (expected !== epoch) return;
        dispatched = true;
        const response = await view.fetch('/api/v1/growth/cloud-bridge/invite', { method: 'POST', headers, body: JSON.stringify({ invitee_id: 'pending' }), credentials: 'same-origin', cache: 'no-store', redirect: 'error' });
        const body = await response.json();
        const receipt = response.status === 200 ? confirmedLink(body) : null;
        if (!receipt) throw new Error('Invitation was not confirmed');
        const current = readMarker(key, intended);
        if (current?.operation !== marker.operation || current.state !== 'pending') throw new Error('Invitation history changed');
        // Retain only replay-prevention metadata, never an invitation capability
        // URL. A reload cannot manufacture a new receipt or automatically retry.
        view.localStorage.setItem(key, JSON.stringify({ ...marker, state: 'created' }));
        if (expected !== epoch || !sameOwner(owner, intended)) return;
        link = receipt; input.value = receipt; container.style.display = 'block'; generate.style.display = 'none';
        for (const control of controls) control.disabled = false;
        note('Invitation link created. Sharing it does not confirm that anyone joined.');
      });
    } catch {
      if (expected === epoch) note(dispatched ? 'The invitation result could not be confirmed or saved. Review existing invitations before trying again.' : 'Invitation creation is held. Verify your session and review any earlier request before trying again.');
    } finally { if (expected === epoch) busy = false; }
  };
  const copyLink = async () => {
    if (!available() || !link || copy.disabled) return;
    const expected = epoch; const operation = ++copyVersion; const captured = link;
    copy.disabled = true; copy.textContent = 'Copying…';
    try {
      await view.navigator.clipboard.writeText(shareText());
      if (expected === epoch && operation === copyVersion && captured === link) { copy.textContent = 'Copied!'; note('Invitation text copied.'); }
    } catch { if (expected === epoch && operation === copyVersion) { copy.textContent = 'Copy'; note('Copy failed. Select the recorded link and copy it manually.'); } }
    finally { if (expected === epoch && operation === copyVersion) copy.disabled = false; }
  };
  const share = provider => {
    if (!available() || !link) return;
    const url = new URL(provider === 'whatsapp' ? 'https://wa.me/' : 'https://twitter.com/intent/tweet');
    url.searchParams.set('text', shareText());
    try { view.open(url.href, '_blank', 'noopener,noreferrer'); note('Share draft requested. Sending or joining is not verified here.'); }
    catch { note('The share draft could not be opened.'); }
  };
  const storageChanged = event => {
    if (event.key === null || event.key === EPOCH_KEY) invalidate();
    else if (owner && event.key === keyFor(owner) && !busy && !link) { generate.disabled = true; note('An invitation request changed in another view. Review existing invitations before retrying.'); }
  };
  generate.addEventListener('click', create); copy.addEventListener('click', copyLink);
  whatsapp?.addEventListener('click', () => share('whatsapp')); twitter?.addEventListener('click', () => share('twitter'));
  view.addEventListener('omnisolo_auth_changed', invalidate); view.addEventListener('storage', storageChanged); view.addEventListener('pagehide', invalidate);
  void start();
}
if (typeof window !== 'undefined') {
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', () => installInviteBridge(), { once: true });
  else installInviteBridge();
}
