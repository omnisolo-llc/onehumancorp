import { auditDocumentsHandle, auditElementHandles, evaluateAuditDocuments, evaluateAuditElements } from './ui_audit_documents';
export type ClickEffects = {
  changed: boolean;
  requestSeen: boolean;
  downloadSeen: boolean;
  fileChooserSeen: boolean;
  popupSeen: boolean;
  validationSeen: boolean;
  dialogSeen: boolean;
  decisionSeen: boolean;
  selectionRestored?: boolean;
  focusSeen?: boolean;
};

export function hasMeaningfulClickEffect(effect: ClickEffects): boolean {
  return effect.selectionRestored !== false && (effect.changed || effect.requestSeen || effect.downloadSeen
    || effect.fileChooserSeen || effect.popupSeen || effect.validationSeen || effect.decisionSeen || effect.focusSeen === true);
}

import type { Dialog, Download, ElementHandle, FileChooser, JSHandle, Page, Request } from '@playwright/test';

// Hover help is preparation for a click, not evidence that the click worked.
// Strip only semantically identified tooltips, not status messages or dialogs.
export function auditDocumentSignature(documents: Document[] = [document]) {
  return documents.map(document => {
  const body = document.body.cloneNode(true) as HTMLElement;
  body.querySelectorAll('[role="tooltip"], .omnisolo-tooltip').forEach((tooltip) => tooltip.remove());
  body.querySelectorAll('[data-ui-audit-click-index], [data-ui-audit-click-key]').forEach((element) => {
    element.removeAttribute('data-ui-audit-click-index');
    element.removeAttribute('data-ui-audit-click-key');
  });
  const checksum = (value: string) => {
    let hash = 0;
    for (let index = 0; index < value.length; index += 1) hash = ((hash << 5) - hash + value.charCodeAt(index)) | 0;
    return hash;
  };
  return `${document.location.href}|${checksum(body.textContent || '')}|${checksum(body.innerHTML)}|${body.querySelectorAll('*').length}`;
  }).join('\n');
}

export async function auditPageSignature(page: Page) {
  return evaluateAuditDocuments(page, auditDocumentSignature).catch(() => page.url());
}

async function waitForClickEffect(page: Page, beforeUrl: string, beforeSignature: string) {
  for (let attempt = 0; attempt < 6; attempt += 1) {
    await page.waitForTimeout(50);
    const afterUrl = page.url();
    const afterSignature = await auditPageSignature(page);
    if (afterUrl !== beforeUrl || afterSignature !== beforeSignature) {
      return { afterUrl, afterSignature, changed: true };
    }
  }

  return { afterUrl: page.url(), afterSignature: await auditPageSignature(page), changed: false };
}


async function prepareExclusiveChoice(target: ElementHandle<HTMLElement | SVGElement>): Promise<string | undefined> {
  const attribute = await target.evaluate((element) =>
    ['aria-pressed', 'aria-selected'].find((name) => element.getAttribute(name) === 'true'));
  if (!attribute) return undefined;
  const handle = await target.evaluateHandle((element, name) => {
    const siblings = Array.from(element.parentElement?.children || []).filter((sibling) =>
      sibling.tagName === 'BUTTON' || ['button', 'tab'].includes(sibling.getAttribute('role') || ''));
    if (siblings.length < 2 || siblings.some((sibling) => !['true', 'false'].includes(sibling.getAttribute(name) || ''))) return null;
    return siblings.find((sibling) => {
      const bounds=sibling.getBoundingClientRect(); const style=getComputedStyle(sibling);
      return sibling !== element && sibling.getAttribute(name) === 'false'
        && !sibling.hasAttribute('disabled') && sibling.getAttribute('aria-disabled') !== 'true'
        && bounds.width > 0 && bounds.height > 0 && style.display !== 'none' && style.visibility !== 'hidden';
    }) || null;
  }, attribute);
  try {
    const alternate=handle.asElement();
    if (!alternate) return undefined;
    await alternate.click({ timeout: 5000 });
    const state=await target.evaluate((element,name)=>({ connected:element.isConnected, deselected:element.getAttribute(name)==='false' }),attribute);
    if (!state.connected) throw new Error('Audit target detached during alternate-choice preparation');
    // Independent toggles are not choices: clicking their neighbor does not
    // deselect this target. They still have to produce their own click effect.
    return state.deselected ? attribute : undefined;
  } finally { await handle.dispose(); }
}

