#!/system/bin/sh
# ColorOSIconsPatch - uninstall.sh

MODDIR="${0%/*}"

MODID="$(grep -m1 '^id=' "$MODDIR/module.prop" | cut -d= -f2)"
[ -z "$MODID" ] && MODID="ColorOSIconsPatch"

PERSIST_DIR="/data/adb/$MODID"

UXICONS_DST="/my_product/media/theme/uxicons"
EMPTY_DST="/data/oplus/uxicons"

if grep -q " $UXICONS_DST " /proc/mounts; then
  umount -l "$UXICONS_DST" 2>/dev/null
fi

if grep -q " $EMPTY_DST " /proc/mounts; then
  umount -l "$EMPTY_DST" 2>/dev/null
fi

rm -rf "$PERSIST_DIR" 2>/dev/null

exit 0
