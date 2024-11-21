#!/bin/bash

set -e
cd "$(dirname "$0")/.."

case "$1" in
  devel)
    MESON_FLAGS="-Dprofile=development"
    ICON_PATH="AppDir/usr/share/icons/hicolor/scalable/apps/es.danirod.Cartero.Devel.svg"
    DESKTOP_PATH="AppDir/usr/share/applications/es.danirod.Cartero.Devel.desktop"
    ;;
  stable)
    MESON_FLAGS="-Dprofile=default"
    ICON_PATH="AppDir/usr/share/icons/hicolor/scalable/apps/es.danirod.Cartero.svg"
    DESKTOP_PATH="AppDir/usr/share/applications/es.danirod.Cartero.desktop"
    ;;
  *)
    echo "Usage: $0 [devel / stable]"
    exit 1
    ;;
esac

meson setup build --prefix="/" $MESON_FLAGS
ninja -C build
DESTDIR=$PWD/build/appimagetool/AppDir/usr ninja -C build install

cd build/appimagetool
[ -x appimagetool-x86_64.AppImage ] || curl -OL https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage
[ -x linuxdeploy-x86_64.AppImage ] || curl -OL https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
[ -x linuxdeploy-plugin-gtk.sh ] || curl -OL https://raw.githubusercontent.com/linuxdeploy/linuxdeploy-plugin-gtk/master/linuxdeploy-plugin-gtk.sh
chmod +x appimagetool-x86_64.AppImage linuxdeploy-x86_64.AppImage linuxdeploy-plugin-gtk.sh

DEPLOY_GTK_VERSION=4

export DEPLOY_GTK_VERSION

# First iteration
./linuxdeploy-x86_64.AppImage --appdir AppDir --plugin gtk --output appimage \
  --executable AppDir/usr/bin/cartero \
  --icon-file "$ICON_PATH" \
  --desktop-file "$DESKTOP_PATH"
  
# Patch the hook in order to support Adwaita theme.
sed -i '/GTK_THEME/d' AppDir/apprun-hooks/linuxdeploy-plugin-gtk.sh
sed -i '/GDK_BACKEND/d' AppDir/apprun-hooks/linuxdeploy-plugin-gtk.sh

# Extra fixes for the icon theme.
gtk4-update-icon-cache -q -t -f AppDir/usr/share/icons/hicolor

# Recompile with the changes.
./appimagetool-x86_64.AppImage AppDir