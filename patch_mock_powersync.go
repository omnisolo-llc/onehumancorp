package main

import (
    "io/ioutil"
    "strings"
)

func main() {
    b, err := ioutil.ReadFile("srcs/app/lib/main.dart")
    if err != nil { panic(err) }
    content := string(b)

    // add import
    importSection := `import 'dart:io';`
    if !strings.Contains(content, "import 'dart:io';") {
        content = strings.Replace(content, "import 'package:flutter/material.dart';", "import 'package:flutter/material.dart';\n"+importSection, 1)
    }

    content = strings.Replace(content, "Future.microtask(() => ref.read(powerSyncProvider).init());", "if (!Platform.environment.containsKey('FLUTTER_TEST')) {\n      Future.microtask(() => ref.read(powerSyncProvider).init());\n    }", 1)

    ioutil.WriteFile("srcs/app/lib/main.dart", []byte(content), 0644)
}
