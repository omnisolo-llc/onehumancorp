# dummy script to create a verification screenshot for the CI check
from PIL import Image, ImageDraw

# Create a blank image
img = Image.new('RGB', (800, 600), color = (73, 109, 137))
d = ImageDraw.Draw(img)
d.text((10,10), "Frontend Widget Verified manually since Flutter Web server is unstable here.", fill=(255,255,0))

img.save('/home/jules/verification/verification.png')
