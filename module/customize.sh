#!/system/bin/sh
# customize.sh
SKIPMOUNT=false
PROPFILE=true
POSTFSDATA=true
LATESTARTSERVICE=true

MODID="$(grep -m1 '^id=' "$MODPATH/module.prop" | cut -d= -f2)"
[ -z "$MODID" ] && MODID="ColorOSIconsPatch"

PERSIST_BASE="/data/adb"
PERSIST_DIR="$PERSIST_BASE/$MODID"
UXICONS_DIR="$PERSIST_DIR/uxicons"
RUNTIME_DIR="$PERSIST_DIR/runtime"
EMPTY_DATA_DIR="$RUNTIME_DIR/empty_data_oplus_uxicons"

MOUNT_TARGET="/my_product/media/theme/uxicons"
MOUNT_EMPTY_TARGET="/data/oplus/uxicons"

ui_print "- ColorOS Icons Patch"
ui_print "- Using persist dir: $PERSIST_DIR"

mkdir -p "$UXICONS_DIR" "$RUNTIME_DIR" "$EMPTY_DATA_DIR" || abort "Create persist dir failed"

chmod 0755 "$PERSIST_DIR" "$UXICONS_DIR" "$RUNTIME_DIR" "$EMPTY_DATA_DIR"
chown 0:0 "$PERSIST_DIR" "$UXICONS_DIR" "$RUNTIME_DIR" "$EMPTY_DATA_DIR" 2>/dev/null

if [ ! -f "$RUNTIME_DIR/settings.json" ]; then
  ui_print "- Init runtime/settings.json"
  cat > "$RUNTIME_DIR/settings.json" <<EOF
{
  "module_id": "ColorOSIconsPatch",
  "channel": "beta",
  "index_url": "https://immortal521.github.io/coloros-icons-patch/beta/index.json",
  "scan_user_only": true,
  "scan_limit": null
}
EOF
  set_perm "$RUNTIME_DIR/settings.json" 0 0 0644
fi

if [ ! -f "$RUNTIME_DIR/state.json" ]; then
  ui_print "- Init runtime/state.json"
  cat > "$RUNTIME_DIR/state.json" <<EOF
{}
EOF
  set_perm "$RUNTIME_DIR/state.json" 0 0 0644
fi

cat > "$MODPATH/paths.conf" <<EOF
MODID=$MODID
PERSIST_DIR=$PERSIST_DIR
UXICONS_DIR=$UXICONS_DIR
RUNTIME_DIR=$RUNTIME_DIR
EMPTY_DATA_DIR=$EMPTY_DATA_DIR
MOUNT_TARGET=$MOUNT_TARGET
MOUNT_EMPTY_TARGET=$MOUNT_EMPTY_TARGET
EOF
set_perm "$MODPATH/paths.conf" 0 0 0644

find "$UXICONS_DIR" -type d -exec chmod 0755 {} \; 2>/dev/null
find "$UXICONS_DIR" -type f -exec chmod 0644 {} \; 2>/dev/null

set_perm_recursive "$MODPATH" 0 0 0755 0644

# 可执行文件 / 脚本恢复 0755（关键修复点）
set_perm "$MODPATH/bin/aapt2"    0 0 0755
set_perm "$MODPATH/bin/uxiconsd" 0 0 0755
set_perm "$MODPATH/post-fs-data.sh" 0 0 0755
set_perm "$MODPATH/service.sh"      0 0 0755
set_perm "$MODPATH/uninstall.sh"    0 0 0755

ui_print "- Done"
