#!/bin/sh

set -eux

. ./common.sh

if [ $uid -eq 0 ]; then
    cargo uninstall --root /usr/local --package $NAME
else
    cargo uninstall --package $NAME
fi
systemctl stop $SYSTEMD_OPT_USER ${NAME}.service
systemctl disable $SYSTEMD_OPT_USER ${NAME}.service
rm -f ${SYSTEMD_SYSTEM_DIR}/${NAME}.service ${ENVFILE}
