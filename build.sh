# REQUIRED BINARIES:
# - dioxus-cli (https://dioxuslabs.com/learn/0.4/CLI/installation)
# - tailwindcss (https://tailwindcss.com/docs/installation)

set -x -e

out_dir="./dist"
book_dir="./guides/books"
book_out_dir="${out_dir}/guides/books"

echo "Cleaning"
cargo clean
dx clean

echo "Creating necessary directories"
mkdir -p "$book_out_dir"

echo "Building Dioxus"
dx build --release

echo "Adding .nojekyll and fallback 404"
touch "$out_dir"/.nojekyll
cp "$out_dir"/index.html "$out_dir"/404.html