import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

const css = readFileSync(join(__dirname, 'assistant.module.css'), 'utf8');

function blockFor(selector: string) {
  const escaped = selector.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const match = css.match(new RegExp(`${escaped}\\s*\\{([^}]+)\\}`));
  return match?.[1] || '';
}

describe('assistant side menu layout css', () => {
  it('keeps the section menu in normal page flow without its own scrollbar', () => {
    const sectionMenuList = blockFor('.sectionMenuList');

    expect(sectionMenuList).toContain('max-height: none');
    expect(sectionMenuList).toContain('overflow: visible');
    expect(sectionMenuList).not.toContain('overflow-y: auto');
  });
});
