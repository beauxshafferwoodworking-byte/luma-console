#!/bin/sh
# Luma's optional cold-boot hook. Installed only when explicitly requested.
APP_DIR=/media/developer/apps/usr/palm/applications/com.beaux.lumaconsole
HOOK=/var/lib/webosbrew/init.d/66-luma-console
case "${1:-}" in
  enable)
    [ "$(id -u)" = 0 ] || { printf '%s\n' 'Run as root.' >&2; exit 1; }
    if [ -e /var/lib/webosbrew/init.d/64-luma-home ] || [ -L /var/lib/webosbrew/init.d/64-luma-home ] ||
       [ -e /var/lib/webosbrew/init.d/65-luma ] || [ -L /var/lib/webosbrew/init.d/65-luma ] ||
       [ -e /var/lib/webosbrew/init.d/65-luma-console-home ] || [ -L /var/lib/webosbrew/init.d/65-luma-console-home ]; then
      printf '%s\n' 'Disable the existing Home or boot integration before enabling Console autostart.' >&2
      exit 1
    fi
    [ -f "$APP_DIR/appinfo.json" ] || { printf '%s\n' 'Install Luma first.' >&2; exit 1; }
    mkdir -p /var/lib/webosbrew/init.d
    if [ -e "$HOOK" ] || [ -L "$HOOK" ]; then
      [ "$(readlink "$HOOK")" = "$APP_DIR/autostart.sh" ] || { printf '%s\n' '66-luma-console is already used by another file.' >&2; exit 1; }
    fi
    chmod 755 "$APP_DIR/autostart.sh"
    ln -sf "$APP_DIR/autostart.sh" "$HOOK"
    printf '%s\n' 'Luma will open after a full boot.'
    exit 0 ;;
  disable)
    [ "$(id -u)" = 0 ] || exit 1
    if [ "$(readlink "$HOOK")" = "$APP_DIR/autostart.sh" ]; then rm -f "$HOOK"; fi
    printf '%s\n' 'Luma boot launch disabled.'
    exit 0 ;;
esac
# A lock lives for this boot only; do not launch twice when the hook is invoked again.
(
    exec 9>/tmp/luma-console-boot.lock
    flock -n 9 || exit 0
    [ -f /tmp/luma-console-boot-done ] && exit 0
    [ -f "$APP_DIR/appinfo.json" ] || exit 0
    sleep 25
    # -n prints the reply. -t is timing mode, not a firmware-specific flag.
    RESULT="$(/usr/bin/luna-send -n 1 -w 5000 luna://com.webos.service.applicationmanager/launch '{"id":"com.beaux.lumaconsole"}' 2>/dev/null)"
    printf '%s\n' "$RESULT"
    case "$RESULT" in *'"returnValue":true'*|*'"returnValue": true'*) touch /tmp/luma-console-boot-done ;; esac
) </dev/null >/tmp/luma-console-boot.log 2>&1 &
