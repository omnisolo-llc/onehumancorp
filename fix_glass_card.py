import re

# Update GlassCard to accept color
with open('srcs/app/lib/widgets/glass_card.dart', 'r') as f:
    content = f.read()

content = content.replace('final Widget child;\n  final EdgeInsetsGeometry? margin;', 'final Widget child;\n  final EdgeInsetsGeometry? margin;\n  final Color? color;')
content = content.replace('const GlassCard({super.key, required this.child, this.margin});', 'const GlassCard({super.key, required this.child, this.margin, this.color});')

content = content.replace('final color = Theme.of(context).colorScheme.primary;', 'final color = widget.color ?? Theme.of(context).colorScheme.primary;')

with open('srcs/app/lib/widgets/glass_card.dart', 'w') as f:
    f.write(content)

# Update WizardScreen to pass color
with open('srcs/app/lib/screens/wizard_screen.dart', 'r') as f:
    content = f.read()

content = content.replace('child: GlassCard(\n          child: ListTile(\n            leading: Icon(\n              Icons.check_circle,', 'child: GlassCard(\n          color: Theme.of(context).colorScheme.primary,\n          child: ListTile(\n            leading: Icon(\n              Icons.check_circle,')
content = content.replace('child: GlassCard(\n        child: ListTile(\n          leading: Icon(\n            Icons.warning_amber,', 'child: GlassCard(\n        color: Theme.of(context).colorScheme.error,\n        child: ListTile(\n          leading: Icon(\n            Icons.warning_amber,')
content = content.replace('child: GlassCard(\n        child: ListTile(\n          leading: Icon(icon', 'child: GlassCard(\n        color: color,\n        child: ListTile(\n          leading: Icon(icon')

with open('srcs/app/lib/screens/wizard_screen.dart', 'w') as f:
    f.write(content)

# Remove unused imports
import glob

dart_files = glob.glob('srcs/app/lib/screens/*.dart')
for file in dart_files:
    with open(file, 'r') as f:
        content = f.read()

    if "import '../widgets/glass_card.dart';" in content and "GlassCard(" not in content:
        content = content.replace("import '../widgets/glass_card.dart';\n", '')
        with open(file, 'w') as f:
            f.write(content)
