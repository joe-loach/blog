dev:
    npx wrangler dev --local

css:
    npx @tailwindcss/cli -i ./src/global.css -o ./public/tailwind.css --watch

deploy:
    npx @tailwindcss/cli -i ./src/global.css -o ./public/tailwind.css
    npx wrangler deploy
