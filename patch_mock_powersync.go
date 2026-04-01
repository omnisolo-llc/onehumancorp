package main

import (
    "io/ioutil"
    "strings"
)

func main() {
    b, err := ioutil.ReadFile("srcs/app/lib/services/powersync_service.dart")
    if err != nil { panic(err) }
    content := string(b)

    // We mock powerSyncProvider in test
    // Actually the test environment does not have the `.so` for PowerSync.
    // I can add sqlite3_flutter_libs to fix `libpowersync_x64.so` issue.
    // Or better, let's add powersync_flutter_libs to pubspec!
}
