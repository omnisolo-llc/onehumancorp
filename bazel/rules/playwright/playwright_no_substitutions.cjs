#!/usr/bin/env node
"use strict";

const fs = require("node:fs");
const path = require("node:path");
const ts = require("typescript");

const AUTH_LOGIN_PATH = "/api/v1/auth/login";
const SAFE_METHODS = new Set(["GET", "HEAD", "OPTIONS"]);
const MUTATING_REQUEST_METHODS = new Set(["post", "put", "patch", "delete"]);
const LOCATOR_REFINEMENTS = new Set(["filter", "first", "last", "nth"]);
const FABRICATED_VALUE_NAME = /(^|_)(dummy|fake|mock|sample)(_|$)|^(dummy|fake|mock|sample)[A-Z]/;

function unwrap(node) {
  while (node && (
    ts.isAwaitExpression(node)
    || ts.isParenthesizedExpression(node)
    || ts.isAsExpression(node)
    || ts.isTypeAssertionExpression(node)
    || ts.isNonNullExpression(node)
    || ts.isSatisfiesExpression(node)
  )) node = node.expression;
  return node;
}

function member(node, evaluateString) {
  node = unwrap(node);
  if (ts.isPropertyAccessExpression(node)) return { receiver: node.expression, name: node.name.text, property: node.name };
  if (ts.isElementAccessExpression(node)) return { receiver: node.expression, name: evaluateString(node.argumentExpression), property: node.argumentExpression };
  return undefined;
}

function resolveLocalImport(importer, specifier) {
  let base;
  if (specifier.startsWith(".")) {
    base = path.resolve(path.dirname(importer), specifier);
  } else if (specifier.startsWith("@/")) {
    const marker = `${path.sep}src${path.sep}ui${path.sep}next${path.sep}src${path.sep}`;
    const absolute = path.resolve(importer);
    const index = absolute.indexOf(marker);
    if (index < 0) return undefined;
    base = path.join(absolute.slice(0, index + marker.length - 1), specifier.slice(2));
  } else {
    return undefined;
  }
  const candidates = [
    base,
    `${base}.ts`,
    `${base}.tsx`,
    `${base}.js`,
    `${base}.mjs`,
    `${base}.cjs`,
    path.join(base, "index.ts"),
    path.join(base, "index.tsx"),
    path.join(base, "index.js"),
  ];
  return candidates.find((candidate) => fs.existsSync(candidate) && fs.statSync(candidate).isFile()) || "";
}

function createProgram(rootNames) {
  const nodeModules = (process.env.NODE_PATH || "").split(path.delimiter).find(Boolean)
    || path.join(process.cwd(), "node_modules");
  const baseUrl = path.dirname(nodeModules);
  return ts.createProgram({
    rootNames,
    options: {
      allowJs: true, skipLibCheck: true, skipDefaultLibCheck: true,
      baseUrl,
      jsx: ts.JsxEmit.ReactJSX,
      module: ts.ModuleKind.NodeNext,
      moduleResolution: ts.ModuleResolutionKind.NodeNext,
      noEmit: true,
      skipLibCheck: true,
      target: ts.ScriptTarget.ES2022,
      paths: { "*": [`${path.basename(nodeModules)}/*`] },
    },
  });
}

function renderedType(checker, node) {
  try {
    return checker.typeToString(checker.getTypeAtLocation(node));
  } catch {
    return "";
  }
}

function typeHasDeclaredSymbol(checker, node, expectedNames, sourcePattern) {
  const seen = new Set();
  function visit(type) {
    if (!type || seen.has(type)) return false;
    seen.add(type);
    if (type.isUnionOrIntersection?.() && type.types.some(visit)) return true;
    for (const symbol of [type.aliasSymbol, type.symbol, type.target?.symbol]) {
      if (!symbol || !expectedNames.has(symbol.getName())) continue;
      if ((symbol.declarations || []).some((declaration) => sourcePattern.test(declaration.getSourceFile().fileName))) {
        return true;
      }
    }
    return type.target && type.target !== type ? visit(type.target) : false;
  }
  try {
    return visit(checker.getTypeAtLocation(node));
  } catch {
    return false;
  }
}

const PLAYWRIGHT_DECLARATION = /node_modules[\\/](?:@playwright[\\/]test|playwright(?:-core)?)[\\/]/;

function commonDirectory(filenames) {
  if (filenames.length === 0) return process.cwd();
  let parts = path.dirname(path.resolve(filenames[0])).split(path.sep);
  for (const filename of filenames.slice(1)) {
    const candidate = path.dirname(path.resolve(filename)).split(path.sep);
    let index = 0;
    while (index < parts.length && index < candidate.length && parts[index] === candidate[index]) index += 1;
    parts = parts.slice(0, index);
  }
  return parts.length === 1 && parts[0] === "" ? path.sep : parts.join(path.sep);
}

function symbolAt(checker, node) {
  const symbol = checker.getSymbolAtLocation(node);
  return symbol && (symbol.flags & ts.SymbolFlags.Alias) ? checker.getAliasedSymbol(symbol) : symbol;
}

function isAmbientSymbol(symbol) {
  const declarations = symbol?.declarations || [];
  return Boolean(symbol)
    && (declarations.length === 0
      || declarations.every((declaration) => declaration.getSourceFile().isDeclarationFile));
}

function declarationInitializer(symbol) {
  for (const declaration of symbol?.declarations || []) {
    if (ts.isVariableDeclaration(declaration) || ts.isParameter(declaration) || ts.isPropertyDeclaration(declaration)) {
      if (declaration.initializer) return declaration.initializer;
      const owner = declaration.parent?.parent;
      if (ts.isVariableDeclaration(declaration) && ts.isForOfStatement(owner)) {
        return { iteration: true, receiver: owner.expression };
      }
    }
    if (ts.isBindingElement(declaration)) {
      const variable = declaration.parent?.parent;
      if (ts.isVariableDeclaration(variable) && variable.initializer) {
        return { binding: declaration, receiver: variable.initializer };
      }
    }
    if (ts.isPropertyAssignment(declaration)) return declaration.initializer;
    if (ts.isShorthandPropertyAssignment(declaration)) return declaration.name;
  }
  return undefined;
}

