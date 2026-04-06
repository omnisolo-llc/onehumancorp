import re

with open('srcs/app/lib/screens/handoffs_screen.dart', 'r') as f:
    content = f.read()

content = content.replace("final colors = Theme.of(context).colorScheme;", "")

search_str = """  @override
  Widget build(BuildContext context) {

    return Semantics("""

replace_str = """  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return Semantics("""

content = content.replace(search_str, replace_str)

with open('srcs/app/lib/screens/handoffs_screen.dart', 'w') as f:
    f.write(content)
