import re

content = open("srcs/app/lib/screens/channels_screen.dart").read()
print(content[content.find("class _ChannelCard"):])