// Observe only a focus transition produced during this same trusted click.
// Hover/button preparation and later timers are outside the event interval.
export function installClickFocusProbe(element: HTMLElement | SVGElement) {
  const document = element.ownerDocument;
  const view = document.defaultView!;
  let clicked: MouseEvent | undefined;
  let before: Element | null = null;
  let focusSeen = false;
  const capture = (event: MouseEvent) => {
    if (!event.isTrusted || !event.composedPath().includes(element)) return;
    clicked = event;
    before = document.activeElement;
  };
  const bubble = (event: MouseEvent) => {
    if (event !== clicked || !event.isTrusted) return;
    const focused = document.activeElement;
    if (focused === before || focused === element || focused === document.body
        || focused?.nodeType !== 1) return;
    if (!(['INPUT', 'TEXTAREA', 'SELECT'].includes(focused.tagName) || (focused as HTMLElement).isContentEditable)
        || focused.matches(':disabled') || focused.closest('[hidden], [inert], [aria-hidden="true"]')) return;
    const bounds = focused.getBoundingClientRect();
    if (bounds.width <= 0 || bounds.height <= 0) return;
    for (let current: Element | null = focused; current; current = current.parentElement) {
      const style = getComputedStyle(current);
      if (style.display === 'none' || style.visibility === 'hidden' || style.visibility === 'collapse'
          || Number(style.opacity) === 0) return;
    }
    focusSeen = true;
  };
  view.addEventListener('click', capture, true);
  view.addEventListener('click', bubble);
  return {
    get focusSeen() { return focusSeen; },
    dispose() {
      view.removeEventListener('click', capture, true);
      view.removeEventListener('click', bubble);
    },
  };
}

// Playwright exposes a popup Page only after its initial response starts. A
// slow destination must not erase a real navigation from this trusted click.
export function installClickPopupProbe(element: HTMLElement | SVGElement) {
  const document = element.ownerDocument;
  const view = document.defaultView!;
  const originalOpen = view.open;
  const closedGetter = Object.getOwnPropertyDescriptor(view, 'closed')?.get;
  const nativeOpen = /\{\s*\[native code\]\s*\}/.test(Function.prototype.toString.call(originalOpen));
  const opened: Array<{ popup: Window; url?: string }> = [];
  let clicked: MouseEvent | undefined;
  const capture = (event: MouseEvent) => {
    if (event.isTrusted && event.composedPath().includes(element)) clicked = event;
  };
  const isLiveWindow = (popup: Window) => {
    // Native getter branding rejects fake { closed: false } return values and
    // works for real child WindowProxies across JavaScript realms.
    try { return popup !== view && closedGetter?.call(popup) === false; } catch { return false; }
  };
  const observeOpen: Window['open'] = function (this: Window, ...args) {
    const trusted = nativeOpen && clicked?.isTrusted && view.event === clicked;
    const popup = Reflect.apply(originalOpen, this, args) as Window | null;
    const [url, name] = args;
    // Never take ownership of _self/_parent/_top or a reused named window.
    if (nativeOpen && popup
        && (name === undefined || name === '' || (typeof name === 'string' && name.toLowerCase() === '_blank'))
        && isLiveWindow(popup)) {
      let destinationUrl: string | undefined;
      if (trusted && typeof url === 'string' && url.trim().length > 0) {
        try {
          const destination = new URL(url, document.baseURI);
          if (['http:', 'https:'].includes(destination.protocol)) {
            destination.hash = '';
            destinationUrl = destination.href;
          }
        } catch { /* The native call retains its normal result for invalid URLs. */ }
      }
      // Also retire empty/late windows without crediting their calls as effects.
      opened.push({ popup, url: destinationUrl });
    }
    return popup;
  };
  view.addEventListener('click', capture, true);
  view.open = observeOpen;
  return {
    get destinations() { return opened.flatMap(({ popup, url }) => url && isLiveWindow(popup) ? [url] : []); },
    dispose() {
      view.removeEventListener('click', capture, true);
      if (view.open === observeOpen) view.open = originalOpen;
      // Pending windows do not yet exist in Playwright's page list.
      for (const { popup } of opened) if (isLiveWindow(popup)) popup.close();
    },
  };
}

