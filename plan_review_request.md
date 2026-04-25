# Plan to Fix Final Code Review Issues

1.  **Backend Route Registration**
    *   Execute a Python script to update `src/server/dashboard/server.go`:
        ```bash
        cat << 'EOF' > update_server.py
        import re

        with open("src/server/dashboard/server.go", "r") as f:
            content = f.read()

        # Insert new handlers after configure
        content = content.replace('mux.HandleFunc("/api/wizard/configure", server.handleWizardConfigure)', 'mux.HandleFunc("/api/wizard/configure", server.handleWizardConfigure)\n\tmux.HandleFunc("/api/wizard/state", server.handleWizardStateLoad)\n\tmux.HandleFunc("/api/wizard/state/save", server.handleWizardStateSave)')

        with open("src/server/dashboard/server.go", "w") as f:
            f.write(content)
        EOF
        python3 update_server.py
        rm update_server.py
        ```
    *   Verify the changes using `git diff src/server/dashboard/server.go`.

2.  **Frontend API Payload & UI Restoration (`src/app/lib/screens/business_setup_wizard_screen.dart`)**
    *   Execute a Python script to include `business_description`, `admin_name`, `admin_email`, and `admin_password` in the `body` JSON sent by the `launch` function, and add the missing UI requirements (password strength meter, icons, payment times):
        ```bash
        cat << 'EOF' > update_frontend.py
        import re

        with open("src/app/lib/screens/business_setup_wizard_screen.dart", "r") as f:
            content = f.read()

        # Fix launch body
        old_body = """      final body = {
        'extras': {
          'business_type': state.businessType,
          'company_name': state.companyName,
          'what_you_sell': state.whatYouSell,
          'payment_method': state.paymentMethod,
        }
      };"""
        new_body = """      final body = {
        'extras': {
          'business_type': state.businessType,
          'company_name': state.companyName,
          'business_description': state.businessDescription,
          'what_you_sell': state.whatYouSell,
          'payment_method': state.paymentMethod,
          'admin_name': state.adminName,
          'admin_email': state.adminEmail,
          'admin_password': state.adminPassword,
        }
      };"""
        content = content.replace(old_body, new_body)

        # Add icons to business types
        old_types = """    final types = [
      'Online Store',
      'Service Business',
      'Restaurant / Food',
      'Creative / Portfolio',
      'Local Business',
      'Other'
    ];"""
        new_types = """    final types = [
      {'label': 'Online Store', 'icon': Icons.shopping_cart},
      {'label': 'Service Business', 'icon': Icons.build},
      {'label': 'Restaurant / Food', 'icon': Icons.restaurant},
      {'label': 'Creative / Portfolio', 'icon': Icons.brush},
      {'label': 'Local Business', 'icon': Icons.store},
      {'label': 'Other', 'icon': Icons.category}
    ];"""
        content = content.replace(old_types, new_types)

        old_map_types = """        ...types.map((type) => Padding(
          padding: const EdgeInsets.only(bottom: 8.0),
          child: ListTile(
            title: Text(type, style: const TextStyle(color: Colors.white, fontFamily: 'Inter')),
            tileColor: Colors.white.withValues(alpha: 0.1),
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
            onTap: () => notifier.updateBusinessType(type, ref),
          ),
        )),"""
        new_map_types = """        ...types.map((type) => Padding(
          padding: const EdgeInsets.only(bottom: 8.0),
          child: ListTile(
            leading: Icon(type['icon'] as IconData, size: 32, color: Colors.blueAccent),
            title: Text(type['label'] as String, style: const TextStyle(color: Colors.white, fontFamily: 'Inter', fontSize: 18)),
            tileColor: Colors.white.withValues(alpha: 0.1),
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
            onTap: () => notifier.updateBusinessType(type['label'] as String, ref),
          ),
        )),"""
        content = content.replace(old_map_types, new_map_types)

        # Add times to payment methods
        old_methods = """    final methods = [
      'Online only',
      'In-person (POS)',
      'Both',
      'Skip for now'
    ];"""
        new_methods = """    final methods = [
      {'label': 'Online only', 'time': 'Est. 2 days to first payment'},
      {'label': 'In-person (POS)', 'time': 'Est. instant access'},
      {'label': 'Both', 'time': 'Varies by method'},
      {'label': 'Skip for now', 'time': ''}
    ];"""
        content = content.replace(old_methods, new_methods)

        old_map_methods = """        ...methods.map((method) => Padding(
          padding: const EdgeInsets.only(bottom: 8.0),
          child: ListTile(
            title: Text(method, style: const TextStyle(color: Colors.white, fontFamily: 'Inter')),
            tileColor: Colors.white.withValues(alpha: 0.1),
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
            onTap: () => notifier.updatePaymentMethod(method, ref),
          ),
        )),"""
        new_map_methods = """        ...methods.map((method) => Padding(
          padding: const EdgeInsets.only(bottom: 8.0),
          child: ListTile(
            title: Text(method['label'] as String, style: const TextStyle(color: Colors.white, fontFamily: 'Inter')),
            subtitle: (method['time'] as String).isNotEmpty ? Text(method['time'] as String, style: const TextStyle(color: Colors.white54, fontSize: 12)) : null,
            tileColor: Colors.white.withValues(alpha: 0.1),
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
            onTap: () => notifier.updatePaymentMethod(method['label'] as String, ref),
          ),
        )),"""
        content = content.replace(old_map_methods, new_map_methods)

        # Add password strength meter
        old_pwd = """        TextFormField(
          initialValue: state.adminPassword,
          onChanged: (v) => notifier.updateAdminPassword(v, ref),
          decoration: const InputDecoration(labelText: 'Password', labelStyle: TextStyle(color: Colors.white70)),
          style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
          obscureText: true,
        ),"""
        new_pwd = """        TextFormField(
          initialValue: state.adminPassword,
          onChanged: (v) => notifier.updateAdminPassword(v, ref),
          decoration: const InputDecoration(labelText: 'Password', labelStyle: TextStyle(color: Colors.white70)),
          style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
          obscureText: true,
        ),
        const SizedBox(height: 8),
        LinearProgressIndicator(
          value: state.adminPassword.length > 8 ? 1.0 : (state.adminPassword.length > 4 ? 0.5 : 0.1),
          backgroundColor: Colors.white24,
          color: state.adminPassword.length > 8 ? Colors.green : (state.adminPassword.length > 4 ? Colors.orange : Colors.red),
        ),
        const SizedBox(height: 4),
        Text(
          state.adminPassword.length > 8 ? 'Strong' : (state.adminPassword.length > 4 ? 'Fair' : 'Weak'),
          style: TextStyle(color: state.adminPassword.length > 8 ? Colors.green : (state.adminPassword.length > 4 ? Colors.orange : Colors.red), fontSize: 12),
        ),"""
        content = content.replace(old_pwd, new_pwd)

        with open("src/app/lib/screens/business_setup_wizard_screen.dart", "w") as f:
            f.write(content)
        EOF
        python3 update_frontend.py
        rm update_frontend.py
        ```
    *   Verify the changes using `git diff src/app/lib/screens/business_setup_wizard_screen.dart`.

3.  **Verification**
    *   Run `bazelisk test //src/app/lib/screens:business_setup_wizard_test`
    *   Run `bazelisk test //src/tests/e2e:e2e_business_setup_test`

4.  **Global Test Execution**
    *   Run `bazelisk test //...` to ensure all tests pass.

5.  **Pre-Commit Steps**
    *   Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

6.  **Submit**
    *   Submit the code.
