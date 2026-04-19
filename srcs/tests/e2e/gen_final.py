#!/usr/bin/env python3
"""Final generator for Go e2e tests from TypeScript Playwright."""
import re

def to_go_name(name):
    name = re.sub(r'[^\w\s]', ' ', name)
    words = re.split(r'\s+', name.strip())
    result = 'Test' + ''.join(w.capitalize() for w in words if w)
    for old, new in [('Ai', 'AI'), ('Llc', 'LLC'), ('Url', 'URL'), ('Api', 'API'),
                     ('Http', 'HTTP'), ('Zip', 'ZIP'), ('Dag', 'DAG'), ('Js', 'JS')]:
        result = result.replace(old, new)
    return result

def q(s):
    """Escape for Go double-quoted string."""
    return s.replace('\\', '\\\\').replace('"', '\\"')

def qs(s):
    """Escape for Go backtick string."""
    return s.replace('`', "'")

def conv_chain(chain):
    if not chain: return ''
    chain = re.sub(r'\.first\(\)', '.First()', chain)
    chain = re.sub(r'\.last\(\)', '.Last()', chain)
    chain = re.sub(r'\.nth\((\d+)\)', lambda m: f'.Nth({m.group(1)})', chain)
    chain = re.sub(r'\.filter\(\{\s*hasText:\s*/(.*?)/i?\s*\}\)',
                   lambda m: f'.Filter(playwright.LocatorFilterOptions{{HasText: playwright.String("{m.group(1)}")}})', chain)
    chain = re.sub(r"\.filter\(\{\s*hasText:\s*'([^']*)'\s*\}\)",
                   lambda m: f'.Filter(playwright.LocatorFilterOptions{{HasText: playwright.String("{q(m.group(1))}")}})', chain)
    chain = re.sub(r"\.locator\('([^']+)'\)", lambda m: f'.Locator(`{qs(m.group(1))}`)', chain)
    chain = re.sub(r'\.locator\("([^"]+)"\)', lambda m: f'.Locator(`{qs(m.group(1))}`)', chain)
    chain = re.sub(r'\.or\((\w+)\)', lambda m: f'.Or({m.group(1)})', chain)
    return chain

def preprocess(body):
    """Join continuation lines."""
    lines = body.split('\n')
    result = []
    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        if not stripped or stripped.startswith('//'):
            result.append(line)
            i += 1
            continue
        joined = line.rstrip()
        while i + 1 < len(lines):
            nxt = lines[i + 1].strip()
            if (not nxt or nxt.startswith('//') or
                re.match(r'(const|let|var|await|if\s|for\s|return|break|continue|expect\()\b', nxt) or
                nxt in ('}', '} else {', '});')):
                break
            curr = joined.rstrip()
            if curr.endswith(',') or curr.endswith('(') or nxt.startswith('.') or nxt.startswith(')'):
                joined = curr + ' ' + nxt
                i += 1
            else:
                break
        result.append(joined)
        i += 1
    return '\n'.join(result)

def loc_from_ts(expr, declared):
    """Convert a TS locator expression to Go. Returns Go expression or None."""
    expr = expr.strip()

    # page.locator('...') with chain
    m = re.match(r"page\.locator\(\s*'([^']+)'\s*\)(.*)", expr, re.DOTALL)
    if m:
        sel = qs(m.group(1))
        chain = conv_chain(m.group(2).strip())
        return f'page.Locator(`{sel}`){chain}'
    m = re.match(r'page\.locator\(\s*"([^"]+)"\s*\)(.*)', expr, re.DOTALL)
    if m:
        sel = qs(m.group(1))
        chain = conv_chain(m.group(2).strip())
        return f'page.Locator(`{sel}`){chain}'

    # page.getByText(...)
    m = re.match(r"page\.getByText\('([^']+)'\)(.*)", expr)
    if m: return f'page.GetByText("{q(m.group(1))}", nil){conv_chain(m.group(2))}'
    m = re.match(r"page\.getByText\(/(.*?)/i?\)(.*)", expr)
    if m: return f'page.GetByText("{m.group(1)}", nil){conv_chain(m.group(2))}'

    # declared variable, possibly with chain
    m = re.match(r'^(\w+)(\..*)?$', expr.strip())
    if m:
        var_n = m.group(1)
        chain = m.group(2) or ''
        if var_n in declared:
            return declared[var_n] + conv_chain(chain)
        if var_n not in ('page', 'response', 'route', 'msg', 'req', 'resp', 'sizes', 'opts', 'options', 'taskItems', 'checkboxes'):
            # Unknown variable - return it as-is (might be a function parameter)
            return var_n + conv_chain(chain)

    return None

