#!/system/bin/sh
MODDIR=${0%/*}

mkdir -p "$MODDIR/runtime"
mkdir -p "$MODDIR/uxicons/hdpi"
mkdir -p "$MODDIR/bin"

chmod 0755 "$MODDIR/bin/uxiconsd" 2>/dev/null
chmod 0755 "$MODDIR/bin/aapt2" 2>/dev/null

exit 0
