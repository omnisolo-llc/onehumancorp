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

// Runs in the browser realm. Use the native setter, not React's instance value
// tracker: dispatching after assigning control.value otherwise leaves React
// state empty even though the DOM appears filled.
export function fillEmptyAuditControls() {
  for (const control of document.querySelectorAll<HTMLInputElement | HTMLTextAreaElement>('input, textarea')) {
    const style = window.getComputedStyle(control);
    const rect = control.getBoundingClientRect();
    if (style.visibility === 'hidden' || style.display === 'none' || !rect.width || !rect.height
        || control.disabled || control.readOnly || control.value) continue;
    let value = 'Audit value';
    if (control instanceof HTMLInputElement) {
      if (['button', 'checkbox', 'color', 'file', 'hidden', 'image', 'radio', 'range', 'reset', 'submit'].includes(control.type)) continue;
      const values: Record<string, string> = {
        url: 'https://example.test', email: 'ui-audit@example.test', tel: '+15550101000',
        number: String(Math.max(Number(control.min) || 0, 1)), date: '2026-10-01',
        'datetime-local': '2026-10-01T12:00', time: '12:00', month: '2026-10', week: '2026-W40',
      };
      value = values[control.type] ?? value;
    }
    const prototype = control instanceof HTMLInputElement ? HTMLInputElement.prototype : HTMLTextAreaElement.prototype;
    Object.getOwnPropertyDescriptor(prototype, 'value')!.set!.call(control, value);
    control.dispatchEvent(new Event('input', { bubbles: true }));
    control.dispatchEvent(new Event('change', { bubbles: true }));
  }
}

import type { Dialog, Download, ElementHandle, FileChooser, Page, Request } from '@playwright/test';

// Hover help is preparation for a click, not evidence that the click worked.
// Strip only semantically identified tooltips, not status messages or dialogs.
export function auditDocumentSignature() {
  const body = document.body.cloneNode(true) as HTMLElement;
  body.querySelectorAll('[role="tooltip"], .omnisolo-tooltip').forEach((tooltip) => tooltip.remove());
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
  const context = page.context();
  await page.close();
  return context.newPage();
}
