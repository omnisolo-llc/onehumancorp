import type { ElementHandle, JSHandle, Page } from '@playwright/test';

// This function runs in the browser realm. A visible documentation frame must
// remain the expected same-origin document; silently omitting it loses coverage.
export function collectAuditDocuments(): Document[] {
  const documents = [document];
  const frames = Array.from(document.querySelectorAll<HTMLIFrameElement>('iframe[data-ohc-api-docs-viewer]'));
  if (frames.length > 1) throw new Error('Ambiguous documentation frame inventory');
  for (const frame of frames) {
    const destination = new URL(frame.src, location.href);
    if (destination.origin !== location.origin || destination.pathname !== '/api-docs/viewer') throw new Error('Unexpected documentation frame destination');
    const child = frame.contentDocument;
    if (!child || child.location.origin !== location.origin || child.location.pathname !== '/api-docs/viewer') throw new Error('Documentation frame is not ready on its expected origin');
    if (!frame.getBoundingClientRect().width || !frame.getBoundingClientRect().height || getComputedStyle(frame).display === 'none' || getComputedStyle(frame).visibility === 'hidden') throw new Error('Documentation frame is unexpectedly hidden');
    documents.push(child);
  }
  return documents;
}

export function auditDocumentsHandle(page: Page): Promise<JSHandle<Document[]>> {
  return page.evaluateHandle(collectAuditDocuments);
}

export async function evaluateAuditDocuments<Result, Argument = undefined>(page: Page, evaluate: (documents: Document[], argument: Argument) => Result, argument?: Argument | JSHandle<Argument>): Promise<Result> {
  const documents = await auditDocumentsHandle(page);
  try { return await documents.evaluate(evaluate as Parameters<typeof documents.evaluate>[0], argument) as Result; }
  finally { await documents.dispose(); }
}

async function elementsHandle(page: Page, selector: string): Promise<JSHandle<Element[]>> {
  const documents = await auditDocumentsHandle(page);
  try {
    return await documents.evaluateHandle((documents, selector) => {
      const visible = selector.includes(':visible');
      const css = selector.replaceAll(':visible', '');
      return documents.flatMap(document => Array.from(document.querySelectorAll(css)).filter(element => {
        if (!visible) return true;
        const style = document.defaultView!.getComputedStyle(element);
        const bounds = element.getBoundingClientRect();
        return style.visibility !== 'hidden' && style.visibility !== 'collapse' && bounds.width > 0 && bounds.height > 0;
      }));
    }, selector);
  } finally { await documents.dispose(); }
}

export async function evaluateAuditElements<Result, Argument = undefined>(page: Page, selector: string, evaluate: (elements: Element[], argument: Argument) => Result, argument?: Argument | JSHandle<Argument>): Promise<Result> {
  const elements = await elementsHandle(page, selector);
  try { return await elements.evaluate(evaluate as Parameters<typeof elements.evaluate>[0], argument) as Result; }
  finally { await elements.dispose(); }
}

export async function auditElementHandles(page: Page, selector: string): Promise<ElementHandle<Node>[]> {
  const count = await evaluateAuditDocuments(page, documents => documents.length);
  const result = await page.locator(selector).elementHandles();
  try {
    // Obtain handles through their owning Playwright frame so trusted clicks,
    // focus and scrolling use the correct browser execution context.
    if (count === 2) result.push(...await page.frameLocator('iframe[data-ohc-api-docs-viewer]').locator(selector).elementHandles());
    return result;
  } catch (error) {
    await Promise.all(result.map(element => element.dispose()));
    throw error;
  }
}

