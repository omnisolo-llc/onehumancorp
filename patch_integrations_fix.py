with open('srcs/app/lib/screens/integrations_screen.dart', 'r') as f:
    content = f.read()

search_str = """class _IntegrationCardState extends State<_IntegrationCard> {
  @override
  Widget build(BuildContext context) {"""

replace_str = """class _IntegrationCardState extends State<_IntegrationCard> {
  bool _hovering = false;

  @override
  Widget build(BuildContext context) {"""

if "bool _hovering = false;" not in content.split("class _IntegrationCardState extends State<_IntegrationCard> {")[1].split("@override")[0]:
    content = content.replace(search_str, replace_str)

with open('srcs/app/lib/screens/integrations_screen.dart', 'w') as f:
    f.write(content)
