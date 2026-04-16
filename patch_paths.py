import os
import glob

for filename in glob.glob('srcs/app/test/widgets/*_test.dart'):
    with open(filename, 'r') as f:
        content = f.read()
    content = content.replace("import '../../lib/widgets/", "import 'package:ohc_app/widgets/")
    content = content.replace("import '../lib/widgets/", "import 'package:ohc_app/widgets/")
    with open(filename, 'w') as f:
        f.write(content)
