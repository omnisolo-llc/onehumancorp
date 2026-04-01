package main

import (
    "io/ioutil"
    "strings"
)

func main() {
    b, err := ioutil.ReadFile("srcs/app/lib/screens/dashboard_screen.dart")
    if err != nil { panic(err) }
    content := string(b)

    // Oh, since "import 'package:powersync/powersync.dart';" exposes `Column` which conflicts with `flutter/material.dart` `Column`,
    // I need to change `import 'package:powersync/powersync.dart';` to `import 'package:powersync/powersync.dart' hide Column, Row, Table;`
    content = strings.Replace(content, "snapshot.data?.anyDownloading", "snapshot.data?.downloading", 1)

    ioutil.WriteFile("srcs/app/lib/screens/dashboard_screen.dart", []byte(content), 0644)
}