export async function observeClickEffects(page: Page, target: ElementHandle<HTMLElement | SVGElement>): Promise<ClickEffects> {
  const selectionAttribute = await prepareExclusiveChoice(target);
  await target.hover({ timeout: 5000 });
  await target.focus();
  const beforeUrl = page.url();
  const beforeSignature = await auditPageSignature(page);
  const observed: ClickEffects = { changed: false, requestSeen: false, downloadSeen: false,
    fileChooserSeen: false, popupSeen: false, validationSeen: false, dialogSeen: false, decisionSeen: false, focusSeen: false };
  const popups: Page[] = [];
  const initialPopupRequests = new Set<string>();
  const pending: Promise<unknown>[] = [];
  const onDialog = async (dialog: Dialog) => {
    observed.dialogSeen = true;
    // A user decision is an observable interaction, not a completed mutation.
    // Never approve destructive actions or provide prompt data in this crawl.
    observed.decisionSeen ||= ['confirm', 'prompt'].includes(dialog.type());
    await dialog.dismiss().catch(() => undefined);
  };
  const onRequest = (request: Request) => {
    if (request.isNavigationRequest() || ['fetch', 'xhr'].includes(request.resourceType())) observed.requestSeen = true;
  };
  const onContextRequest = (request: Request) => {
    if (!request.isNavigationRequest()) return;
    try { request.frame(); } catch {
      // Initial popup requests have no frame until the first response. Only a
      // matching live window from the exact trusted click can claim this URL.
      initialPopupRequests.add(request.url());
    }
  };
  const onDownload = (download: Download) => { observed.downloadSeen = Boolean(download.suggestedFilename()); };
  const onFileChooser = (chooser: FileChooser) => {
    observed.fileChooserSeen = true;
    pending.push(chooser.setFiles([]));
  };
  const onPopup = (popup: Page) => {
    popups.push(popup);
    pending.push((async () => {
      await popup.waitForLoadState('domcontentloaded', { timeout: 2000 }).catch(() => undefined);
      const url = popup.url();
      const hasDocument = (url !== '' && url !== 'about:blank')
        || Boolean(await popup.locator('body').textContent({ timeout: 500 }).catch(() => ''));
      observed.popupSeen ||= !popup.isClosed() && hasDocument;
    })());
  };
  await target.evaluate(element => {
    const document = element.ownerDocument;
    const state = document.defaultView as Window & { __uiAuditInvalid?: boolean };
    state.__uiAuditInvalid = false;
    document.addEventListener('invalid', (event) => {
      const input = event.target;
      if (input instanceof document.defaultView!.HTMLInputElement || input instanceof document.defaultView!.HTMLTextAreaElement || input instanceof document.defaultView!.HTMLSelectElement) {
        if (!input.validity.valid && input.validationMessage) state.__uiAuditInvalid = true;
      }
    }, { capture: true, once: true });
  });
  page.on('dialog', onDialog);
  page.on('request', onRequest);
  page.on('download', onDownload);
  page.on('filechooser', onFileChooser);
  page.on('popup', onPopup);
  page.context().on('request', onContextRequest);
  let popupProbe: JSHandle<ReturnType<typeof installClickPopupProbe>> | undefined;
  let focusProbe: JSHandle<ReturnType<typeof installClickFocusProbe>> | undefined;
  try {
    popupProbe = await target.evaluateHandle(installClickPopupProbe);
    focusProbe = await target.evaluateHandle(installClickFocusProbe);
    // A real user gesture is required by clipboard, popup and file APIs.
    await target.click({ timeout: 5000 });
    const effect = await waitForClickEffect(page, beforeUrl, beforeSignature);
    observed.changed = effect.changed;
    const popupDestinations = await popupProbe.evaluate(probe => probe.destinations).catch(() => [] as string[]);
    observed.requestSeen ||= popupDestinations.some(url => initialPopupRequests.has(url));
    observed.focusSeen = await focusProbe.evaluate(probe => probe.focusSeen).catch(() => false);
    observed.validationSeen = await target.evaluate(element => Boolean((element.ownerDocument.defaultView as Window & { __uiAuditInvalid?: boolean }).__uiAuditInvalid)).catch(() => false);
    await Promise.all(pending);
    if (selectionAttribute) {
      observed.selectionRestored=await target.evaluate((element,attribute)=>element.isConnected && element.getAttribute(attribute)==='true',selectionAttribute).catch(()=>false);
    }
    return observed;
  } finally {
    page.off('dialog', onDialog);
    page.off('request', onRequest);
    page.off('download', onDownload);
    page.off('filechooser', onFileChooser);
    page.off('popup', onPopup);
    page.context().off('request', onContextRequest);
    if (popupProbe) {
      await popupProbe.evaluate(probe => probe.dispose()).catch(() => undefined);
      await popupProbe.dispose();
    }
    if (focusProbe) {
      await focusProbe.evaluate(probe => probe.dispose()).catch(() => undefined);
      await focusProbe.dispose();
    }
    for (const popup of popups) await popup.close().catch(() => undefined);
  }
}

