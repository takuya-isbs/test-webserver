#!/bin/sh

set -eux

. ./common.sh

cargo uninstall --package $NAME
systemctl disable $SYSTEMD_OPT_USER ${NAME}.service
rm -f ${SYSTEMD_SYSTEM_DIR}/${NAME}.service ${ENVFILE}
