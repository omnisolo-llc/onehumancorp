#!/bin/bash
/home/jules/go/bin/bazelisk analyze-profile $(/home/jules/go/bin/bazelisk info output_base)/command.profile.gz || true