// Retire the old document before another route audit starts. A delayed checkout
// navigation from that document must not replace the next document under test.
export async function replaceAuditDocument(page: Page): Promise<Page> {
  const retiredUrl = page.url();
  const viewport = page.viewportSize();
  // A committed full-document navigation destroys the old JavaScript realm,
  // including delayed callbacks, while preserving one page/video per route.
  try {
    await page.goto('about:blank', { waitUntil: 'load' });
  } catch (error) {
    let logoutInterruption: string | undefined;
    try {
      const source = new URL(retiredUrl);
      if (['http:', 'https:'].includes(source.protocol) && !source.username && !source.password) {
        logoutInterruption = `page.goto: Navigation to "about:blank" is interrupted by another navigation to "${source.origin}/login"`;
      }
    } catch { /* An unclassified source cannot establish the logout boundary. */ }
    if (!(error instanceof Error) || error.message.split('\n', 1)[0] !== logoutInterruption) throw error;
    // Logout may replace the document after its real click effect was observed.
    // Retire that realm without retrying either the click or the navigation.
    // Keeping the context retains cookies and its recording of both pages.
    const context = page.context();
    await page.close({ runBeforeUnload: false });
    const replacement = await context.newPage();
    if (viewport) await replacement.setViewportSize(viewport);
    return replacement;
  }
  return page;
}

export async function resolveAuditTarget(
  page: Page,
  key: string,
  retag: () => Promise<Array<{ key: string; index: number; label: string }>>,
  timeout = 5000,
): Promise<ElementHandle<HTMLElement | SVGElement>> {
  const deadline = Date.now() + timeout;
  const url = page.url();
  let expired = false;
  let timer: ReturnType<typeof setTimeout> | undefined;
  const failure = () => new Error(`Audit target could not be stably resolved before any click: ${key}`);
  const expiry = new Promise<never>((_resolve, reject) => {
    timer = setTimeout(() => { expired = true; reject(failure()); }, timeout);
  });
  // Reacquisition is read-only. Once observation starts, even a detached target
  // remains a failure; this function never repeats a clicked business action.
  const locate = async () => {
    let documentHandle: JSHandle<Document[]> | undefined;
    let selected: ElementHandle<HTMLElement | SVGElement> | undefined;
    try {
      documentHandle = await auditDocumentsHandle(page);
      const sameDocument = () => evaluateAuditDocuments<boolean, Document[]>(page, (documents, original) => documents.length === original.length && documents.every((document, index) => document === original[index]), documentHandle!).catch(() => false);
      while (!expired && Date.now() < deadline) {
        if (page.url() !== url || !(await sameDocument())) throw new Error(`Audit document changed before inspecting ${key}`);
        const candidate = (await retag()).find((item) => item.key === key);
        if (expired) break;
        if (candidate) {
          const handles = await auditElementHandles(page, `[data-ui-audit-click-index="${candidate.index}"]`);
          if (expired) {
            await Promise.all(handles.map((handle) => handle.dispose()));
            break;
          }
          if (handles.length === 1) {
            const target = handles[0] as ElementHandle<HTMLElement | SVGElement>;
            let retained = false;
            try {
              await target.waitForElementState('stable', { timeout: Math.max(1, deadline - Date.now()) });
              const observedKey = await target.getAttribute('data-ui-audit-click-key');
              const connected = await target.evaluate((element) => element.isConnected);
              const identicalDocument = await sameDocument();
              if (!expired && Date.now() < deadline && page.url() === url && identicalDocument && observedKey === key && connected) {
                retained = true;
                selected = target;
                return target;
              }
            } catch {
              // Hydration may replace the node before any user action. Re-tag the
              // current DOM within the same bounded lookup budget.
            } finally {
              if (!retained) await target.dispose();
            }
          } else {
            await Promise.all(handles.map((handle) => handle.dispose()));
          }
        }
        const remaining = deadline - Date.now();
        if (!expired && remaining > 0) await page.waitForTimeout(Math.min(50, remaining));
      }
      throw failure();
    } finally {
      if (documentHandle) {
        try { await documentHandle.dispose(); } catch { /* A destroyed realm already releases its handles. */ }
      }
      if (expired && selected) {
        try { await selected.dispose(); } catch { /* Never hand back a late target. */ }
      }
    }
  };
  try {
    return await Promise.race([locate(), expiry]);
  } finally {
    if (timer !== undefined) clearTimeout(timer);
  }
}