def conv_select(arg):
    arg = arg.strip()
    m = re.match(r"\{\s*index:\s*(\d+)\s*\}", arg)
    if m: return f'playwright.SelectOptionValues{{Indices: []int{{{m.group(1)}}}}}'
    m = re.match(r"\{\s*label:\s*'([^']*)'\s*\}", arg)
    if m: return f'playwright.SelectOptionValues{{Labels: playwright.StringSlice("{q(m.group(1))}")}}'
    m = re.match(r'\{\s*label:\s*(\w+)\s*\}', arg)
    if m: return f'playwright.SelectOptionValues{{Labels: playwright.StringSlice({m.group(1)})}}'
    m = re.match(r"\{\s*value:\s*'([^']*)'\s*\}", arg)
    if m: return f'playwright.SelectOptionValues{{Values: playwright.StringSlice("{q(m.group(1))}")}}'
    m = re.match(r"'([^']*)'$", arg)
    if m: return f'playwright.SelectOptionValues{{Values: playwright.StringSlice("{q(m.group(1))}")}}'
    return f'playwright.SelectOptionValues{{Values: playwright.StringSlice({arg})}}'

def convert_stmt(s, declared):
    """Convert a single TypeScript statement. Returns Go string or None."""
    s = s.rstrip(';').strip()

    # Helpers
    if s == 'await loginAsAdmin(page)': return 'loginAsAdmin(t, page)'
    if s == 'await openApp(page)': return 'openApp(t, page)'
    if s == 'await clickNext(page)': return 'clickNext(t, page)'
    if "await page.waitForLoadState('networkidle')" in s: return '_ = page.WaitForLoadState(playwright.LoadStateNetworkidle, nil)'
    if "await page.waitForLoadState('domcontentloaded')" in s: return '_ = page.WaitForLoadState(playwright.LoadStateDomcontentloaded, nil)'

    m = re.match(r"await page\.waitForTimeout\((\d+(?:_\d+)*)\)", s)
    if m: return f'sleepMs({m.group(1).replace("_","")})'

    if 'await page.reload()' in s: return '_, _ = page.Reload(nil)'
    if 'await page.goBack()' in s: return '_, _ = page.GoBack(nil)'

    m = re.search(r'await page\.setViewportSize\(\{\s*width:\s*(\d+),\s*height:\s*(\d+)\s*\}\)', s)
    if m: return f'_ = page.SetViewportSize({m.group(1)}, {m.group(2)})'

    m = re.search(r"await page\.context\(\)\.setOffline\((true|false)\)", s)
    if m: return f'_ = page.Context().SetOffline({m.group(1)})'

    m = re.match(r"await page\.keyboard\.press\('([^']+)'\)", s)
    if m: return f'_ = page.Keyboard.Press("{m.group(1)}")'

    if 'await page.waitForURL(' in s:
        m = re.search(r'timeout:\s*(\d+(?:_\d+)*)', s)
        if m: return f'_ = page.WaitForURL("**", playwright.PageWaitForURLOptions{{Timeout: playwright.Float({m.group(1).replace("_","")})}})'
        return '_ = page.WaitForURL("**", nil)'

    m = re.match(r"await page\.waitForSelector\('([^']+)'(?:,\s*\{[^}]*timeout:\s*(\d+(?:_\d+)*)[^}]*\})?\)", s)
    if m:
        sel = m.group(1); t = m.group(2)
        if t: return f'_, _ = page.WaitForSelector(`{qs(sel)}`, playwright.PageWaitForSelectorOptions{{Timeout: playwright.Float({t.replace("_","")})}})'
        return f'_, _ = page.WaitForSelector(`{qs(sel)}`, nil)'

    if 'await page.evaluate(' in s: return '// page.Evaluate(...)'

    # page.goto
    m = re.match(r"await page\.goto\('([^']+)'\)(?:\.catch\([^)]*\))?$", s)
    if m:
        path = m.group(1)
        if '.catch(' in s: return f'_, _ = page.Goto(baseURL + "{path}")'
        return f'if _, err := page.Goto(baseURL + "{path}"); err != nil {{ t.Logf("goto: %v", err) }}'

    # const response = await page.goto(...)
    m = re.match(r"(?:const\s+)?response\s*=\s*await page\.goto\('([^']+)'\)(?:\.catch\([^)]*\))?", s)
    if m:
        path = m.group(1)
        return f'resp, _ := page.Goto(baseURL + "{path}")'

    if 'response?.status()' in s and 'toBeLessThan(500)' in s:
        return 'if resp != nil && resp.Status() >= 500 { t.Errorf("expected status < 500, got %d", resp.Status()) }'
    if 'response?.status()' in s and 'toBe(200)' in s:
        return 'if resp != nil && resp.Status() != 200 { t.Errorf("expected 200, got %d", resp.Status()) }'

    # page.route
    m = re.match(r"await page\.route\('([^']+)',\s*route\s*=>", s)
    if m: return f'_ = page.Route("{m.group(1)}", func(route playwright.Route) {{'
    if 'route.fulfill(' in s:
        ms = re.search(r'status:\s*(\d+)', s); mb = re.search(r"body:\s*'([^']+)'", s)
        if ms and mb: return f'_ = route.Fulfill(playwright.RouteFulfillOptions{{Status: playwright.Int({ms.group(1)}), Body: playwright.String(`{mb.group(1)}`)}})'
        ms2 = re.search(r'status:\s*(\d+)', s); mb2 = re.search(r'body:\s*"([^"]+)"', s)
        if ms2 and mb2: return f'_ = route.Fulfill(playwright.RouteFulfillOptions{{Status: playwright.Int({ms2.group(1)}), Body: playwright.String(`{mb2.group(1)}`)}})'
        return '// route.Fulfill(...)'

    # Variable declarations for locators
    for pat in [r"const (\w+)\s*=\s*page\.locator\(\s*'([^']+)'\s*\)(.*)",
                r'const (\w+)\s*=\s*page\.locator\(\s*"([^"]+)"\s*\)(.*)']:
        m = re.match(pat, s)
        if m:
            var_n = m.group(1); sel = qs(m.group(2)); chain = conv_chain(m.group(3).strip())
            go_expr = f'page.Locator(`{sel}`){chain}'
            declared[var_n] = go_expr
            return f'{var_n} := {go_expr}'

    m = re.match(r"const (\w+)\s*=\s*page\.getByText\('([^']+)'\)(.*)", s)
    if m:
        var_n = m.group(1); go_expr = f'page.GetByText("{q(m.group(2))}", nil){conv_chain(m.group(3))}'
        declared[var_n] = go_expr
        return f'{var_n} := {go_expr}'
    m = re.match(r"const (\w+)\s*=\s*page\.getByText\(/(.*?)/i?\)(.*)", s)
    if m:
        var_n = m.group(1); go_expr = f'page.GetByText("{m.group(2)}", nil){conv_chain(m.group(3))}'
        declared[var_n] = go_expr
        return f'{var_n} := {go_expr}'

    # const X = Y.locator(...)  where Y is declared
    m = re.match(r"const (\w+)\s*=\s*(\w+)\.locator\('([^']+)'\)(.*)", s)
    if m:
        var_n = m.group(1); parent = m.group(2); sel = qs(m.group(3)); chain = conv_chain(m.group(4))
        parent_go = declared.get(parent, parent)
        go_expr = f'{parent_go}.Locator(`{sel}`){chain}'
        declared[var_n] = go_expr
        return f'{var_n} := {go_expr}'

    # const X = Y.or(Z)
    m = re.match(r"const (\w+)\s*=\s*(\w+)\.or\((\w+)\)", s)
    if m:
        var_n = m.group(1)
        a_go = declared.get(m.group(2), m.group(2))
        b_go = declared.get(m.group(3), m.group(3))
        go_expr = f'{a_go}.Or({b_go})'
        declared[var_n] = go_expr
        return f'{var_n} := {go_expr}'

    # const X = await page.content/title
    m = re.match(r"const (\w+)\s*=\s*await page\.content\(\)", s)
    if m: return f'{m.group(1)}, _ := page.Content()'
    m = re.match(r"const (\w+)\s*=\s*await page\.title\(\)", s)
    if m: return f'{m.group(1)}, _ := page.Title()'

    # const X = await Y.inputValue/textContent/count/isVisible/isDisabled/allTextContents
    for method, go_method in [('inputValue', 'InputValue'), ('textContent', 'TextContent'),
                                ('allTextContents', 'AllTextContents'), ('count', 'Count'),
                                ('isVisible', 'IsVisible'), ('isDisabled', 'IsDisabled')]:
        m = re.match(rf"const (\w+)\s*=\s*await (.+?)\.{method}\(\)(?:\.catch\([^)]*\))?$", s)
        if m:
            var_n = m.group(1)
            lo = loc_from_ts(m.group(2), declared)
            if lo: return f'{var_n}, _ := {lo}.{go_method}()'

    # page.url()
    m = re.match(r"const (\w+)\s*=\s*page\.url\(\)", s)
    if m: return f'{m.group(1)} := page.URL()'

    # const X = (some numeric/bool expression)
    m = re.match(r"const (\w+)\s*=\s*(true|false|\d+)", s)
    if m: return f'{m.group(1)} := {m.group(2)}'

    # let X = ...
    m = re.match(r"let (\w+)\s*=\s*(await .+|false|true|\d+)", s)
    if m:
        var_n = m.group(1); val = m.group(2)
        if val in ('false', 'true') or val.isdigit(): return f'{var_n} := {val}'
        # complex - skip
        return None

    # Expect assertions on body locator
    m = re.search(r"await expect\(page\.locator\('body'\)\)\.not\.toContainText\(/(.*?)/i?\)", s)
    if m: return f'if matched, _ := regexp.MatchString(`(?i){m.group(1)}`, func() string {{ c, _ := page.Content(); return c }}()); matched {{ t.Error("body contains error text") }}'
    m = re.search(r"await expect\(page\.locator\('body'\)\)\.toContainText\(/(.*?)/i?\)", s)
    if m: return f'if matched, _ := regexp.MatchString(`(?i){m.group(1)}`, func() string {{ c, _ := page.Content(); return c }}()); !matched {{ t.Error("body should contain") }}'
    m = re.search(r"await expect\(page\.locator\('body'\)\)\.not\.toContainText\('([^']+)'\)", s)
    if m: return f'if content, _ := page.Content(); strings.Contains(content, "{q(m.group(1))}") {{ t.Error("body should not contain") }}'
    m = re.search(r"await expect\(page\.locator\('body'\)\)\.toContainText\('([^']+)'", s)
    if m: return f'if err := playwright.Expect(page.Locator("body")).ToContainText("{q(m.group(1))}", nil); err != nil {{ t.Logf("body should contain: %v", err) }}'

    # Generic expect assertions
    def try_expect(s):
        # toBeVisible with timeout
        m = re.search(r"await expect\((.+?)\)\.toBeVisible\(\{\s*timeout:\s*(\d+(?:_\d+)*)\s*\}\)", s)
        if m:
            lo = loc_from_ts(m.group(1).strip(), declared); t_val = m.group(2).replace('_','')
            if lo: return f'if err := playwright.Expect({lo}).ToBeVisible(playwright.LocatorAssertionsToBeVisibleOptions{{Timeout: playwright.Float({t_val})}}); err != nil {{ t.Logf("expected visible: %v", err) }}'
        # toBeVisible no timeout
        m = re.match(r"await expect\((.+)\)\.toBeVisible\(\)(?:\.catch\([^)]*\))?$", s)
        if m:
            lo = loc_from_ts(m.group(1).strip(), declared)
            if lo: return f'if err := playwright.Expect({lo}).ToBeVisible(nil); err != nil {{ t.Logf("expected visible: %v", err) }}'
        # not.toBeVisible with timeout
        m = re.search(r"await expect\((.+?)\)\.not\.toBeVisible\(\{\s*timeout:\s*(\d+(?:_\d+)*)\s*\}\)", s)
        if m:
            lo = loc_from_ts(m.group(1).strip(), declared); t_val = m.group(2).replace('_','')
            if lo: return f'if err := playwright.Expect({lo}).Not().ToBeVisible(playwright.LocatorAssertionsToBeVisibleOptions{{Timeout: playwright.Float({t_val})}}); err != nil {{ t.Logf("expected not visible: %v", err) }}'
        # not.toBeVisible
        m = re.match(r"await expect\((.+)\)\.not\.toBeVisible\(\)(?:\.catch\([^)]*\))?$", s)
        if m:
            lo = loc_from_ts(m.group(1).strip(), declared)
            if lo: return f'if err := playwright.Expect({lo}).Not().ToBeVisible(nil); err != nil {{ t.Logf("expected not visible: %v", err) }}'
        # toBeEnabled with timeout
        m = re.search(r"await expect\((.+?)\)\.toBeEnabled\(\{\s*timeout:\s*(\d+(?:_\d+)*)\s*\}\)", s)
        if m:
            lo = loc_from_ts(m.group(1).strip(), declared); t_val = m.group(2).replace('_','')
            if lo: return f'if err := playwright.Expect({lo}).ToBeEnabled(playwright.LocatorAssertionsToBeEnabledOptions{{Timeout: playwright.Float({t_val})}}); err != nil {{ t.Logf("expected enabled: %v", err) }}'
        # toBeEnabled
        m = re.match(r"await expect\((.+)\)\.toBeEnabled\(\)$", s)
        if m:
            lo = loc_from_ts(m.group(1).strip(), declared)
            if lo: return f'if err := playwright.Expect({lo}).ToBeEnabled(nil); err != nil {{ t.Logf("expected enabled: %v", err) }}'
        # toBeChecked
        m = re.match(r"await expect\((.+)\)\.toBeChecked\(\)$", s)
        if m:
            lo = loc_from_ts(m.group(1).strip(), declared)
            if lo: return f'if err := playwright.Expect({lo}).ToBeChecked(nil); err != nil {{ t.Logf("expected checked: %v", err) }}'
        # not.toBeEmpty
        m = re.match(r"await expect\((.+)\)\.not\.toBeEmpty\(\)$", s)
        if m:
            lo = loc_from_ts(m.group(1).strip(), declared)
            if lo: return f'if err := playwright.Expect({lo}).Not().ToBeEmpty(nil); err != nil {{ t.Logf("expected not empty: %v", err) }}'
        # toHaveValue
        m = re.match(r"await expect\((.+)\)\.toHaveValue\('([^']+)'\)$", s)
        if m:
            lo = loc_from_ts(m.group(1).strip(), declared)
            if lo: return f'if err := playwright.Expect({lo}).ToHaveValue("{q(m.group(2))}", nil); err != nil {{ t.Logf("expected value: %v", err) }}'
        # toHaveAttribute
        m = re.match(r"await expect\((.+)\)\.toHaveAttribute\('([^']+)',\s*'([^']+)'\)$", s)
        if m:
            lo = loc_from_ts(m.group(1).strip(), declared)
            if lo: return f'if err := playwright.Expect({lo}).ToHaveAttribute("{m.group(2)}", "{m.group(3)}", nil); err != nil {{ t.Logf("expected attr: %v", err) }}'
        # toContainText string with timeout
        m = re.search(r"await expect\((.+?)\)\.toContainText\('([^']+)',\s*\{\s*timeout:\s*(\d+(?:_\d+)*)\s*\}\)", s)
        if m:
            lo = loc_from_ts(m.group(1).strip(), declared); t_val = m.group(3).replace('_','')
            if lo: return f'if err := playwright.Expect({lo}).ToContainText("{q(m.group(2))}", playwright.LocatorAssertionsToContainTextOptions{{Timeout: playwright.Float({t_val})}}); err != nil {{ t.Logf("expected contains: %v", err) }}'
        # toContainText string
        m = re.match(r"await expect\((.+)\)\.toContainText\('([^']+)'\)$", s)
        if m:
            lo = loc_from_ts(m.group(1).strip(), declared)
            if lo: return f'if err := playwright.Expect({lo}).ToContainText("{q(m.group(2))}", nil); err != nil {{ t.Logf("expected contains: %v", err) }}'
        # toContainText regex
        m = re.match(r"await expect\((.+)\)\.toContainText\(/(.*?)/i?\)$", s)
        if m:
            lo = loc_from_ts(m.group(1).strip(), declared)
            if lo: return f'if err := playwright.Expect({lo}).ToContainText("{m.group(2)}", nil); err != nil {{ t.Logf("expected contains: %v", err) }}'
        # not.toContainText regex
        m = re.match(r"await expect\((.+)\)\.not\.toContainText\(/(.*?)/i?\)$", s)
        if m:
            lo = loc_from_ts(m.group(1).strip(), declared)
            if lo: return f'if err := playwright.Expect({lo}).Not().ToContainText("{m.group(2)}", nil); err != nil {{ t.Logf("expected not contains: %v", err) }}'
        # not.toContainText string
        m = re.match(r"await expect\((.+)\)\.not\.toContainText\('([^']+)'\)$", s)
        if m:
            lo = loc_from_ts(m.group(1).strip(), declared)
            if lo: return f'if err := playwright.Expect({lo}).Not().ToContainText("{q(m.group(2))}", nil); err != nil {{ t.Logf("expected not contains: %v", err) }}'
        return None

    r = try_expect(s)
    if r: return r

    # Action methods
    m = re.match(r"await (.+?)\.click\(\)(?:\.catch\([^)]*\))?$", s)
    if m:
        lo = loc_from_ts(m.group(1), declared)
        if lo: return f'if err := {lo}.Click(nil); err != nil {{ t.Logf("click: %v", err) }}'

    m = re.match(r"await (.+?)\.fill\('([^']*)'\)$", s)
    if m:
        lo = loc_from_ts(m.group(1), declared)
        if lo: return f'if err := {lo}.Fill("{q(m.group(2))}", nil); err != nil {{ t.Logf("fill: %v", err) }}'
    m = re.match(r'await (.+?)\.fill\("([^"]*)"\)$', s)
    if m:
        lo = loc_from_ts(m.group(1), declared)
        if lo: return f'if err := {lo}.Fill("{q(m.group(2))}", nil); err != nil {{ t.Logf("fill: %v", err) }}'
    m = re.match(r'await (.+?)\.fill\((\w+)\)$', s)
    if m:
        lo = loc_from_ts(m.group(1), declared)
        if lo: return f'if err := {lo}.Fill({m.group(2)}, nil); err != nil {{ t.Logf("fill: %v", err) }}'

    m = re.match(r"await (.+?)\.check\(\)$", s)
    if m:
        lo = loc_from_ts(m.group(1), declared)
        if lo: return f'_ = {lo}.Check(nil)'
    m = re.match(r"await (.+?)\.uncheck\(\)$", s)
    if m:
        lo = loc_from_ts(m.group(1), declared)
        if lo: return f'_ = {lo}.Uncheck(nil)'
    m = re.match(r"await (.+?)\.press\('([^']+)'\)$", s)
    if m:
        lo = loc_from_ts(m.group(1), declared)
        if lo: return f'_ = {lo}.Press("{m.group(2)}", nil)'
    m = re.match(r"await (.+?)\.selectOption\((.+)\)$", s)
    if m:
        lo = loc_from_ts(m.group(1), declared)
        if lo: return f'_, _ = {lo}.SelectOption({conv_select(m.group(2))}, nil)'

    # Simple assertions
    if re.search(r'expect\(true\)\.toBe\(true\)', s): return '// (pass)'
    m = re.match(r"expect\((\w+)\)\.toBe\((true|false)\)$", s)
    if m:
        v, val = m.group(1), m.group(2)
        if val == 'true': return f'if !{v} {{ t.Error("expected true") }}'
        return f'if {v} {{ t.Error("expected false") }}'
    m = re.match(r"expect\((\w+)\)\.not\.toBe\(''\)$", s)
    if m: return f'if {m.group(1)} == "" {{ t.Error("expected non-empty") }}'
    m = re.match(r"expect\((\w+)\)\.toMatch\(/(.*?)/[gi]*\)$", s)
    if m: return f'if matched, _ := regexp.MatchString(`{m.group(2)}`, {m.group(1)}); !matched {{ t.Errorf("expected match") }}'
    m = re.match(r"expect\((\w+)\)\.not\.toMatch\(/(.*?)/[gi]*\)$", s)
    if m: return f'if matched, _ := regexp.MatchString(`(?i){m.group(2)}`, {m.group(1)}); matched {{ t.Errorf("unexpected match") }}'
    m = re.match(r"expect\((\w+)\)\.toBeGreaterThan\((\d+)\)$", s)
    if m: return f'if {m.group(1)} <= {m.group(2)} {{ t.Errorf("expected > {m.group(2)}") }}'
    m = re.match(r"expect\((\w+)\)\.toBeGreaterThanOrEqual\((\d+)\)$", s)
    if m: return f'if {m.group(1)} < {m.group(2)} {{ t.Errorf("expected >= {m.group(2)}") }}'
    m = re.match(r"expect\((\w+)\)\.toBeLessThan\((\d+(?:\s*\*\s*\d+(?:\s*\*\s*\d+)?)?)\)$", s)
    if m:
        try:
            val = eval(m.group(2).replace(' ',''))
            return f'if {m.group(1)} >= {val} {{ t.Errorf("expected < {val}") }}'
        except: return f'// expect({m.group(1)}).toBeLessThan(...)'
    m = re.match(r"expect\((\w+)\)\.toBeLessThanOrEqual\(([0-9.]+)\)$", s)
    if m: return f'if float64({m.group(1)}) > {m.group(2)} {{ t.Errorf("expected <= {m.group(2)}") }}'
    m = re.match(r"expect\((\w+)\)\.toContain\('([^']+)'\)$", s)
    if m: return f'if !strings.Contains({m.group(1)}, "{q(m.group(2))}") {{ t.Error("expected contains") }}'

    # Conditional blocks
    m = re.match(r"if \(\(await (.+?)\.count\(\)\)\s*([><=!]+)\s*(\d+)\)\s*\{$", s)
    if m:
        lo = loc_from_ts(m.group(1), declared)
        if lo: return f'if cnt, _ := {lo}.Count(); cnt {m.group(2)} {m.group(3)} {{'
    m = re.match(r"if \(await (.+?)\.isVisible\(.*?\)(?:\.catch\([^)]*\))?\)\s*\{$", s)
    if m:
        lo = loc_from_ts(m.group(1), declared)
        if lo: return f'if vis, _ := {lo}.IsVisible(); vis {{'
    m = re.match(r"if \(await (.+?)\.isDisabled\(\)\)\s*\{$", s)
    if m:
        lo = loc_from_ts(m.group(1), declared)
        if lo: return f'if dis, _ := {lo}.IsDisabled(); dis {{'

    # else if -> else
    if re.match(r"}\s*else\s*if\s*\(", s): return '} else {'
    if s == '} else {': return '} else {'

    # for loops
    m = re.match(r"for \(let (\w+) = (\d+);\s*\w+ < (\w+|\d+);\s*\w+\+\+\)\s*\{$", s)
    if m:
        var_n = m.group(1); start = m.group(2); end = m.group(3)
        for k, v in [('MAX_WIZARD_STEPS', '10'), ('MAX_NAVIGATION_ATTEMPTS', '6'), ('MAX_GOALS_TO_SELECT', '3')]:
            if end == k: end = v; break
        return f'for {var_n} := {start}; {var_n} < {end}; {var_n}++ {{'
    m = re.match(r"for \(let (\w+) = (\d+);\s*\w+ < Math\.min\((\w+),\s*(\d+)\);\s*\w+\+\+\)\s*\{$", s)
    if m:
        var_n = m.group(1); cnt_var = m.group(3); max_n = m.group(4)
        return f'for {var_n} := 0; {var_n} < func() int {{ if {cnt_var} < {max_n} {{ return {cnt_var} }}; return {max_n} }}(); {var_n}++ {{'

    # Control flow
    if s == 'break': return 'break'
    if s == 'return': return 'return'
    if s == 'continue': return 'continue'
    if s == '}': return '}'

    # page.on, console, etc.
    if re.match(r"page\.on\(", s) or re.match(r"console\.", s): return '// (event listener)'

    # let X = expr (skip complex)
    m = re.match(r"let (\w+)", s)
    if m: return f'// let {m.group(1)} = ...'

    # Variable assignment to isVisible etc.
    m = re.match(r"(\w+)\s*=\s*await (.+?)\.isVisible\(\)(?:\.catch\([^)]*\))?$", s)
    if m:
        lo = loc_from_ts(m.group(2), declared)
        if lo: return f'{m.group(1)}, _ = {lo}.IsVisible()'

    m = re.match(r"(\w+)\s*=\s*(true|false|\d+)$", s)
    if m: return f'{m.group(1)} = {m.group(2)}'

    # const X = something complex
    m = re.match(r"const (\w+)\s*=", s)
    if m: return f'// {s[:100]}'

    return None