// Browser-realm layout measurements retain every within-document comparison and
// also compare visible embedded controls with controls in their parent document.
export function measureAuditLayouts(documents: Document[], selector: string) {
  const inventories: Array<Array<{ label: string; left: number; top: number; right: number; bottom: number; width: number; height: number }>> = [];
  const layouts = documents.map(document => {
    const window = document.defaultView!;
    const documentElement = document.documentElement;
    const body = document.body;
    const horizontalOverflow = Math.max(documentElement.scrollWidth, body.scrollWidth) - window.innerWidth;
    const verticalOverflow = Math.max(documentElement.scrollHeight, body.scrollHeight) - window.innerHeight;

    const elements = Array.from(document.querySelectorAll(selector))
      .filter((element) => {
        const rect = element.getBoundingClientRect();
        const style = window.getComputedStyle(element);
        if (element.closest('[data-ui-overlay="true"]')) return false;
        if (element.closest('[aria-hidden="true"]')) return false;
        if (element.closest('nextjs-portal')) return false;
        return style.visibility !== 'hidden' && style.display !== 'none' && style.opacity !== '0' && rect.width > 0 && rect.height > 0;
      })
      .map((element, index) => {
        const rect = element.getBoundingClientRect();
        return {
          index,
          label: element.getAttribute('aria-label') || (element.textContent || '').trim().replace(/\s+/g, ' ').slice(0, 80) || element.tagName.toLowerCase(),
          left: rect.left,
          top: rect.top,
          right: rect.right,
          bottom: rect.bottom,
          width: rect.width,
          height: rect.height,
        };
      });

    inventories.push(elements);
    const overlaps: string[] = [];
    for (let i = 0; i < elements.length; i += 1) {
      for (let j = i + 1; j < elements.length; j += 1) {
        const a = elements[i];
        const b = elements[j];
        const overlapX = Math.max(0, Math.min(a.right, b.right) - Math.max(a.left, b.left));
        const overlapY = Math.max(0, Math.min(a.bottom, b.bottom) - Math.max(a.top, b.top));
        const overlapArea = overlapX * overlapY;
        if (overlapArea === 0) continue;
        const smallerArea = Math.min(a.width * a.height, b.width * b.height);
        if (smallerArea > 0 && overlapArea / smallerArea > 0.35) {
          overlaps.push(`"${a.label}" overlaps "${b.label}"`);
        }
      }
    }

    return { horizontalOverflow, verticalOverflow, overlaps };
  });
  const parent = documents[0];
  for (let index = 1; index < documents.length; index += 1) {
    const frame = documents[index].defaultView!.frameElement as HTMLIFrameElement | null;
    if (!frame || frame.ownerDocument !== parent) throw new Error('Unexpected layout frame owner');
    const bounds = frame.getBoundingClientRect();
    const scaleX = bounds.width / frame.offsetWidth;
    const scaleY = bounds.height / frame.offsetHeight;
    if (!Number.isFinite(scaleX) || !Number.isFinite(scaleY) || scaleX <= 0 || scaleY <= 0) throw new Error('Invalid documentation frame layout');
    for (const embedded of inventories[index]) {
      // Child client rectangles are relative to its viewport. Clip to that
      // viewport, then account for the frame border and any CSS scale.
      const left = Math.max(0, embedded.left);
      const top = Math.max(0, embedded.top);
      const right = Math.min(frame.clientWidth, embedded.right);
      const bottom = Math.min(frame.clientHeight, embedded.bottom);
      if (right <= left || bottom <= top) continue;
      const projected = { left: bounds.left + (frame.clientLeft + left) * scaleX,
        top: bounds.top + (frame.clientTop + top) * scaleY,
        right: bounds.left + (frame.clientLeft + right) * scaleX,
        bottom: bounds.top + (frame.clientTop + bottom) * scaleY };
      for (const control of inventories[0]) {
        const area = Math.max(0, Math.min(control.right, projected.right) - Math.max(control.left, projected.left))
          * Math.max(0, Math.min(control.bottom, projected.bottom) - Math.max(control.top, projected.top));
        const smaller = Math.min(control.width * control.height, (projected.right - projected.left) * (projected.bottom - projected.top));
        if (smaller > 0 && area / smaller > 0.35) layouts[0].overlaps.push(`"${control.label}" overlaps embedded "${embedded.label}"`);
      }
    }
  }
  return layouts;
}
