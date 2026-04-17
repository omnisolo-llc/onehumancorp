with open('srcs/app/lib/screens/agent_hire_wizard_screen.dart', 'r') as f:
    content = f.read()

# I reverted the dart file implicitly via the reset earlier, so I need to re-apply the agent hire logic changes!

old_role_section = """                Wrap(
                  spacing: 12,
                  runSpacing: 12,
                  children:
                      _roles.map((role) {
                        final isSelected = _selectedRole == role;
                        final dummyAgent = Agent(id: '', name: '', role: role, status: '', organizationId: '', createdAt: DateTime.now());
                        return ChoiceChip(
                          label: Text(dummyAgent.formattedRole),
                          selected: isSelected,
                          onSelected: (selected) {
                            setState(
                              () => _selectedRole = selected ? role : '',
                            );
                            if (selected && _nameController.text.isEmpty) {
                              _nameController.text =
                                  'Senior ${dummyAgent.formattedRole}';
                            }
                          },
                        );
                      }).toList(),
                ),"""

new_role_section = """                GridView.builder(
                  shrinkWrap: true,
                  physics: const NeverScrollableScrollPhysics(),
                  gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
                    crossAxisCount: 2,
                    childAspectRatio: 2.5,
                    crossAxisSpacing: 12,
                    mainAxisSpacing: 12,
                  ),
                  itemCount: _roles.length,
                  itemBuilder: (context, index) {
                    final role = _roles[index];
                    final isSelected = _selectedRole == role;
                    final dummyAgent = Agent(id: '', name: '', role: role, status: '', organizationId: '', createdAt: DateTime.now());
                    return Card(
                      color: isSelected ? Theme.of(context).colorScheme.primaryContainer : Theme.of(context).colorScheme.surface,
                      elevation: isSelected ? 4 : 1,
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(12),
                        side: BorderSide(
                          color: isSelected ? Theme.of(context).colorScheme.primary : Colors.transparent,
                          width: 2,
                        ),
                      ),
                      child: InkWell(
                        onTap: () {
                          setState(() => _selectedRole = role);
                          if (_nameController.text.isEmpty) {
                            _nameController.text = 'Senior ${dummyAgent.formattedRole}';
                          }
                        },
                        borderRadius: BorderRadius.circular(12),
                        child: Center(
                          child: Padding(
                            padding: const EdgeInsets.all(8.0),
                            child: Text(
                              dummyAgent.formattedRole,
                              textAlign: TextAlign.center,
                              style: TextStyle(
                                fontWeight: isSelected ? FontWeight.bold : FontWeight.normal,
                                color: isSelected ? Theme.of(context).colorScheme.onPrimaryContainer : Theme.of(context).colorScheme.onSurface,
                              ),
                            ),
                          ),
                        ),
                      ),
                    );
                  },
                ),"""

content = content.replace(old_role_section, new_role_section)

# Ensure labels are plain language in topology
content = content.replace(
    "RadioListTile<String>(\n                  title: const Text('Independent Worker'),",
    "RadioListTile<String>(\n                  title: const Text('Independent Worker (No Sub-agents)'),"
)
content = content.replace(
    "RadioListTile<String>(\n                  title: const Text('Delegator / Supervisor'),",
    "RadioListTile<String>(\n                  title: const Text('Delegator / Supervisor (Manages Sub-agents)'),"
)

with open('srcs/app/lib/screens/agent_hire_wizard_screen.dart', 'w') as f:
    f.write(content)
