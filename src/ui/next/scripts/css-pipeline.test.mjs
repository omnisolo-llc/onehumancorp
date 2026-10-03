import { readFile } from 'node:fs/promises';
import { createRequire } from 'node:module';
import { describe, expect, it } from 'vitest';
import postcss from 'postcss';
import config from '../postcss.config.mjs';

const require = createRequire(import.meta.url);

describe('application CSS build', () => {
  // This compiles the real application stylesheet and scans all production
  // content, rather than mocking Tailwind. Cold compilation takes about four
  // seconds alone and can exceed the unit-test budget under full-suite load.
  // Bound this build integration case without changing other test timeouts.
  it('generates the spacing and sizing utilities used by launchers, controls, and cards', async () => {
    const source = new URL('../src/app/globals.css', import.meta.url);
    const plugins = Object.entries(config.plugins).map(([name, options]) => require(name)(options));
    const result = await postcss(plugins).process(await readFile(source, 'utf8'), { from: source.pathname });
    const declarations = (selector) => {
      const values = {};
      result.root.walkRules(selector, (rule) => {
        rule.walkDecls((declaration) => { values[declaration.prop] = declaration.value; });
      });
      return values;
    };

    expect(declarations('.fixed')).toMatchObject({ position: 'fixed' });
    expect(declarations('.bottom-6')).toMatchObject({ bottom: '1.5rem' });
    expect(declarations('.right-6')).toMatchObject({ right: '1.5rem' });
    expect(declarations('.w-11')).toMatchObject({ width: '2.75rem' });
    expect(declarations('.h-6')).toMatchObject({ height: '1.5rem' });
    expect(declarations('.p-5')).toMatchObject({ padding: '1.25rem' });
  }, 15_000);
});
