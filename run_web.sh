#!/bin/bash
cd srcs/app
flutter run -d web-server --web-port 8080 > flutter_run.log 2>&1 &
