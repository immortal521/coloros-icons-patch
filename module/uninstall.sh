#!/system/bin/sh
MODDIR=${0%/*}

UXICONS_DST="/my_product/media/theme/uxicons/hdpi"

umount "$UXICONS_DST" 2>/dev/null
rm -rf "$MODDIR/runtime" 2>/dev/null

exit 0
