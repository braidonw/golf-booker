dev-web:
  cargo watch -x run

dev-tailwind:
  npx tailwindcss -i input.css -o assets/styles.css --watch=always

dev: 
  #!/bin/sh
  just dev-tailwind &
  pid1=$!
  just dev-web &
  pid2=$!
  trap "kill $pid1 $pid2" EXIT
  wait $pid1 $pid2

