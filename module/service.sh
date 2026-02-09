#!/system/bin/sh
MODDIR=${0%/*}
log() { echo "[coloros-uxicons] $*"; }

UXICONS_SRC="$MODDIR/uxicons/hdpi"
UXICONS_DST="/my_product/media/theme/uxicons/hdpi"

DATA_UXICONS="$MODDIR/runtime/empty_data_oplus_uxicons"
DATA_OPLUS="/data/oplus/uxicons"

sleep 5

mkdir -p "$MODDIR/runtime"
mkdir -p "$UXICONS_SRC"
mkdir -p "$DATA_UXICONS"

# bind mount 到 /my_product
umount "$UXICONS_DST" 2>/dev/null
if mount --bind "$UXICONS_SRC" "$UXICONS_DST"; then
  log "bind mounted: $UXICONS_SRC -> $UXICONS_DST"
else
  log "ERROR: failed to bind mount: $UXICONS_SRC -> $UXICONS_DST"
fi

# bind mount 空目录到 /data/oplus/uxicons
umount "$DATA_OPLUS" 2>/dev/null
if mount --bind "$DATA_UXICONS" "$DATA_OPLUS"; then
  log "bind mounted: $DATA_UXICONS -> $DATA_OPLUS"
else
  log "ERROR: failed to bind mount: $DATA_UXICONS -> $DATA_OPLUS"
fi

if [ -x "$MODDIR/bin/uxiconsd" ]; then
  "$MODDIR/bin/uxiconsd" status --state "$MODDIR/runtime/state.json" >/dev/null 2>&1
fi

exit 0
