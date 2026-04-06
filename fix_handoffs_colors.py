with open('srcs/app/lib/screens/handoffs_screen.dart', 'r') as f:
    content = f.read()

# Make sure it only has one instance of "final colors =" inside _HandoffCardState.build

search_str = """  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return Semantics("""

replace_str = """  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return Semantics("""

if "final colors = Theme.of(context).colorScheme;" not in content:
    content = content.replace("  @override\n  Widget build(BuildContext context) {\n    return Semantics(", "  @override\n  Widget build(BuildContext context) {\n    final colors = Theme.of(context).colorScheme;\n    return Semantics(")

with open('srcs/app/lib/screens/handoffs_screen.dart', 'w') as f:
    f.write(content)