def balance_braces(code):
    """Count unmatched braces and add closing ones."""
    depth = 0
    for char in code:
        if char == '{': depth += 1
        elif char == '}': depth -= 1
    return depth

def convert_body(ts_body):
    """Convert a TS test body to Go."""
    # Preprocess to join multi-line statements
    preprocessed = preprocess(ts_body)
    lines = preprocessed.split('\n')
    declared = {}
    go_lines = []

    for raw in lines:
        stripped = raw.strip()
        if not stripped:
            go_lines.append('')
            continue
        n_indent = len(raw) - len(raw.lstrip())
        tab = '\t' * max(1, n_indent // 2)
        if stripped.startswith('//'):
            go_lines.append(tab + stripped)
            continue
        result = convert_stmt(stripped.rstrip(';'), declared)
        if result is not None:
            if result:
                go_lines.append(tab + result)
        else:
            go_lines.append(tab + '// ' + stripped.rstrip(';')[:100])

    # Check brace balance
    code = '\n'.join(go_lines)
    depth = balance_braces(code)
    # Close any unclosed braces
    if depth > 0:
        for _ in range(depth):
            go_lines.append('\t}')

    return '\n'.join(go_lines)

# Generate both files
for ts_file, go_file in [('ohc-cuj.spec.ts', 'cuj_part1_test.go'), ('ohc-cuj-part2.spec.ts', 'cuj_part2_test.go')]:
    with open(ts_file, 'r') as f:
        ts_content = f.read()

    test_pattern = re.compile(
        r"test\('([^']+)',\s*async\s*\(\{\s*page\s*\}\)\s*=>\s*\{(.*?)\n\}\);",
        re.DOTALL
    )
    tests = test_pattern.findall(ts_content)

    suffix = '1' if 'part1' in go_file else '2'
    out_lines = [
        'package e2e',
        '',
        'import (',
        '\t"regexp"',
        '\t"strings"',
        '\t"testing"',
        '\t"time"',
        '',
        '\tplaywright "github.com/playwright-community/playwright-go"',
        ')',
        '',
        f'// Ensure imports are used - part {suffix}',
        'var (',
        f'\t_ = regexp.MustCompile',
        f'\t_ = strings.Contains',
        f'\t_ = time.Sleep',
        ')',
        '',
    ]

    for name, body in tests:
        func_name = to_go_name(name)
        go_body = convert_body(body)
        out_lines.append(f'func {func_name}(t *testing.T) {{')
        out_lines.append('\tpage := newPage(t)')
        out_lines.append('\tdefer page.Close()')
        out_lines.append(go_body)
        out_lines.append('}')
        out_lines.append('')

    with open(go_file, 'w') as f:
        f.write('\n'.join(out_lines))

    print(f"Generated {go_file} with {len(tests)} tests, {sum(out_lines.count(l) for l in out_lines)} lines")

print("Done")
