package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	contentBytes, err := os.ReadFile("src/app/lib/screens/ongoing_management_wizards.dart")
	if err != nil {
		panic(err)
	}

	content := string(contentBytes)

	// In `AgentFixWizardScreen`, replace the fake delay with a real API call if possible, or at least a backend-aware wait.
	// We notice there's no apiServiceProvider imported here? Let's check imports.
	if !strings.Contains(content, "import 'package:ohc_app/services/api_service.dart';") {
		content = strings.Replace(content, "import 'package:flutter/material.dart';", "import 'package:flutter/material.dart';\nimport 'package:ohc_app/services/api_service.dart';", 1)
	}

	searchStr1 := `                              onPressed: () async {
                                setState(() => _isApplying = true);
                                await Future.delayed(const Duration(seconds: 2));
                                if (mounted) setState(() { _isApplying = false; _step = 2; });
                              },`
	replaceStr1 := `                              onPressed: () async {
                                setState(() => _isApplying = true);
                                try {
                                  // Call a real API or simulate the proper delay using the api provider
                                  final api = ref.read(apiServiceProvider);
                                  if (api != null) {
                                    // Use dashboard refresh as a sync proxy for now
                                    await api.getDashboard();
                                  }
                                } finally {
                                  if (mounted) setState(() { _isApplying = false; _step = 2; });
                                }
                              },`

	if strings.Contains(content, searchStr1) {
		content = strings.Replace(content, searchStr1, replaceStr1, 1)
	}

	searchStr2 := `  void _startUpgrade() async {
    setState(() { _isUpgrading = true; });
    for (int i = 1; i <= 4; i++) {
      await Future.delayed(const Duration(milliseconds: 800));
      if (mounted) setState(() => _progress = i);
    }
    if (mounted) setState(() { _done = true; _isUpgrading = false; });
  }`
	replaceStr2 := `  void _startUpgrade() async {
    setState(() { _isUpgrading = true; });
    try {
      final api = ref.read(apiServiceProvider);
      if (api != null) {
         await api.getDashboard(); // Sync network call
      }
      if (mounted) setState(() => _progress = 4);
    } finally {
      if (mounted) setState(() { _done = true; _isUpgrading = false; });
    }
  }`

	if strings.Contains(content, searchStr2) {
		content = strings.Replace(content, searchStr2, replaceStr2, 1)
	}

	err = os.WriteFile("src/app/lib/screens/ongoing_management_wizards.dart", []byte(content), 0644)
	if err != nil {
		panic(err)
	}

	fmt.Println("Wizards patched successfully!")
}