// Browser-realm fragment validation used by the purpose/link contracts.
export function hasFragmentTarget(target: string | { href: string; embedded?: boolean }): boolean {
  const href = typeof target === 'string' ? target : target.href;
  const owner = typeof target === 'object' && target.embedded ? document.querySelector<HTMLIFrameElement>('iframe[data-ohc-api-docs-viewer]')?.contentDocument : document;
  if (!owner) return false;
  if (!href.startsWith('#') || href.length === 1) return false;
  let name: string;
  try { name = decodeURIComponent(href.slice(1)); } catch { return false; }
  return owner.getElementById(name) !== null
    || Array.from(owner.getElementsByName(name)).some(element => element.tagName === 'A');
}


export const clickableAuditSelector = [
  'button:visible:not([disabled])',
  '[role="button"]:visible:not([aria-disabled="true"])',
  '[onclick]:visible',
  'input[type="button"]:visible:not([disabled])',
  'input[type="submit"]:visible:not([disabled])',
  'input[type="reset"]:visible:not([disabled])',
  'summary:visible',
].join(', ');

export async function tagClickTargets(page: Page, ownerNamespace?: string, canonicalIds: Record<string, string> = {}) {
  return evaluateAuditElements(page, clickableAuditSelector, (elements, { namespace, identifiers }) => {
    const counts = new Map<string, number>();
    return elements.filter((element) => {
      const style = window.getComputedStyle(element);
      return !element.closest('[aria-hidden="true"], nextjs-portal')
        && style.pointerEvents !== 'none' && style.opacity !== '0';
    }).map((element, index) => {
      const label = element.getAttribute('aria-label') || (element.textContent || '').trim().replace(/\s+/g, ' ')
        || element.getAttribute('title') || element.id || element.tagName;
      // Case-owned rows receive new database IDs, but the same canonical
      // record retains its suffix. Include record ancestry so two Dismiss
      // buttons cannot swap identities when backend result ordering changes.
      const canonical = (value: string) => {
        let result = namespace ? value.split(namespace).join('audit-owner') : value;
        for (const [generated, original] of Object.entries(identifiers)) result = result.split(generated).join(original);
        return result;
      };
      const record = element.closest('[data-testid]')?.getAttribute('data-testid') || '';
      const identity = JSON.stringify(namespace
        ? [element.tagName, canonical(element.id), canonical(label), canonical(record)]
        : [element.tagName, element.id, label]);
      const occurrence = counts.get(identity) || 0;
      counts.set(identity, occurrence + 1);
      const frame = element.ownerDocument === document ? '' : 'api-docs-viewer|';
      const key = `${frame}${identity}:${occurrence}`;
      element.setAttribute('data-ui-audit-click-index', String(index));
      element.setAttribute('data-ui-audit-click-key', key);
      return { index, label, key };
    });
  }, { namespace: ownerNamespace, identifiers: canonicalIds });
}
