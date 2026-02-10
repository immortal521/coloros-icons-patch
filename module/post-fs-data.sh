#!/system/bin/sh
# ColorOSIconsPatch - post-fs-data.sh

MODDIR=${0%/*}

CONF="$MODDIR/paths.conf"

MODID="ColorOSIconsPatch"
PERSIST_DIR="/data/adb/$MODID"
UXICONS_DIR="$PERSIST_DIR/uxicons"
RUNTIME_DIR="$PERSIST_DIR/runtime"
EMPTY_DATA_DIR="$RUNTIME_DIR/empty_data_oplus_uxicons"
MOUNT_TARGET="/my_product/media/theme/uxicons"
MOUNT_EMPTY_TARGET="/data/oplus/uxicons"

if [ -f "$CONF" ]; then
  # shellcheck disable=SC1090
  . "$CONF"
fi

LOG="/data/adb/${MODID}/runtime/mount.log"
mkdir -p "${LOG%/*}" 2>/dev/null

mkdir -p "$UXICONS_DIR" 2>/dev/null
chmod 0755 "$UXICONS_DIR" 2>/dev/null

mount --bind "$UXICONS_DIR" "$MOUNT_TARGET" 2>/dev/null
mount --bind "$EMPTY_DATA_DIR" "$MOUNT_EMPTY_TARGET" 2>/dev/null
