#!/usr/bin/env sh

cargo build --release

cp target/release/link-router ~/.local/bin/
cp link-router.desktop ~/.local/share/applications
xdg-settings set default-web-browser link-router.desktop
