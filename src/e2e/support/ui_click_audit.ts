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
};

export function hasMeaningfulClickEffect(effect: ClickEffects): boolean {
  return effect.selectionRestored !== false && (effect.changed || effect.requestSeen || effect.downloadSeen
    || effect.fileChooserSeen || effect.popupSeen || effect.validationSeen || effect.decisionSeen);
}

import type { Dialog, Download, ElementHandle, FileChooser, JSHandle, Page, Request } from '@playwright/test';

// Hover help is preparation for a click, not evidence that the click worked.
// Strip only semantically identified tooltips, not status messages or dialogs.
export function auditDocumentSignature() {
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
  return `${location.href}|${checksum(body.textContent || '')}|${checksum(body.innerHTML)}|${body.querySelectorAll('*').length}`;
}

async function pageSignature(page: Page) {
  return page.evaluate(auditDocumentSignature).catch(() => page.url());
}

async function waitForClickEffect(page: Page, beforeUrl: string, beforeSignature: string) {
  for (let attempt = 0; attempt < 6; attempt += 1) {
    await page.waitForTimeout(50);
    const afterUrl = page.url();
    const afterSignature = await pageSignature(page);
    if (afterUrl !== beforeUrl || afterSignature !== beforeSignature) {
      return { afterUrl, afterSignature, changed: true };
    }
  }

  return { afterUrl: page.url(), afterSignature: await pageSignature(page), changed: false };
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

export async function observeClickEffects(page: Page, target: ElementHandle<HTMLElement | SVGElement>): Promise<ClickEffects> {
  const selectionAttribute = await prepareExclusiveChoice(target);
  await target.hover({ timeout: 5000 });
  await target.focus();
  const beforeUrl = page.url();
  const beforeSignature = await pageSignature(page);
  const observed: ClickEffects = { changed: false, requestSeen: false, downloadSeen: false,
    fileChooserSeen: false, popupSeen: false, validationSeen: false, dialogSeen: false, decisionSeen: false };
  const popups: Page[] = [];
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
  const onDownload = (download: Download) => { observed.downloadSeen = Boolean(download.suggestedFilename()); };
  const onFileChooser = (chooser: FileChooser) => {
    observed.fileChooserSeen = true;
    pending.push(chooser.setFiles([]));
  };
  const onPopup = (popup: Page) => {
    popups.push(popup);
    pending.push((async () => {
      await popup.waitForLoadState('domcontentloaded', { timeout: 2000 }).catch(() => undefined);
      observed.popupSeen ||= popup.url() !== 'about:blank' || Boolean(await popup.locator('body').textContent({ timeout: 500 }).catch(() => ''));
    })());
  };
  await page.evaluate(() => {
    const state = window as Window & { __uiAuditInvalid?: boolean };
    state.__uiAuditInvalid = false;
    document.addEventListener('invalid', (event) => {
      const input = event.target;
      if ((input instanceof HTMLInputElement || input instanceof HTMLTextAreaElement || input instanceof HTMLSelectElement)
          && !input.validity.valid && input.validationMessage) state.__uiAuditInvalid = true;
    }, { capture: true, once: true });
  });
  page.on('dialog', onDialog);
  page.on('request', onRequest);
  page.on('download', onDownload);
  page.on('filechooser', onFileChooser);
  page.on('popup', onPopup);
  try {
    // A real user gesture is required by clipboard, popup and file APIs.
    await target.click({ timeout: 5000 });
    const effect = await waitForClickEffect(page, beforeUrl, beforeSignature);
    observed.changed = effect.changed;
    observed.validationSeen = await page.evaluate(() => Boolean((window as Window & { __uiAuditInvalid?: boolean }).__uiAuditInvalid)).catch(() => false);
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
    for (const popup of popups) await popup.close().catch(() => undefined);
  }
}

// Retire the old document before another route audit starts. A delayed checkout
// navigation from that document must not replace the next document under test.
export async function replaceAuditDocument(page: Page): Promise<Page> {
  // A committed full-document navigation destroys the old JavaScript realm,
  // including delayed callbacks, while preserving one page/video per route.
  await page.goto('about:blank', { waitUntil: 'load' });
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
    let documentHandle: JSHandle<Document> | undefined;
    let selected: ElementHandle<HTMLElement | SVGElement> | undefined;
    try {
      documentHandle = await page.evaluateHandle(() => document);
      const sameDocument = () => page.evaluate((original) => original === document, documentHandle!).catch(() => false);
      while (!expired && Date.now() < deadline) {
        if (page.url() !== url || !(await sameDocument())) throw new Error(`Audit document changed before inspecting ${key}`);
        const candidate = (await retag()).find((item) => item.key === key);
        if (expired) break;
        if (candidate) {
          const handles = await page.locator(`[data-ui-audit-click-index="${candidate.index}"]`).elementHandles();
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
export function hasFragmentTarget(href: string): boolean {
  if (!href.startsWith('#') || href.length === 1) return false;
  let name: string;
  try { name = decodeURIComponent(href.slice(1)); } catch { return false; }
  return document.getElementById(name) !== null
    || Array.from(document.getElementsByName(name)).some(element => element.tagName === 'A');
}
