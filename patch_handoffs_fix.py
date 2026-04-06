with open('srcs/app/lib/screens/handoffs_screen.dart', 'r') as f:
    content = f.read()

search_str = """  @override
  Widget build(BuildContext context) {


    return Scaffold("""

replace_str = """  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return Scaffold("""

content = content.replace(search_str, replace_str)

with open('srcs/app/lib/screens/handoffs_screen.dart', 'w') as f:
    f.write(content)
