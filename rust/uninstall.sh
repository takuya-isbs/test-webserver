#!/bin/sh

set -eux

. ./common.sh

cargo uninstall --package $NAME
systemctl stop $SYSTEMD_OPT_USER ${NAME}.service
systemctl disable $SYSTEMD_OPT_USER ${NAME}.service
rm -f ${SYSTEMD_SYSTEM_DIR}/${NAME}.service ${ENVFILE}