function makeStringEvaluator(checker) {
  function evaluate(node, seen = new Set()) {
    node = unwrap(node);
    if (!node) return undefined;
    if (ts.isStringLiteralLike(node)) return node.text;
    if (ts.isIdentifier(node)) {
      const symbol = symbolAt(checker, node);
      if (!symbol || seen.has(symbol)) return undefined;
      seen.add(symbol);
      const initializer = declarationInitializer(symbol);
      return initializer && !initializer.binding ? evaluate(initializer, seen) : undefined;
    }
    if (ts.isBinaryExpression(node) && node.operatorToken.kind === ts.SyntaxKind.PlusToken) {
      const left = evaluate(node.left, new Set(seen));
      const right = evaluate(node.right, new Set(seen));
      return left === undefined || right === undefined ? undefined : left + right;
    }
    if (ts.isTemplateExpression(node)) {
      let value = node.head.text;
      for (const span of node.templateSpans) {
        const expression = evaluate(span.expression, new Set(seen));
        if (expression === undefined) return undefined;
        value += expression + span.literal.text;
      }
      return value;
    }
    return undefined;
  }
  return evaluate;
}

function discoverReachable(initialFiles) {
  const files = new Set(initialFiles.map((file) => path.resolve(file)));
  const discoveryFindings = [];
  let changed = true;
  while (changed) {
    changed = false;
    const program = createProgram([...files]);
    const checker = program.getTypeChecker();
    const evaluateString = makeStringEvaluator(checker);
    for (const filename of [...files]) {
      const sourceFile = program.getSourceFile(filename);
      if (!sourceFile) continue;
      function addSpecifier(node, dynamic) {
        const specifier = evaluateString(node);
        if (specifier === undefined) {
          if (dynamic) discoveryFindings.push(["unresolved dynamic import", filename]);
          return;
        }
        const resolved = resolveLocalImport(filename, specifier);
        if (resolved === "") discoveryFindings.push(["unresolved local import", filename]);
        else if (resolved && !files.has(resolved)) {
          files.add(resolved);
          changed = true;
        }
      }
      function visit(node) {
        if ((ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) && node.moduleSpecifier) {
          addSpecifier(node.moduleSpecifier, false);
        }
        if (ts.isImportEqualsDeclaration(node)
          && ts.isExternalModuleReference(node.moduleReference)
          && node.moduleReference.expression) {
          addSpecifier(node.moduleReference.expression, false);
        }
        if (ts.isCallExpression(node)) {
          const expression = unwrap(node.expression);
          if (expression.kind === ts.SyntaxKind.ImportKeyword) {
            discoveryFindings.push(["dynamic import", filename]);
            if (node.arguments.length === 1) addSpecifier(node.arguments[0], true);
            else discoveryFindings.push(["unresolved dynamic import", filename]);
          } else if (ts.isIdentifier(expression) && expression.text === "require") {
            if (node.arguments.length === 1) addSpecifier(node.arguments[0], true);
            else discoveryFindings.push(["unresolved dynamic import", filename]);
          }
        }
        ts.forEachChild(node, visit);
      }
      visit(sourceFile);
    }
  }
  return { files: [...files], findings: discoveryFindings };
}

