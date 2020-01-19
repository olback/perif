#!/bin/bash

### Prep

SNAP_BUILD=_snap_build

rm -rf snap/gui
mkdir snap/gui
cp data/net.olback.Perif.desktop snap/gui
sed -i -e 's/Icon=net.olback.Perif/Icon=\$\{SNAP\}\/usr\/share\/icons\/hicolor\/scalable\/apps\/net.olback.Perif.svg/g' snap/gui/net.olback.Perif.desktop
cp data/icons/hicolor/scalable/apps/net.olback.Perif.svg snap/gui

rm -rf $SNAP_BUILD
mkdir $SNAP_BUILD

# /usr
mkdir -p $SNAP_BUILD/usr/share

# Icon
cp -r data/icons $SNAP_BUILD/usr/share

# Appdata
mkdir $SNAP_BUILD/usr/share/metainfo
cp data/net.olback.Perif.appdata.xml $SNAP_BUILD/usr/share/metainfo

# GSchema
mkdir -p $SNAP_BUILD/usr/share/glib-2.0/schemas
cp data/net.olback.perif.gschema.xml $SNAP_BUILD/usr/share/glib-2.0/schemas

# Binary
mkdir $SNAP_BUILD/usr/bin
cp target/release/perif $SNAP_BUILD/usr/bin

### snapcraft.yaml
cp snap/local/snapcraft.yaml snap/snapcraft.yaml

# Set version
PERIF_VERSION=$(cat Cargo.toml | grep 'version' | head -n 1 | rev | cut -c2-6 | rev)
echo "Version $PERIF_VERSION"
sed -i -e "s/{PERIF_VERSION}/$PERIF_VERSION/g" snap/snapcraft.yaml

