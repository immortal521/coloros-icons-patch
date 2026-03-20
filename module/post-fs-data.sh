#!/system/bin/sh
# ColorOSIconsPatch - post-fs-data.sh

MODDIR=${0%/*}

mount --bind "$MODDIR/my_product/media/theme/uxicons" "/my_product/media/theme/uxicons"
mount --bind "$MODDIR/data/oplus/uxicons" "/data/oplus/uxicons"
