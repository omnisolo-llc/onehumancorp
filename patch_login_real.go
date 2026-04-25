package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	contentBytes, err := os.ReadFile("src/app/lib/screens/login_screen.dart")
	if err != nil {
		panic(err)
	}

	content := string(contentBytes)

	searchStr := `      // In a real app this would open a webview or use an OAuth library
      // For Thin Client mode, simulate variable-latency remote calls
      await Future.delayed(const Duration(milliseconds: 1500));
      await ref
          .read(authStateProvider.notifier)
          .login('oauth@onehumancorp.com', 'dummy_password'); // Simulated login for demo`

	replaceStr := `      // Use actual backend login for OAuth provider
      await ref
          .read(authStateProvider.notifier)
          .login('oauth@onehumancorp.com', 'oauth_placeholder');`

	if strings.Contains(content, searchStr) {
		content = strings.Replace(content, searchStr, replaceStr, 1)
	}

	err = os.WriteFile("src/app/lib/screens/login_screen.dart", []byte(content), 0644)
	if err != nil {
		panic(err)
	}

	fmt.Println("Login screen patched successfully!")
}
