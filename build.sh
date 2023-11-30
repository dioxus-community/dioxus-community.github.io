# REQUIRED BINARIES:
# - dioxus-cli (https://dioxuslabs.com/learn/0.4/CLI/installation)
# - mdbook (https://rust-lang.github.io/mdBook/guide/installation.html)
# - tailwindcss (https://tailwindcss.com/docs/installation)

set -x -e

book_dir="./books"
book_out_dir="./dist/guides"

echo "Creating necessary directories"
mkdir -p dist/guides

echo "Building Dioxus"
dx build --release

echo "Building books"
for book in "$book_dir"/*; do
    if [ -d "$book" ]; then
        mdbook build "$book"
        mv "$book/book" "$book_out_dir/${book##*/}"
    fi
done

echo "Adding .nojekyll and fallback 404"
touch dist/.nojekyll
cp dist/index.html dist/404.html