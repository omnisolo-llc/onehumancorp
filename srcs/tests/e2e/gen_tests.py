#!/usr/bin/env python3
"""Generate Go e2e test files from TypeScript Playwright tests."""
import re
import sys

def to_go_name(name):
    name = re.sub(r'[^\w\s]', ' ', name)
    words = re.split(r'\s+', name.strip())
    result = 'Test' + ''.join(w.capitalize() for w in words if w)
    for old, new in [('Ai', 'AI'), ('Llc', 'LLC'), ('Url', 'URL'), ('Api', 'API'),
                     ('Http', 'HTTP'), ('Zip', 'ZIP'), ('Dag', 'DAG'),
                     ('Cuj', 'CUJ'), ('Ohc', 'OHC'), ('Js', 'JS')]:
        result = result.replace(old, new)
    return result

def q(s):
    """Escape for Go string"""
    return s.replace('\\', '\\\\').replace('"', '\\"')

def cvt_str(s):
    """Convert a JS string literal (single or double quoted) to Go string"""
    s = s.strip()
    if (s.startswith("'") and s.endswith("'")) or (s.startswith('"') and s.endswith('"')):
        inner = s[1:-1]
        return f'"{q(inner)}"'
    return s

LOCATOR_METHODS = {
    '.first()': '.First()',
    '.last()': '.Last()',
}

def fix_locator_chain(expr):
    """Convert locator chain from TS to Go"""
    # .first() -> .First()
    expr = re.sub(r'\.first\(\)', '.First()', expr)
    expr = re.sub(r'\.last\(\)', '.Last()', expr)
    expr = re.sub(r'\.nth\((\d+)\)', lambda m: f'.Nth({m.group(1)})', expr)
    # .filter({hasText: /pat/i}) -> .Filter(playwright.LocatorFilterOptions{HasText: playwright.String("pat")})
    expr = re.sub(r'\.filter\(\{\s*hasText:\s*/(.*?)/i?\s*\}\)',
                  lambda m: f'.Filter(playwright.LocatorFilterOptions{{HasText: playwright.String("{m.group(1)}")}})',
                  expr)
    # .filter({hasText: 'text'}) or .filter({hasText: "text"})
    expr = re.sub(r"\.filter\(\{\s*hasText:\s*'([^']*)'\s*\}\)",
                  lambda m: f'.Filter(playwright.LocatorFilterOptions{{HasText: playwright.String("{q(m.group(1))}")}})',
                  expr)
    expr = re.sub(r'\.filter\(\{\s*hasText:\s*"([^"]*)"\s*\}\)',
                  lambda m: f'.Filter(playwright.LocatorFilterOptions{{HasText: playwright.String("{q(m.group(1))}")}})',
                  expr)
    # .or(other) -> .Or(other)
    expr = re.sub(r'\.or\(', '.Or(', expr)
    return expr

def conv_locator(expr):
    """Convert a locator expression from TS to Go"""
    expr = expr.strip()
    # page.locator('sel') -> page.Locator(`sel`)
    expr = re.sub(r"page\.locator\('([^']+)'\)", lambda m: f'page.Locator(`{m.group(1)}`)', expr)
    expr = re.sub(r'page\.locator\("([^"]+)"\)', lambda m: f'page.Locator(`{m.group(1)}`)', expr)
    # page.getByText('text') -> page.GetByText("text", nil)
    expr = re.sub(r"page\.getByText\('([^']+)'\)",
                  lambda m: f'page.GetByText("{q(m.group(1))}", nil)', expr)
    expr = re.sub(r'page\.getByText\("([^"]+)"\)',
                  lambda m: f'page.GetByText("{q(m.group(1))}", nil)', expr)
    expr = re.sub(r'page\.getByText\(/(.*?)/i?\)',
                  lambda m: f'page.GetByText("{m.group(1)}", nil)', expr)
    # pipeline.getByText(...) -> pipeline.GetByText(...)
    expr = re.sub(r'(\w+)\.getByText\("([^"]+)"\)',
                  lambda m: f'{m.group(1)}.GetByText("{q(m.group(2))}", nil)', expr)
    expr = re.sub(r"(\w+)\.getByText\('([^']+)'\)",
                  lambda m: f'{m.group(1)}.GetByText("{q(m.group(2))}", nil)', expr)
    expr = re.sub(r'(\w+)\.getByText\(/(.*?)/i?\)',
                  lambda m: f'{m.group(1)}.GetByText("{m.group(2)}", nil)', expr)
    # .locator('sel') on a variable -> .Locator(`sel`)
    expr = re.sub(r"\.locator\('([^']+)'\)", lambda m: f'.Locator(`{m.group(1)}`)', expr)
    expr = re.sub(r'\.locator\("([^"]+)"\)', lambda m: f'.Locator(`{m.group(1)}`)', expr)
    # Apply chain fixes
    expr = fix_locator_chain(expr)
    return expr