function scanMarkers(filenames) {
  const program = createProgram(filenames);
  const checker = program.getTypeChecker();
  const evaluateString = makeStringEvaluator(checker);
  const findings = [];

  function isTestFunction(node) {
    return typeHasDeclaredSymbol(checker, unwrap(node), new Set(["TestType"]), PLAYWRIGHT_DECLARATION);
  }

  function isDescribeFunction(node) {
    const access = member(node, evaluateString);
    return access?.name === "describe" && isTestFunction(access.receiver);
  }

  function classifyMarkerMember(receiver, name) {
    const testInfo = typeHasDeclaredSymbol(checker, receiver, new Set(["TestInfo"]), PLAYWRIGHT_DECLARATION);
    if (!name && (isTestFunction(receiver) || testInfo)) return "unresolved Playwright marker is forbidden";
    if (name === "only" && (isTestFunction(receiver) || isDescribeFunction(receiver))) {
      return "focused Playwright tests are forbidden";
    }
    if (["fixme", "skip"].includes(name) && (isTestFunction(receiver) || isDescribeFunction(receiver))) {
      return "skipped Playwright tests are forbidden";
    }
    if (["fixme", "skip"].includes(name) && testInfo) return "runtime Playwright skips are forbidden";
    return undefined;
  }

  function classifyMarkerReference(expression, seen = new Set()) {
    expression = unwrap(expression);
    if (ts.isIdentifier(expression)) {
      const symbol = symbolAt(checker, expression);
      if (!symbol || seen.has(symbol)) return undefined;
      seen.add(symbol);
      const initializer = declarationInitializer(symbol);
      if (initializer?.binding) {
        const property = initializer.binding.propertyName || initializer.binding.name;
        const name = evaluateString(property) || property.getText().replace(/^['"]|['"]$/g, "");
        return classifyMarkerMember(initializer.receiver, name);
      }
      return initializer ? classifyMarkerReference(initializer, seen) : undefined;
    }
    if (ts.isCallExpression(expression)) {
      const access = member(expression.expression, evaluateString);
      if (["apply", "bind", "call"].includes(access?.name)) return classifyMarkerReference(access.receiver, seen);
    }
    const access = member(expression, evaluateString);
    if (!access) return undefined;
    if (["apply", "bind", "call"].includes(access.name)) return classifyMarkerReference(access.receiver, seen);
    const direct = classifyMarkerMember(access.receiver, access.name);
    if (direct) return direct;
    const propertySymbol = access.property ? symbolAt(checker, access.property) : undefined;
    if (!propertySymbol || seen.has(propertySymbol)) return undefined;
    seen.add(propertySymbol);
    const initializer = declarationInitializer(propertySymbol);
    return initializer && !initializer.binding ? classifyMarkerReference(initializer, seen) : undefined;
  }

  for (const filename of filenames) {
    const sourceFile = program.getSourceFile(filename);
    if (!sourceFile) continue;
    const categories = new Set();
    function visit(node) {
      if (ts.isCallExpression(node)) {
        const category = classifyMarkerReference(node.expression);
        if (category) categories.add(category);
      }
      ts.forEachChild(node, visit);
    }
    visit(sourceFile);
    for (const category of [...categories].sort()) findings.push([category, filename]);
  }
  return findings;
}

function scanFiles(filenames) {
  const program = createProgram(filenames);
  const checker = program.getTypeChecker();
  const evaluateString = makeStringEvaluator(checker);
  const entityKinds = new Map();
  const functionReturns = new Map();
  const trackedRoot = commonDirectory(filenames);
  const assignedValues = new Map();

  function propertyAssignmentValue(property) {
    if (!ts.isShorthandPropertyAssignment(property)) return property.initializer;
    const valueSymbol = checker.getShorthandAssignmentValueSymbol?.(property);
    const declaration = valueSymbol?.valueDeclaration || valueSymbol?.declarations?.[0];
    return declaration?.name || property.name;
  }

  function recordAssignedValue(symbol, expression) {
    if (!symbol || !expression) return;
    const values = assignedValues.get(symbol) || [];
    values.push(expression);
    assignedValues.set(symbol, values);
  }

  function effectiveAssignedValue(symbol) {
    const values = assignedValues.get(symbol) || [];
    return values.length > 0 ? values[values.length - 1] : undefined;
  }

  for (const filename of filenames) {
    const sourceFile = program.getSourceFile(filename);
    if (!sourceFile) continue;
    function collectAssignments(node) {
      if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name) && node.initializer) {
        recordAssignedValue(symbolAt(checker, node.name), node.initializer);
      }
      if (ts.isBinaryExpression(node) && node.operatorToken.kind === ts.SyntaxKind.EqualsToken
        && ts.isIdentifier(unwrap(node.left))) {
        recordAssignedValue(symbolAt(checker, unwrap(node.left)), node.right);
      }
      ts.forEachChild(node, collectAssignments);
    }
    collectAssignments(sourceFile);
  }

  function addKind(symbol, kind) {
    if (!symbol) return false;
    const kinds = entityKinds.get(symbol) || new Set();
    const before = kinds.size;
    kinds.add(kind);
    entityKinds.set(symbol, kinds);
    return kinds.size !== before;
  }

  function typeKinds(node) {
    const kinds = new Set();
    const declarations = [
      ["APIRequestContext", "request"],
      ["BrowserContext", "context"],
      ["ElementHandle", "element-handle"],
      ["FileChooser", "file-chooser"],
      ["Frame", "frame"],
      ["Locator", "locator"],
      ["Page", "page"],
      ["Route", "route"],
    ];
    for (const [typeName, kind] of declarations) {
      if (typeHasDeclaredSymbol(checker, node, new Set([typeName]), PLAYWRIGHT_DECLARATION)) kinds.add(kind);
    }
    return kinds;
  }

  function kindsOf(node, seen = new Set()) {
    node = unwrap(node);
    if (!node) return new Set();
    const kinds = typeKinds(node);
    const symbol = ts.isIdentifier(node) || ts.isBindingName(node) ? symbolAt(checker, node) : undefined;
    if (ts.isIdentifier(node) && node.text === "navigator" && isAmbientSymbol(symbol)) kinds.add("navigator");
    if (ts.isIdentifier(node) && node.text === "XMLHttpRequest" && isAmbientSymbol(symbol)) kinds.add("xhr-constructor");
    const globalAccess = member(node, evaluateString);
    const globalReceiver = globalAccess ? unwrap(globalAccess.receiver) : undefined;
    if (globalAccess && ts.isIdentifier(globalReceiver)
      && ["globalThis", "window", "self"].includes(globalReceiver.text)
      && isAmbientSymbol(symbolAt(checker, globalReceiver))) {
      if (globalAccess.name === "navigator") kinds.add("navigator");
      if (globalAccess.name === "XMLHttpRequest") kinds.add("xhr-constructor");
    }
    // Page.request is a Playwright APIRequestContext, but TypeScript can lose
    // that property provenance when Page flowed through a typed helper.
    if (globalAccess?.name === "request"
      && kindsOf(globalAccess.receiver, new Set(seen)).has("page")) {
      kinds.add("request");
    }
    if (symbol && entityKinds.has(symbol)) for (const kind of entityKinds.get(symbol)) kinds.add(kind);
    if (symbol && functionReturns.has(symbol)) for (const kind of functionReturns.get(symbol)) kinds.add(kind);
    if (symbol && !seen.has(symbol)) {
      seen.add(symbol);
      const initializer = declarationInitializer(symbol);
      if (initializer && !initializer.binding) for (const kind of kindsOf(initializer, seen)) kinds.add(kind);
    }
    if (ts.isCallExpression(node)) {
      const access = member(node.expression, evaluateString);
      if (access && LOCATOR_REFINEMENTS.has(access.name) && kindsOf(access.receiver, new Set(seen)).has("locator")) {
        kinds.add("locator");
      }
      const calledSymbol = symbolAt(checker, ts.isIdentifier(node.expression) ? node.expression : node.expression.name || node.expression);
      if (calledSymbol && functionReturns.has(calledSymbol)) {
        for (const kind of functionReturns.get(calledSymbol)) kinds.add(kind);
      }
    }
    if (ts.isNewExpression(node) && kindsOf(node.expression, new Set(seen)).has("xhr-constructor")) {
      kinds.add("xhr");
    }
    return kinds;
  }

  function addKindsToBinding(name, kinds) {
    let changed = false;
    if (ts.isIdentifier(name)) {
      const symbol = symbolAt(checker, name);
      for (const kind of kinds) changed = addKind(symbol, kind) || changed;
    } else if (ts.isObjectBindingPattern(name)) {
      for (const element of name.elements) changed = addKindsToBinding(element.name, kinds) || changed;
    }
    return changed;
  }

  function mapArgumentToParameter(argument, parameter) {
    let changed = false;
    if (ts.isIdentifier(parameter.name)) return addKindsToBinding(parameter.name, kindsOf(argument));
    const resolved = unwrap(argument);
    if (ts.isObjectBindingPattern(parameter.name) && ts.isObjectLiteralExpression(resolved)) {
      for (const binding of parameter.name.elements) {
        const wanted = binding.propertyName?.getText().replace(/^['"]|['"]$/g, "") || binding.name.getText();
        const property = resolved.properties.find((candidate) => {
          if (!ts.isPropertyAssignment(candidate) && !ts.isShorthandPropertyAssignment(candidate)) return false;
          return candidate.name.getText().replace(/^['"]|['"]$/g, "") === wanted;
        });
        if (!property) continue;
        const value = propertyAssignmentValue(property);
        changed = addKindsToBinding(binding.name, kindsOf(value)) || changed;
      }
    }
    return changed;
  }

  let changed = true;
  while (changed) {
    changed = false;
    for (const filename of filenames) {
      const sourceFile = program.getSourceFile(filename);
      if (!sourceFile) continue;
      function propagate(node) {
        if (ts.isVariableDeclaration(node) && node.initializer) {
          changed = addKindsToBinding(node.name, kindsOf(node.initializer)) || changed;
        }
        if (ts.isBinaryExpression(node) && node.operatorToken.kind === ts.SyntaxKind.EqualsToken) {
          changed = addKindsToBinding(node.left, kindsOf(node.right)) || changed;
        }
        if (ts.isCallExpression(node)) {
          const signature = checker.getResolvedSignature(node);
          const declaration = signature?.declaration;
          if (declaration && ts.isFunctionLike(declaration)) {
            node.arguments.forEach((argument, index) => {
              if (declaration.parameters[index]) changed = mapArgumentToParameter(argument, declaration.parameters[index]) || changed;
            });
          }
        }
        if (ts.isReturnStatement(node) && node.expression) {
          let owner = node.parent;
          while (owner && !ts.isFunctionLike(owner)) owner = owner.parent;
          if (owner?.name && ts.isIdentifier(owner.name)) {
            const symbol = symbolAt(checker, owner.name);
            const returns = functionReturns.get(symbol) || new Set();
            const before = returns.size;
            for (const kind of kindsOf(node.expression)) returns.add(kind);
            functionReturns.set(symbol, returns);
            changed = returns.size !== before || changed;
          }
        }
        ts.forEachChild(node, propagate);
      }
      propagate(sourceFile);
    }
  }

  function classifyMethodReference(expression, seen = new Set()) {
    expression = unwrap(expression);
    if (ts.isIdentifier(expression)) {
      const symbol = symbolAt(checker, expression);
      if (!symbol || seen.has(symbol)) return undefined;
      seen.add(symbol);
      const initializer = declarationInitializer(symbol);
      if (initializer?.binding) {
        const property = initializer.binding.propertyName || initializer.binding.name;
        const name = evaluateString(property) || property.getText().replace(/^['"]|['"]$/g, "");
        return classifyMember(initializer.receiver, name);
      }
      if (initializer) return classifyMethodReference(initializer, seen);
      if (expression.text === "fetch" && isAmbientSymbol(symbol)) return { category: "fetch" };
      return undefined;
    }
    if (ts.isCallExpression(expression)) {
      const access = member(expression.expression, evaluateString);
      if (access?.name === "bind") {
        const classification = classifyMethodReference(access.receiver, seen);
        if (!classification) return undefined;
        return {
          ...classification,
          boundArguments: [...(classification.boundArguments || []), ...expression.arguments.slice(1)],
        };
      }
      const calledSymbol = symbolAt(checker, ts.isIdentifier(expression.expression)
        ? expression.expression
        : expression.expression.name || expression.expression);
      if (calledSymbol && !seen.has(calledSymbol)) {
        seen.add(calledSymbol);
        for (const returned of returnedExpressions(calledSymbol)) {
          const classification = classifyMethodReference(returned, new Set(seen));
          if (classification) return classification;
        }
      }
    }
    const access = member(expression, evaluateString);
    if (!access) return undefined;
    if (!access.name) {
      const kinds = kindsOf(access.receiver);
      if (["context", "element-handle", "file-chooser", "frame", "locator", "page", "request", "route"]
        .some((kind) => kinds.has(kind))) {
        return { category: "unresolved Playwright method" };
      }
      return undefined;
    }
    if (["bind", "call", "apply"].includes(access.name)) {
      return classifyMethodReference(access.receiver, seen);
    }
    const direct = classifyMember(access.receiver, access.name);
    if (direct) return direct;
    const propertySymbol = access.property ? symbolAt(checker, access.property) : undefined;
    if (!propertySymbol || seen.has(propertySymbol)) return undefined;
    seen.add(propertySymbol);
    const initializer = declarationInitializer(propertySymbol);
    return initializer && !initializer.binding ? classifyMethodReference(initializer, seen) : undefined;
  }

  function classifyMember(receiver, name) {
    const kinds = kindsOf(receiver);
    if (name === "route" && (kinds.has("page") || kinds.has("context"))) return { category: "network interception" };
    if (name === "setContent" && kinds.has("page")) return { category: "injected page content" };
    if (name === "addInitScript" && (kinds.has("page") || kinds.has("context"))) return { category: "injected page content" };
    if (name === "fulfill" && kinds.has("route")) return { category: "synthetic response" };
    if (name === "setInputFiles" && (kinds.has("locator") || kinds.has("element-handle"))) {
      return { category: "upload", uploadArgumentIndex: 0 };
    }
    if (name === "setFiles" && kinds.has("file-chooser")) {
      return { category: "upload", uploadArgumentIndex: 0 };
    }
    if (name === "setInputFiles" && (kinds.has("page") || kinds.has("frame"))) {
      return { category: "upload", uploadArgumentIndex: 1 };
    }
    if (MUTATING_REQUEST_METHODS.has(name) && kinds.has("request")) return { category: "mutation", method: name.toUpperCase() };
    if (name === "fetch" && kinds.has("request")) return { category: "fetch" };
    if (name === "open" && kinds.has("xhr")) return { category: "xhr" };
    if (name === "sendBeacon" && kinds.has("navigator")) return { category: "beacon" };
    const unwrapped = unwrap(receiver);
    if (name === "fetch" && ts.isIdentifier(unwrapped)
      && ["globalThis", "window", "self"].includes(unwrapped.text)
      && isAmbientSymbol(symbolAt(checker, unwrapped))) {
      return { category: "fetch" };
    }
    return undefined;
  }

  function resolveExpression(expression, seen = new Set()) {
    expression = unwrap(expression);
    if (!ts.isIdentifier(expression)) return expression;
    const symbol = symbolAt(checker, expression);
    if (!symbol || seen.has(symbol)) return expression;
    seen.add(symbol);
    const initializer = declarationInitializer(symbol);
    const assigned = effectiveAssignedValue(symbol);
    if (assigned) return resolveExpression(assigned, seen);
    return initializer && !initializer.binding ? resolveExpression(initializer, seen) : expression;
  }

  function propertyValue(receiver, propertyName, seen = new Set()) {
    receiver = unwrap(receiver);
    if (ts.isIdentifier(receiver)) {
      const symbol = symbolAt(checker, receiver);
      if (!symbol || seen.has(symbol)) return undefined;
      seen.add(symbol);
      const initializer = declarationInitializer(symbol);
      if (!initializer || initializer.binding) return undefined;
      return propertyValue(initializer, propertyName, seen);
    }
    const receiverAccess = member(receiver, evaluateString);
    if (receiverAccess?.name) {
      const nested = propertyValue(receiverAccess.receiver, receiverAccess.name, seen);
      return nested ? propertyValue(nested, propertyName, seen) : undefined;
    }
    if (!ts.isObjectLiteralExpression(receiver)) return undefined;
    let found;
    for (const property of receiver.properties) {
      if (ts.isSpreadAssignment(property)) {
        found = propertyValue(property.expression, propertyName, new Set(seen)) || found;
        continue;
      }
      if (!ts.isPropertyAssignment(property) && !ts.isShorthandPropertyAssignment(property)) continue;
      const name = ts.isComputedPropertyName(property.name)
        ? evaluateString(property.name.expression)
        : property.name.getText().replace(/^['"]|['"]$/g, "");
      if (name !== propertyName) continue;
      found = propertyAssignmentValue(property);
    }
    return found;
  }

  function bindingValue(initializer) {
    if (initializer?.iteration) return initializer.receiver;
    if (!initializer?.binding) return initializer;
    const property = initializer.binding.propertyName || initializer.binding.name;
    const name = evaluateString(property) || property.getText().replace(/^['"]|['"]$/g, "");
    return propertyValue(initializer.receiver, name);
  }

  function returnedExpressions(symbol) {
    const results = [];
    function collect(functionLike) {
      if (!functionLike?.body) return;
      if (!ts.isBlock(functionLike.body)) {
        results.push(functionLike.body);
        return;
      }
      function visit(node) {
        if (node !== functionLike && ts.isFunctionLike(node)) return;
        if (ts.isReturnStatement(node) && node.expression) results.push(node.expression);
        else ts.forEachChild(node, visit);
      }
      visit(functionLike.body);
    }
    for (const declaration of symbol?.declarations || []) {
      if (ts.isFunctionLike(declaration)) collect(declaration);
      if (ts.isVariableDeclaration(declaration) && declaration.initializer
        && ts.isFunctionLike(declaration.initializer)) collect(declaration.initializer);
      if (ts.isPropertyAssignment(declaration) && ts.isFunctionLike(declaration.initializer)) {
        collect(declaration.initializer);
      }
    }
    return results;
  }

  function isNetworkResponseExpression(expression, seen = new Set()) {
    expression = unwrap(expression);
    if (!expression || ts.isNewExpression(expression)) return false;
    if (ts.isIdentifier(expression)) {
      const symbol = symbolAt(checker, expression);
      if (!symbol || seen.has(symbol)) return false;
      seen.add(symbol);
      const initializer = bindingValue(declarationInitializer(symbol));
      const source = effectiveAssignedValue(symbol) || initializer;
      return Boolean(source && isNetworkResponseExpression(source, new Set(seen)));
    }
    if (!ts.isCallExpression(expression)) return false;
    const access = member(expression.expression, evaluateString);
    if (access && kindsOf(access.receiver).has("request")
      && ["delete", "fetch", "get", "head", "patch", "post", "put"].includes(access.name)) return true;
    if (classifyMethodReference(expression.expression)?.category === "fetch") return true;
    const calledSymbol = symbolAt(checker, ts.isIdentifier(expression.expression)
      ? expression.expression
      : expression.expression.name || expression.expression);
    if (!calledSymbol || seen.has(calledSymbol)) return false;
    seen.add(calledSymbol);
    return returnedExpressions(calledSymbol).some((returned) => isNetworkResponseExpression(returned, new Set(seen)));
  }

  function isRealResponseValue(expression, seen = new Set()) {
    expression = unwrap(expression);
    if (!expression) return false;
    if (ts.isIdentifier(expression)) {
      const symbol = symbolAt(checker, expression);
      if (!symbol || seen.has(symbol)) return false;
      seen.add(symbol);
      const initializer = bindingValue(declarationInitializer(symbol));
      const source = effectiveAssignedValue(symbol) || initializer;
      return Boolean(source && isRealResponseValue(source, new Set(seen)));
    }
    if (!ts.isCallExpression(expression)) return false;
    const access = member(expression.expression, evaluateString);
    if (!["body", "json", "text"].includes(access?.name)) return false;
    return isNetworkResponseExpression(access.receiver, seen);
  }

  function isFabricatedValue(expression, seen = new Set()) {
    expression = unwrap(expression);
    if (!expression || isRealResponseValue(expression)) return false;
    if (ts.isIdentifier(expression)) {
      if (FABRICATED_VALUE_NAME.test(expression.text)) return true;
      const symbol = symbolAt(checker, expression);
      if (!symbol || seen.has(symbol)) return false;
      seen.add(symbol);
      const initializer = bindingValue(declarationInitializer(symbol));
      const source = effectiveAssignedValue(symbol) || initializer;
      return Boolean(source && isFabricatedValue(source, new Set(seen)));
    }
    const property = member(expression, evaluateString);
    if (property?.name) {
      const value = propertyValue(property.receiver, property.name, new Set(seen));
      return Boolean(value && isFabricatedValue(value, seen));
    }
    if (ts.isObjectLiteralExpression(expression)
      || ts.isArrayLiteralExpression(expression)
      || ts.isStringLiteralLike(expression)
      || ts.isNumericLiteral(expression)
      || ts.isTemplateExpression(expression)
      || ts.isNoSubstitutionTemplateLiteral(expression)
      || ts.isNewExpression(expression)) return true;
    if (ts.isCallExpression(expression)) {
      const access = member(expression.expression, evaluateString);
      const callee = unwrap(expression.expression);
      const calleeName = access?.name || (ts.isIdentifier(callee) ? callee.text : "");
      if (FABRICATED_VALUE_NAME.test(calleeName)) return true;
      if (["body", "json", "text"].includes(access?.name)) return true;
      if (access?.name === "stringify" || access?.name === "freeze" || access?.name === "seal") {
        return expression.arguments.some((argument) => isFabricatedValue(argument, new Set(seen)));
      }
      if (ts.isIdentifier(callee) && ["String", "Number", "Boolean"].includes(callee.text)) {
        return expression.arguments.some((argument) => isFabricatedValue(argument, new Set(seen)));
      }
      const calledSymbol = symbolAt(checker, ts.isIdentifier(callee) ? callee : callee.name || callee);
      if (calledSymbol && !seen.has(calledSymbol)) {
        seen.add(calledSymbol);
        return returnedExpressions(calledSymbol)
          .some((returned) => isFabricatedValue(returned, new Set(seen)));
      }
      return false;
    }
    if (ts.isConditionalExpression(expression)) {
      return isFabricatedValue(expression.whenTrue, new Set(seen))
        || isFabricatedValue(expression.whenFalse, new Set(seen));
    }
    return ts.isBinaryExpression(expression);
  }

  function optionsContainFabricatedPayload(expression, payloadNames, seen = new Set()) {
    expression = unwrap(expression);
    if (!expression) return false;
    if (ts.isIdentifier(expression)) {
      const symbol = symbolAt(checker, expression);
      if (!symbol || seen.has(symbol)) return false;
      seen.add(symbol);
      const initializer = bindingValue(declarationInitializer(symbol));
      return Boolean(initializer && optionsContainFabricatedPayload(initializer, payloadNames, seen));
    }
    if (!ts.isObjectLiteralExpression(expression)) return false;
    for (const property of expression.properties) {
      if (ts.isSpreadAssignment(property)) {
        if (optionsContainFabricatedPayload(property.expression, payloadNames, new Set(seen))) return true;
        continue;
      }
      if (!ts.isPropertyAssignment(property) && !ts.isShorthandPropertyAssignment(property)) continue;
      const name = ts.isComputedPropertyName(property.name)
        ? evaluateString(property.name.expression)
        : property.name.getText().replace(/^['"]|['"]$/g, "");
      if (!payloadNames.has(name)) continue;
      const value = propertyAssignmentValue(property);
      if (isFabricatedValue(value)) return true;
    }
    return false;
  }

  function staticallyResolvedPath(expression, importer) {
    expression = resolveExpression(expression);
    let value = evaluateString(expression);
    if (value === undefined && ts.isCallExpression(expression)) {
      const access = member(expression.expression, evaluateString);
      if (["join", "resolve"].includes(access?.name)
        && expression.arguments.length > 0
        && ts.isIdentifier(unwrap(expression.arguments[0]))
        && unwrap(expression.arguments[0]).text === "__dirname") {
        const suffix = expression.arguments.slice(1).map((argument) => evaluateString(resolveExpression(argument)));
        if (suffix.every((part) => part !== undefined)) value = path.join(path.dirname(importer), ...suffix);
      }
    }
    if (value === undefined) return undefined;
    const candidates = path.isAbsolute(value)
      ? [path.normalize(value)]
      : [path.resolve(path.dirname(importer), value), path.resolve(process.cwd(), value)];
    return candidates.find((candidate) => {
      const relative = path.relative(trackedRoot, candidate);
      return relative !== ".."
        && !relative.startsWith(`..${path.sep}`)
        && fs.existsSync(candidate)
        && fs.statSync(candidate).isFile();
    });
  }

  function uploadUsesTrackedFile(argumentsList, argumentIndex, importer) {
    if (argumentIndex >= argumentsList.length) return false;
    const value = resolveExpression(argumentsList[argumentIndex]);
    if (ts.isArrayLiteralExpression(value)) {
      return value.elements.length === 0
        || value.elements.every((element) => Boolean(staticallyResolvedPath(element, importer)));
    }
    return Boolean(staticallyResolvedPath(value, importer));
  }

  function methodFromOptions(expression, seen = new Set()) {
    expression = resolveExpression(expression, seen);
    if (!ts.isObjectLiteralExpression(expression)) return { known: false };
    let hasMethod = false;
    let known = true;
    let method;
    for (const property of expression.properties) {
      if (ts.isSpreadAssignment(property)) {
        const spread = methodFromOptions(property.expression, new Set(seen));
        if (!spread.known) known = false;
        if (spread.hasMethod) {
          hasMethod = true;
          known = true;
          method = spread.method;
        }
        continue;
      }
      if (!ts.isPropertyAssignment(property) && !ts.isShorthandPropertyAssignment(property)) continue;
      const name = ts.isComputedPropertyName(property.name)
        ? evaluateString(property.name.expression)
        : property.name.getText().replace(/^['"]|['"]$/g, "");
      if (ts.isComputedPropertyName(property.name) && name === undefined) {
        known = false;
        continue;
      }
      if (name !== "method") continue;
      const value = propertyAssignmentValue(property);
      const resolved = evaluateString(resolveExpression(value));
      if (resolved === undefined) return { known: false };
      hasMethod = true;
      known = true;
      method = resolved.toUpperCase();
    }
    return { hasMethod, known, method };
  }

  function payloadFromOptions(expression, seen = new Set()) {
    expression = resolveExpression(expression, seen);
    if (!ts.isObjectLiteralExpression(expression)) return { known: false };
    let fabricated = false;
    let hasPayload = false;
    let known = true;
    for (const property of expression.properties) {
      if (ts.isSpreadAssignment(property)) {
        const spread = payloadFromOptions(property.expression, new Set(seen));
        if (!spread.known) known = false;
        if (spread.hasPayload) {
          fabricated = spread.fabricated;
          hasPayload = true;
        }
        continue;
      }
      if (!ts.isPropertyAssignment(property) && !ts.isShorthandPropertyAssignment(property)) continue;
      const name = ts.isComputedPropertyName(property.name)
        ? evaluateString(property.name.expression)
        : property.name.getText().replace(/^['"]|['"]$/g, "");
      if (ts.isComputedPropertyName(property.name) && name === undefined) {
        known = false;
        continue;
      }
      if (!["body", "data", "form", "multipart"].includes(name)) continue;
      const value = propertyAssignmentValue(property);
      fabricated = isFabricatedValue(value);
      hasPayload = true;
    }
    return { fabricated, hasPayload, known };
  }

  function isGlobalConstructor(expression, constructorName, seen = new Set()) {
    expression = unwrap(expression);
    if (ts.isIdentifier(expression)) {
      const symbol = symbolAt(checker, expression);
      if (expression.text === constructorName && isAmbientSymbol(symbol)) return true;
      if (!symbol || seen.has(symbol)) return false;
      seen.add(symbol);
      const initializer = bindingValue(declarationInitializer(symbol));
      return Boolean(initializer && isGlobalConstructor(initializer, constructorName, seen));
    }
    const access = member(expression, evaluateString);
    const receiver = access ? unwrap(access.receiver) : undefined;
    return access?.name === constructorName
      && ts.isIdentifier(receiver)
      && ["globalThis", "self", "window"].includes(receiver.text)
      && isAmbientSymbol(symbolAt(checker, receiver));
  }

  function arrayElements(expression) {
    expression = resolveExpression(expression);
    return ts.isArrayLiteralExpression(expression) ? [...expression.elements] : undefined;
  }

  function effectiveCallArguments(call, classification) {
    const access = member(call.expression, evaluateString);
    let invocationArguments = [...call.arguments];
    if (access?.name === "apply"
      && ts.isIdentifier(unwrap(access.receiver))
      && unwrap(access.receiver).text === "Reflect") {
      invocationArguments = arrayElements(invocationArguments[2]) || [];
    } else if (access?.name === "call") invocationArguments = invocationArguments.slice(1);
    else if (access?.name === "apply") invocationArguments = arrayElements(invocationArguments[1]) || [];
    return [...(classification?.boundArguments || []), ...invocationArguments];
  }

  function fetchDetails(argumentsList) {
    let url = argumentsList[0] ? evaluateString(resolveExpression(argumentsList[0])) : undefined;
    const outerOptions = argumentsList[1]
      ? methodFromOptions(argumentsList[1])
      : { hasMethod: false, known: true };
    const first = argumentsList[0] ? resolveExpression(argumentsList[0]) : undefined;
    let requestOptions = { hasMethod: false, known: true };
    let requestPayload = { fabricated: false, hasPayload: false, known: true };
    let locallyConstructedRequest = false;
    if (first && ts.isNewExpression(first) && isGlobalConstructor(first.expression, "Request")) {
      locallyConstructedRequest = true;
      url = first.arguments?.[0] ? evaluateString(resolveExpression(first.arguments[0])) : undefined;
      requestOptions = first.arguments?.[1]
        ? methodFromOptions(first.arguments[1])
        : { hasMethod: false, known: true };
      requestPayload = first.arguments?.[1]
        ? payloadFromOptions(first.arguments[1])
        : { fabricated: false, hasPayload: false, known: true };
    }
    const outerOverridesMethod = outerOptions.known && outerOptions.hasMethod;
    const known = outerOverridesMethod || (outerOptions.known && requestOptions.known);
    const method = outerOverridesMethod
      ? outerOptions.method
      : !known
        ? undefined
        : requestOptions.hasMethod
          ? requestOptions.method
          : "GET";
    const mutates = method === undefined || !SAFE_METHODS.has(method);
    const outerPayload = argumentsList[1]
      ? payloadFromOptions(argumentsList[1])
      : { fabricated: false, hasPayload: false, known: true };
    const effectivePayload = outerPayload.hasPayload ? outerPayload : requestPayload;
    const fabricatedPayload = effectivePayload.fabricated
      || (locallyConstructedRequest && mutates && !effectivePayload.hasPayload);
    return {
      auth: url === AUTH_LOGIN_PATH && known && method === "POST",
      fabricatedPayload,
      method,
      mutates,
    };
  }

  function explicitMutationDetails(methodNode, urlNode) {
    const method = methodNode ? evaluateString(resolveExpression(methodNode))?.toUpperCase() : undefined;
    const url = urlNode ? evaluateString(resolveExpression(urlNode)) : undefined;
    return {
      auth: method === "POST" && url === AUTH_LOGIN_PATH,
      mutates: method === undefined || !SAFE_METHODS.has(method),
    };
  }

  function isSyntheticMutationEndpoint(urlNode) {
    const url = urlNode ? evaluateString(resolveExpression(urlNode)) : undefined;
    return typeof url === "string"
      && /\/(?:dev|mock|simulate)(?:[-_/]|$)/i.test(url);
  }

  function isAmbientBrowserStorage(expression, expectedNames, seen = new Set()) {
    expression = unwrap(expression);
    if (ts.isIdentifier(expression)) {
      const symbol = symbolAt(checker, expression);
      if (expectedNames.has(expression.text) && isAmbientSymbol(symbol)) return true;
      if (!symbol || seen.has(symbol)) return false;
      seen.add(symbol);
      const initializer = bindingValue(declarationInitializer(symbol));
      return Boolean(initializer
        && isAmbientBrowserStorage(initializer, expectedNames, seen));
    }
    const access = member(expression, evaluateString);
    const receiver = access ? unwrap(access.receiver) : undefined;
    return Boolean(access?.name
      && expectedNames.has(access.name)
      && ts.isIdentifier(receiver)
      && ["globalThis", "self", "window"].includes(receiver.text)
      && isAmbientSymbol(symbolAt(checker, receiver)));
  }

  function isBrowserStorageMutationReference(expression, seen = new Set()) {
    expression = unwrap(expression);
    if (ts.isIdentifier(expression)) {
      const symbol = symbolAt(checker, expression);
      if (!symbol || seen.has(symbol)) return false;
      seen.add(symbol);
      const initializer = declarationInitializer(symbol);
      if (initializer?.binding) {
        const property = initializer.binding.propertyName || initializer.binding.name;
        const name = evaluateString(property) || property.getText().replace(/^['"]|['"]$/g, "");
        return name === "setItem"
          && isAmbientBrowserStorage(
            initializer.receiver,
            new Set(["localStorage", "sessionStorage"]),
          );
      }
      return Boolean(initializer
        && isBrowserStorageMutationReference(initializer, seen));
    }
    if (ts.isCallExpression(expression)) {
      const invocation = member(expression.expression, evaluateString);
      return Boolean(invocation?.name === "bind"
        && isBrowserStorageMutationReference(invocation.receiver, seen));
    }
    const access = member(expression, evaluateString);
    if (!access) return false;
    if (["apply", "bind", "call"].includes(access.name)) {
      return isBrowserStorageMutationReference(access.receiver, seen);
    }
    return (access.name === "setItem"
      && isAmbientBrowserStorage(access.receiver, new Set(["localStorage", "sessionStorage"])))
      || (access.name === "open"
        && isAmbientBrowserStorage(access.receiver, new Set(["indexedDB"])));
  }

  const findings = [];
  for (const filename of filenames) {
    const sourceFile = program.getSourceFile(filename);
    if (!sourceFile) continue;
    const categories = new Set();
    function visit(node) {
      if (ts.isBinaryExpression(node)
        && node.operatorToken.kind === ts.SyntaxKind.EqualsToken) {
        const assignment = member(node.left, evaluateString);
        if (assignment
          && isAmbientBrowserStorage(
            assignment.receiver,
            new Set(["localStorage", "sessionStorage"]),
          )) {
          categories.add("fabricated browser storage");
        }
      }
      if (ts.isCallExpression(node)) {
        const expressionAccess = member(node.expression, evaluateString);
        if (isBrowserStorageMutationReference(node.expression)) {
          categories.add("fabricated browser storage");
        }
        const isReflectApply = expressionAccess?.name === "apply"
          && ts.isIdentifier(unwrap(expressionAccess.receiver))
          && unwrap(expressionAccess.receiver).text === "Reflect";
        const classification = isReflectApply && node.arguments[0]
          ? classifyMethodReference(node.arguments[0])
          : classifyMethodReference(node.expression);
        const callArguments = effectiveCallArguments(node, classification);
        if (classification?.category === "network interception"
          || classification?.category === "injected page content"
          || classification?.category === "synthetic response"
          || classification?.category === "unresolved Playwright method") {
          categories.add(classification.category);
        } else if (classification?.category === "upload"
          && !uploadUsesTrackedFile(callArguments, classification.uploadArgumentIndex, filename)) {
          categories.add("fabricated file bytes");
        } else if (classification?.category === "mutation") {
          const auth = classification.method === "POST"
            && callArguments[0]
            && evaluateString(resolveExpression(callArguments[0])) === AUTH_LOGIN_PATH;
          const fabricatedPayload = callArguments[1]
            && optionsContainFabricatedPayload(
              callArguments[1],
              new Set(["body", "data", "form", "multipart"]),
            );
          if (!auth && (isSyntheticMutationEndpoint(callArguments[0]) || fabricatedPayload)) {
            categories.add("fabricated business payload");
          }
        } else if (classification?.category === "fetch") {
          const details = fetchDetails(callArguments);
          if (details.mutates && !details.auth
            && (details.fabricatedPayload || isSyntheticMutationEndpoint(callArguments[0]))) {
            categories.add("fabricated business payload");
          }
        } else if (classification?.category === "xhr") {
          const details = explicitMutationDetails(node.arguments[0], node.arguments[1]);
          if (details.mutates && !details.auth) categories.add("fabricated business payload");
        } else if (classification?.category === "beacon") {
          const url = node.arguments[0] ? evaluateString(resolveExpression(node.arguments[0])) : undefined;
          if (url !== AUTH_LOGIN_PATH) categories.add("fabricated business payload");
        }
      }
      ts.forEachChild(node, visit);
    }
    visit(sourceFile);
    for (const category of [...categories].sort()) findings.push([category, filename]);
  }
  return findings;
}

const markerOnly = process.argv[2] === "--markers-only";
const initial = process.argv.slice(markerOnly ? 3 : 2).map((file) => path.resolve(file));
let failed = false;
for (const filename of initial) {
  if (!fs.existsSync(filename) || !fs.statSync(filename).isFile()) {
    process.stdout.write(`missing imported source\t${filename}\n`);
    failed = true;
  }
}
if (!failed) {
  const reachable = discoverReachable(initial);
  const findings = markerOnly
    ? scanMarkers(reachable.files)
    : [...reachable.findings, ...scanFiles(reachable.files)];
  const unique = new Map(findings.map(([category, filename]) => [`${category}\0${filename}`, [category, filename]]));
  for (const [category, filename] of unique.values()) process.stdout.write(`${category}\t${filename}\n`);
  failed = unique.size > 0;
}
process.exitCode = failed ? 1 : 0;
