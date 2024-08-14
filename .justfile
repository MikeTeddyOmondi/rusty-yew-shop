# Default
default:
  just --list
  
# Trunk serve  
trunk-serve:
  trunk serve

# Compile tailwind
tailwind:
  npx tailwindcss -i ./css/styles.css -o ./css/bundle.css

# Watch compile tailwind
watch-tailwind: 
  npx tailwindcss -i ./css/styles.css -o ./css/bundle.css --watch