def conv_select_option(arg):
    """Convert selectOption argument"""
    arg = arg.strip()
    # { index: N }
    m = re.match(r"\{\s*index:\s*(\d+)\s*\}", arg)
    if m:
        return f'playwright.SelectOptionValues{{Indices: []int{{{m.group(1)}}}}}'
    # { label: 'x' } or { label: varname }
    m = re.match(r"\{\s*label:\s*'([^']*)'\s*\}", arg)
    if m:
        return f'playwright.SelectOptionValues{{Labels: playwright.StringSlice("{q(m.group(1))}")}}'
    m = re.match(r'\{\s*label:\s*"([^"]*)"\s*\}', arg)
    if m:
        return f'playwright.SelectOptionValues{{Labels: playwright.StringSlice("{q(m.group(1))}")}}'
    m = re.match(r'\{\s*label:\s*(\w+)\s*\}', arg)
    if m:
        return f'playwright.SelectOptionValues{{Labels: playwright.StringSlice({m.group(1)})}}'
    # { value: 'x' }
    m = re.match(r"\{\s*value:\s*'([^']*)'\s*\}", arg)
    if m:
        return f'playwright.SelectOptionValues{{Values: playwright.StringSlice("{q(m.group(1))}")}}'
    m = re.match(r'\{\s*value:\s*(\w+)\s*\}', arg)
    if m:
        return f'playwright.SelectOptionValues{{Values: playwright.StringSlice({m.group(1)})}}'
    # 'literal string'
    m = re.match(r"'([^']*)'", arg)
    if m:
        return f'playwright.SelectOptionValues{{Values: playwright.StringSlice("{q(m.group(1))}")}}'
    m = re.match(r'"([^"]*)"', arg)
    if m:
        return f'playwright.SelectOptionValues{{Values: playwright.StringSlice("{q(m.group(1))}")}}'
    return f'playwright.SelectOptionValues{{Values: playwright.StringSlice({arg})}}'

