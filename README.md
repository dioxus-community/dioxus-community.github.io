Website for showing off all sorts of cool stuff.
Just go to <https://dioxus-community.github.io>!
You can find everything there.

# Contributing

For styling, we use Tailwind, but there is a `build.rs` which should handle building the output file for you.
All you need to do is use any Tailwind classes wherever you'd like!

If the build script fails to run Tailwind, it will let you know. In that case, run:

```bash
npx tailwindcss -i ./src/input.css -o ./public/tailwind.css --watch --minify
```

Have a look at the [Tailwind docs](https://tailwindcss.com/docs/installation) for more details about the setup.