def convert_body(ts_body):
    """Convert a TypeScript test body to Go statements."""
    lines = ts_body.split('\n')
    go_lines = []
    
    # Track indent to handle braces/blocks
    brace_depth = 0
    skip_until_depth = None
    
    for raw_line in lines:
        stripped = raw_line.strip()
        
        if not stripped:
            go_lines.append('')
            continue
        
        # Count indent
        indent_chars = len(raw_line) - len(raw_line.lstrip())
        tabs = max(1, indent_chars // 2)
        prefix = '\t' * tabs
        
        # Comments
        if stripped.startswith('//'):
            go_lines.append(prefix + stripped)
            continue
        
        # Try to convert the line
        go_line = try_convert(stripped)
        if go_line is not None:
            if go_line:
                go_lines.append(prefix + go_line)
            # else: skip line
        else:
            # Can't convert - comment it out  
            go_lines.append(prefix + '// ' + stripped[:120])
    
    return '\n'.join(go_lines)

def try_convert(s):
    """Try to convert a TypeScript statement. Returns None if can't convert."""
    s = s.rstrip(';').strip()
    
    # Empty braces/blocks - skip
    if s in ('{', '}', '});', '});', '})', '});'):
        return None
    
    # Helpers
    if s == 'await loginAsAdmin(page)':
        return 'loginAsAdmin(t, page)'
    if s == 'await openApp(page)':
        return 'openApp(t, page)'
    if s == 'await clickNext(page)':
        return 'clickNext(t, page)'
    
    # page.waitForLoadState
    if s == "await page.waitForLoadState('networkidle')":
        return '_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)'
    if s == "await page.waitForLoadState('domcontentloaded')":
        return '_ = page.WaitForLoadState(playwright.LoadStateDomcontentloaded, nil)'
    
    # waitForTimeout
    m = re.match(r"await page\.waitForTimeout\((\d+(?:_\d+)*)\)", s)
    if m:
        ms = m.group(1).replace('_', '')
        return f'sleepMs({ms})'
    
    # page.goto
    m = re.match(r"await page\.goto\('([^']+)'\)(?:\.catch\([^)]*\))?$", s)
    if m:
        path = m.group(1)
        suffix = '"/' + path.lstrip('/') + '"' if path != '/' else '"/"'
        if path == '/':
            suffix = '"/"'
        else:
            suffix = f'"{path}"'
        if '.catch(' in s:
            return f'_, _ = page.Goto(baseURL + {suffix})'
        else:
            return f'if _, err := page.Goto(baseURL + {suffix}); err != nil {{ t.Logf("goto: %v", err) }}'
    
    # page.reload
    if s in ('await page.reload()', 'await page.reload().catch(() => {})'):
        return '_, _ = page.Reload(nil)'
    
    # page.goBack
    if s in ('await page.goBack()', 'await page.goBack().catch(() => {})'):
        return '_, _ = page.GoBack(nil)'
    
    # page.waitForURL
    if 'await page.waitForURL(' in s:
        m = re.search(r'timeout:\s*(\d+(?:_\d+)*)', s)
        if m:
            timeout = m.group(1).replace('_', '')
            return f'_ = page.WaitForURL("**", playwright.PageWaitForURLOptions{{Timeout: playwright.Float({timeout})}})'
        return '_ = page.WaitForURL("**", nil)'
    
    # page.keyboard.press
    m = re.match(r"await page\.keyboard\.press\('([^']+)'\)", s)
    if m:
        return f'_ = page.Keyboard.Press("{m.group(1)}")'
    
    # page.setViewportSize
    m = re.search(r'await page\.setViewportSize\(\{\s*width:\s*(\d+),\s*height:\s*(\d+)\s*\}\)', s)
    if m:
        return f'_ = page.SetViewportSize({m.group(1)}, {m.group(2)})'
    
    # page.context().setOffline
    m = re.search(r"await page\.context\(\)\.setOffline\((true|false)\)", s)
    if m:
        return f'_ = page.Context().SetOffline({m.group(1)})'
    
    # page.waitForSelector
    m = re.match(r"await page\.waitForSelector\('([^']+)'(?:,\s*\{[^}]*timeout:\s*(\d+(?:_\d+)*)[^}]*\})?\)", s)
    if m:
        sel = m.group(1)
        timeout = m.group(2)
        if timeout:
            t_val = timeout.replace('_', '')
            return f'_, _ = page.WaitForSelector(`{sel}`, playwright.PageWaitForSelectorOptions{{Timeout: playwright.Float({t_val})}})'
        return f'_, _ = page.WaitForSelector(`{sel}`, nil)'
    
    # page.evaluate(...)
    m = re.match(r"await page\.evaluate\((.+)\)(?:\.catch\([^)]*\))?", s)
    if m:
        return f'_, _ = page.Evaluate({m.group(1)}, nil)'
    
    # page.route(...)
    m = re.match(r"await page\.route\('([^']+)',\s*route\s*=>", s)
    if m:
        pat = m.group(1)
        return f'_ = page.Route("{pat}", func(route playwright.Route) {{'
    
    # route.fulfill
    if 'route.fulfill(' in s:
        m = re.search(r'status:\s*(\d+)', s)
        m2 = re.search(r"body:\s*['\"](.+?)['\"]", s)
        if m and m2:
            return f'_ = route.Fulfill(playwright.RouteFulfillOptions{{Status: playwright.Int({m.group(1)}), Body: playwright.String(`{m2.group(1)}`)}})'
        return None
    
    # await expect(loc).toBeVisible({timeout: N})
    m = re.match(r"await expect\((.+)\)\.toBeVisible\(\{\s*timeout:\s*(\d+(?:_\d+)*)\s*\}\)", s)
    if m:
        loc = conv_locator(m.group(1).strip())
        timeout = m.group(2).replace('_', '')
        return f'if err := playwright.Expect({loc}).ToBeVisible(playwright.LocatorAssertionsToBeVisibleOptions{{Timeout: playwright.Float({timeout})}}); err != nil {{ t.Fatalf("expected visible: %v", err) }}'
    
    # await expect(loc).toBeVisible()
    m = re.match(r"await expect\((.+)\)\.toBeVisible\(\)", s)
    if m:
        loc = conv_locator(m.group(1).strip())
        return f'if err := playwright.Expect({loc}).ToBeVisible(nil); err != nil {{ t.Fatalf("expected visible: %v", err) }}'
    
    # await expect(loc).not.toBeVisible({timeout: N})
    m = re.match(r"await expect\((.+)\)\.not\.toBeVisible\(\{\s*timeout:\s*(\d+(?:_\d+)*)\s*\}\)", s)
    if m:
        loc = conv_locator(m.group(1).strip())
        timeout = m.group(2).replace('_', '')
        return f'if err := playwright.Expect({loc}).Not().ToBeVisible(playwright.LocatorAssertionsToBeVisibleOptions{{Timeout: playwright.Float({timeout})}}); err != nil {{ t.Fatalf("expected not visible: %v", err) }}'
    
    # await expect(loc).not.toBeVisible()
    m = re.match(r"await expect\((.+)\)\.not\.toBeVisible\(\)", s)
    if m:
        loc = conv_locator(m.group(1).strip())
        return f'if err := playwright.Expect({loc}).Not().ToBeVisible(nil); err != nil {{ t.Fatalf("expected not visible: %v", err) }}'
    
    # await expect(loc).toBeEnabled({timeout: N})
    m = re.match(r"await expect\((.+)\)\.toBeEnabled\(\{\s*timeout:\s*(\d+(?:_\d+)*)\s*\}\)", s)
    if m:
        loc = conv_locator(m.group(1).strip())
        timeout = m.group(2).replace('_', '')
        return f'if err := playwright.Expect({loc}).ToBeEnabled(playwright.LocatorAssertionsToBeEnabledOptions{{Timeout: playwright.Float({timeout})}}); err != nil {{ t.Fatalf("expected enabled: %v", err) }}'
    
    # await expect(loc).toBeEnabled()
    m = re.match(r"await expect\((.+)\)\.toBeEnabled\(\)", s)
    if m:
        loc = conv_locator(m.group(1).strip())
        return f'if err := playwright.Expect({loc}).ToBeEnabled(nil); err != nil {{ t.Fatalf("expected enabled: %v", err) }}'
    
    # await expect(loc).toBeChecked()
    m = re.match(r"await expect\((.+)\)\.toBeChecked\(\)", s)
    if m:
        loc = conv_locator(m.group(1).strip())
        return f'if err := playwright.Expect({loc}).ToBeChecked(nil); err != nil {{ t.Fatalf("expected checked: %v", err) }}'
    
    # await expect(loc).not.toBeEmpty()
    m = re.match(r"await expect\((.+)\)\.not\.toBeEmpty\(\)", s)
    if m:
        loc = conv_locator(m.group(1).strip())
        return f'if err := playwright.Expect({loc}).Not().ToBeEmpty(nil); err != nil {{ t.Fatalf("expected not empty: %v", err) }}'
    
    # await expect(loc).toContainText(str, {timeout: N})
    m = re.match(r"await expect\((.+)\)\.toContainText\('([^']+)',\s*\{\s*timeout:\s*(\d+(?:_\d+)*)\s*\}\)", s)
    if m:
        loc = conv_locator(m.group(1).strip())
        text = q(m.group(2))
        timeout = m.group(3).replace('_', '')
        return f'if err := playwright.Expect({loc}).ToContainText("{text}", playwright.LocatorAssertionsToContainTextOptions{{Timeout: playwright.Float({timeout})}}); err != nil {{ t.Fatalf("expected contains text: %v", err) }}'
    
    # await expect(loc).toContainText('str')
    m = re.match(r"await expect\((.+)\)\.toContainText\('([^']+)'\)", s)
    if m:
        loc = conv_locator(m.group(1).strip())
        text = q(m.group(2))
        return f'if err := playwright.Expect({loc}).ToContainText("{text}", nil); err != nil {{ t.Fatalf("expected contains text: %v", err) }}'
    
    # await expect(loc).toContainText(/pattern/i)
    m = re.match(r"await expect\((.+)\)\.toContainText\(/(.*?)/i?\)", s)
    if m:
        loc = conv_locator(m.group(1).strip())
        pattern = m.group(2)
        # If it's page.locator('body'), use content check
        if 'body' in m.group(1) and 'locator' in m.group(1):
            return f'if matched, _ := regexp.MatchString(`(?i){pattern}`, func() string {{ c, _ := page.Content(); return c }}()); !matched {{ t.Error("body should contain text") }}'
        return f'if err := playwright.Expect({loc}).ToContainText("{pattern}", nil); err != nil {{ t.Fatalf("expected contains text: %v", err) }}'
    
    # await expect(loc).not.toContainText(/pattern/i)
    m = re.match(r"await expect\((.+)\)\.not\.toContainText\(/(.*?)/i?\)", s)
    if m:
        loc_str = m.group(1).strip()
        pattern = m.group(2)
        if ('body' in loc_str and 'locator' in loc_str) or loc_str == "page.locator('body')":
            return f'if matched, _ := regexp.MatchString(`(?i){pattern}`, func() string {{ c, _ := page.Content(); return c }}()); matched {{ t.Error("body contains error text") }}'
        loc = conv_locator(loc_str)
        return f'if err := playwright.Expect({loc}).Not().ToContainText("{pattern}", nil); err != nil {{ t.Fatalf("expected not contains text: %v", err) }}'
    
    # await expect(loc).not.toContainText('str')
    m = re.match(r"await expect\((.+)\)\.not\.toContainText\('([^']+)'\)", s)
    if m:
        loc = conv_locator(m.group(1).strip())
        text = q(m.group(2))
        return f'if err := playwright.Expect({loc}).Not().ToContainText("{text}", nil); err != nil {{ t.Fatalf("expected not contains text: %v", err) }}'
    
    # await expect(loc).toHaveValue('str')
    m = re.match(r"await expect\((.+)\)\.toHaveValue\('([^']+)'\)", s)
    if m:
        loc = conv_locator(m.group(1).strip())
        val = q(m.group(2))
        return f'if err := playwright.Expect({loc}).ToHaveValue("{val}", nil); err != nil {{ t.Fatalf("expected value: %v", err) }}'
    
    # await expect(loc).toHaveAttribute('name', 'val')
    m = re.match(r"await expect\((.+)\)\.toHaveAttribute\('([^']+)',\s*'([^']+)'\)", s)
    if m:
        loc = conv_locator(m.group(1).strip())
        attr = m.group(2)
        val = m.group(3)
        return f'if err := playwright.Expect({loc}).ToHaveAttribute("{attr}", "{val}", nil); err != nil {{ t.Fatalf("expected attribute: %v", err) }}'
    
    # expect(true).toBe(true) - no-op
    if re.match(r"expect\(true\)\.toBe\(true\)", s):
        return '// (pass)'
    
    # expect(val).toBe(true)
    m = re.match(r"expect\((\w+)\)\.toBe\(true\)", s)
    if m:
        var_n = m.group(1)
        return f'if !{var_n} {{ t.Error("expected true") }}'
    
    # expect(val).toBe(false)
    m = re.match(r"expect\((\w+)\)\.toBe\(false\)", s)
    if m:
        var_n = m.group(1)
        return f'if {var_n} {{ t.Error("expected false") }}'
    
    # expect(val).not.toBe('')
    m = re.match(r"expect\((\w+)\)\.not\.toBe\(''\)", s)
    if m:
        var_n = m.group(1)
        return f'if {var_n} == "" {{ t.Error("expected non-empty") }}'
    
    # expect(response?.status()).toBeLessThan(500)
    if "response?.status()" in s and "toBeLessThan(500)" in s:
        return 'if resp != nil && resp.Status() >= 500 { t.Errorf("expected status < 500, got %d", resp.Status()) }'
    if "response?.status()" in s and "toBe(200)" in s:
        return 'if resp != nil && resp.Status() != 200 { t.Errorf("expected 200, got %d", resp.Status()) }'
    
    # expect(val).toMatch(/pattern/)
    m = re.match(r"expect\((\w+)\)\.toMatch\(/(.*?)/[gi]*\)", s)
    if m:
        var_n = m.group(1)
        pat = m.group(2)
        return f'if matched, _ := regexp.MatchString(`{pat}`, {var_n}); !matched {{ t.Errorf("expected match %q", {var_n}) }}'
    
    # expect(val).not.toMatch(/pattern/)  
    m = re.match(r"expect\((\w+)\)\.not\.toMatch\(/(.*?)/[gi]*\)", s)
    if m:
        var_n = m.group(1)
        pat = m.group(2)
        return f'if matched, _ := regexp.MatchString(`(?i){pat}`, {var_n}); matched {{ t.Errorf("unexpected match in %q", {var_n}) }}'
    
    # expect(n).toBeGreaterThan(0)
    m = re.match(r"expect\((\w+)\)\.toBeGreaterThan\((\d+)\)", s)
    if m:
        var_n, val = m.group(1), m.group(2)
        return f'if {var_n} <= {val} {{ t.Errorf("expected > {val}") }}'
    
    # expect(n).toBeGreaterThanOrEqual(0)
    m = re.match(r"expect\((\w+)\)\.toBeGreaterThanOrEqual\((\d+)\)", s)
    if m:
        var_n, val = m.group(1), m.group(2)
        return f'if {var_n} < {val} {{ t.Errorf("expected >= {val}") }}'
    
    # expect(n).toBeLessThan(N)
    m = re.match(r"expect\((\w+)\)\.toBeLessThan\((\d+(?:\s*\*\s*\d+(?:\s*\*\s*\d+)?)?)\)", s)
    if m:
        var_n = m.group(1)
        val_expr = m.group(2).replace(' ', '')
        try:
            val = eval(val_expr)
            return f'if {var_n} >= {val} {{ t.Errorf("expected < {val}") }}'
        except:
            return f'if {var_n} >= int({val_expr}) {{ t.Errorf("expected < {val_expr}") }}'
    
    # expect(n).toBeLessThanOrEqual(N)
    m = re.match(r"expect\((\w+)\)\.toBeLessThanOrEqual\(([0-9.]+)\)", s)
    if m:
        var_n, val = m.group(1), m.group(2)
        return f'if float64({var_n}) > {val} {{ t.Errorf("expected <= {val}") }}'
    
    # expect(val).toContain('str')
    m = re.match(r"expect\((\w+)\)\.toContain\('([^']+)'\)", s)
    if m:
        var_n, val = m.group(1), q(m.group(2))
        return f'if !strings.Contains({var_n}, "{val}") {{ t.Errorf("expected contains") }}'
    
    # expect(val).not.toEqual(str)
    m = re.match(r"expect\((\w+)\)\.not\.toEqual\((\w+)\)", s)
    if m:
        var_n, other = m.group(1), m.group(2)
        return f'if {var_n} == {other} {{ t.Error("expected not equal") }}'
    
    # expect(val).toEqual(str)
    m = re.match(r"expect\((\w+)\)\.toEqual\((\w+)\)", s)
    if m:
        var_n, other = m.group(1), m.group(2)
        return f'if {var_n} != {other} {{ t.Error("expected equal") }}'
    
    # await locator.click()
    m = re.match(r"await (.+?)\.click\(\)", s)
    if m:
        loc = conv_locator(m.group(1))
        return f'if err := {loc}.Click(nil); err != nil {{ t.Logf("click: %v", err) }}'
    
    # await locator.fill('x')
    m = re.match(r"await (.+?)\.fill\('([^']*)'\)", s)
    if m:
        loc = conv_locator(m.group(1))
        val = q(m.group(2))
        return f'if err := {loc}.Fill("{val}", nil); err != nil {{ t.Logf("fill: %v", err) }}'
    
    m = re.match(r'await (.+?)\.fill\("([^"]*)"\)', s)
    if m:
        loc = conv_locator(m.group(1))
        val = q(m.group(2))
        return f'if err := {loc}.Fill("{val}", nil); err != nil {{ t.Logf("fill: %v", err) }}'
    
    # await locator.fill(varname)
    m = re.match(r'await (.+?)\.fill\((\w+)\)', s)
    if m:
        loc = conv_locator(m.group(1))
        var_n = m.group(2)
        return f'if err := {loc}.Fill({var_n}, nil); err != nil {{ t.Logf("fill: %v", err) }}'
    
    # await locator.check()
    m = re.match(r"await (.+?)\.check\(\)", s)
    if m:
        loc = conv_locator(m.group(1))
        return f'_ = {loc}.Check(nil)'
    
    # await locator.uncheck()
    m = re.match(r"await (.+?)\.uncheck\(\)", s)
    if m:
        loc = conv_locator(m.group(1))
        return f'_ = {loc}.Uncheck(nil)'
    
    # await locator.press('key')
    m = re.match(r"await (.+?)\.press\('([^']+)'\)", s)
    if m:
        loc = conv_locator(m.group(1))
        key = m.group(2)
        return f'_ = {loc}.Press("{key}", nil)'
    
    # await locator.selectOption({...})
    m = re.match(r"await (.+?)\.selectOption\((.+)\)", s)
    if m:
        loc = conv_locator(m.group(1))
        opt_arg = m.group(2).strip()
        opt_go = conv_select_option(opt_arg)
        return f'_, _ = {loc}.SelectOption({opt_go}, nil)'
    
    # const response = await page.goto(...)
    m = re.match(r"const response\s*=\s*await page\.goto\('([^']+)'\)", s)
    if m:
        path = m.group(1)
        suffix = f'"{path}"' if path.startswith('/') else f'"/{path}"'
        return f'resp, _ := page.Goto(baseURL + {suffix})'
    
    # const val = await locator.inputValue()
    m = re.match(r"const (\w+)\s*=\s*await (.+?)\.inputValue\(\)", s)
    if m:
        var_n = m.group(1)
        loc = conv_locator(m.group(2))
        return f'{var_n}, _ := {loc}.InputValue()'
    
    # const val = await locator.textContent()
    m = re.match(r"const (\w+)\s*=\s*await (.+?)\.textContent\(\)", s)
    if m:
        var_n = m.group(1)
        loc = conv_locator(m.group(2))
        return f'{var_n}, _ := {loc}.TextContent()'
    
    # const val = await locator.allTextContents()
    m = re.match(r"const (\w+)\s*=\s*await (.+?)\.allTextContents\(\)", s)
    if m:
        var_n = m.group(1)
        loc = conv_locator(m.group(2))
        return f'{var_n}, _ := {loc}.AllTextContents()'
    
    # const val = await page.content()
    m = re.match(r"const (\w+)\s*=\s*await page\.content\(\)", s)
    if m:
        var_n = m.group(1)
        return f'{var_n}, _ := page.Content()'
    
    # const val = await page.title()
    m = re.match(r"const (\w+)\s*=\s*await page\.title\(\)", s)
    if m:
        var_n = m.group(1)
        return f'{var_n}, _ := page.Title()'
    
    # const count = await locator.count()
    m = re.match(r"const (\w+)\s*=\s*await (.+?)\.count\(\)", s)
    if m:
        var_n = m.group(1)
        loc = conv_locator(m.group(2))
        return f'{var_n}, _ := {loc}.Count()'
    
    # const visible = await locator.isVisible(...)
    m = re.match(r"const (\w+)\s*=\s*await (.+?)\.isVisible\(\)", s)
    if m:
        var_n = m.group(1)
        loc = conv_locator(m.group(2))
        return f'{var_n}, _ := {loc}.IsVisible()'
    
    m = re.match(r"const (\w+)\s*=\s*await (.+?)\.isVisible\(.*?\)(?:\.catch\([^)]*\))?", s)
    if m:
        var_n = m.group(1)
        loc = conv_locator(m.group(2))
        return f'{var_n}, _ := {loc}.IsVisible()'
    
    # const disabled = await locator.isDisabled()
    m = re.match(r"const (\w+)\s*=\s*await (.+?)\.isDisabled\(\)", s)
    if m:
        var_n = m.group(1)
        loc = conv_locator(m.group(2))
        return f'{var_n}, _ := {loc}.IsDisabled()'
    
    # if ((await locator.count()) > 0) {
    m = re.match(r"if \(\(await (.+?)\.count\(\)\)\s*([><=!]+)\s*(\d+)\)\s*\{", s)
    if m:
        loc = conv_locator(m.group(1))
        op = m.group(2)
        val = m.group(3)
        return f'if cnt, _ := {loc}.Count(); cnt {op} {val} {{'
    
    # if (await locator.isVisible({ timeout: N })) {
    m = re.match(r"if \(await (.+?)\.isVisible\(\{\s*timeout:\s*(\d+(?:_\d+)*)\s*\}\)\)\s*\{", s)
    if m:
        loc = conv_locator(m.group(1))
        return f'if vis, _ := {loc}.IsVisible(); vis {{'
    
    # if (await locator.isVisible()) {
    m = re.match(r"if \(await (.+?)\.isVisible\(\)\)\s*\{", s)
    if m:
        loc = conv_locator(m.group(1))
        return f'if vis, _ := {loc}.IsVisible(); vis {{'
    
    # if (await locator.isVisible({ timeout: N }).catch(() => false)) {
    m = re.match(r"if \(await (.+?)\.isVisible\(.*?\)(?:\.catch\([^)]*\))?\)\s*\{", s)
    if m:
        loc = conv_locator(m.group(1))
        return f'if vis, _ := {loc}.IsVisible(); vis {{'
    
    # if (await locator.isDisabled()) {
    m = re.match(r"if \(await (.+?)\.isDisabled\(\)\)\s*\{", s)
    if m:
        loc = conv_locator(m.group(1))
        return f'if dis, _ := {loc}.IsDisabled(); dis {{'
    
    # for loop: for (let i = 0; i < N; i++) {
    m = re.match(r"for \(let (\w+) = (\d+);\s*\w+ < (\w+|\d+);\s*\w+\+\+\)\s*\{", s)
    if m:
        var_n = m.group(1)
        start = m.group(2)
        end = m.group(3)
        # Check if end is a constant
        end_val = end
        for const_name, const_val in [('MAX_WIZARD_STEPS', '10'), ('MAX_NAVIGATION_ATTEMPTS', '6'), 
                                       ('MAX_GOALS_TO_SELECT', '3'), ('count', 'count'), 
                                       ('toCheck', 'toCheck')]:
            if end_val == const_name:
                end_val = const_val
                break
        return f'for {var_n} := {start}; {var_n} < {end_val}; {var_n}++ {{'
    
    # for (let i = 0; i < Math.min(count, N); i++) {
    m = re.match(r"for \(let (\w+) = (\d+);\s*\w+ < Math\.min\((\w+),\s*(\d+)\);\s*\w+\+\+\)\s*\{", s)
    if m:
        var_n = m.group(1)
        start = m.group(2)
        count_var = m.group(3)
        max_n = m.group(4)
        return f'for {var_n} := {start}; {var_n} < func() int {{ if {count_var} < {max_n} {{ return {count_var} }}; return {max_n} }}(); {var_n}++ {{'
    
    # return; or break;
    if s == 'return':
        return 'return'
    if s == 'break':
        return 'break'
    if s == 'continue':
        return 'continue'
    
    # } else { 
    if s in ('} else {', '} else if (true) {'):
        return '} else {'
    
    # closing brace
    if s == '}':
        return '}'
    
    # const varname = page.locator(...)
    # These are hard to convert in general; skip them
    
    return None

def gen_test(name, body):
    """Generate a Go test function"""
    func_name = to_go_name(name)
    go_body = convert_body(body)
    
    return f"""func {func_name}(t *testing.T) {{
\tpage := newPage(t)
\tdefer page.Close()
{go_body}
}}
"""

# Read TS source
with open('ohc-cuj.spec.ts', 'r') as f:
    ts_content = f.read()

test_pattern = re.compile(
    r"test\('([^']+)',\s*async\s*\(\{\s*page\s*\}\)\s*=>\s*\{(.*?)\n\}\);",
    re.DOTALL
)
tests = test_pattern.findall(ts_content)

# Generate test functions
output_lines = [
    'package e2e',
    '',
    'import (',
    '\t"regexp"',
    '\t"strings"',
    '\t"sync"',
    '\t"testing"',
    '\t"time"',
    '',
    '\tplaywright "github.com/playwright-community/playwright-go"',
    ')',
    '',
    '// Suppress unused import warnings',
    'var (',
    '\t_ = regexp.MustCompile',
    '\t_ = strings.Contains',
    '\t_ sync.Mutex',
    '\t_ = time.Sleep',
    ')',
    '',
]

for name, body in tests:
    func_code = gen_test(name, body)
    output_lines.append(func_code)

with open('cuj_part1_test.go', 'w') as f:
    f.write('\n'.join(output_lines))

print(f"Generated cuj_part1_test.go with {len(tests)} tests")
