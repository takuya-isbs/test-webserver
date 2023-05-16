#!/bin/sh

set -eux

. ./common.sh

NAME=rustweb

cargo install --path .
BINPATH=`which rustweb`
BINDIR=`dirname $BINPATH`

sed -e "s;@BINDIR@;$BINDIR;" \
    -e "s;@ENVFILE@;$ENVFILE;" \
    -e "s;@USERNAME@;$USERNAME;" \
    -e "s;@GROUPNAME@;$GROUPNAME;" \
    -e "s;@DISABLE_USERNAME@;$DISABLE_USERNAME;" \
    -e "s;@DISABLE_GROUPNAME@;$DISABLE_GROUPNAME;" \
    ${NAME}.service.in > ${NAME}.service

install -D -m 644 $ENVFILE_TMPL $ENVFILE
install -D -m 644 ${NAME}.service ${SYSTEMD_SYSTEM_DIR}/${NAME}.service
systemctl enable $SYSTEMD_OPT_USER --now ${NAME}.service

systemctl $SYSTEMD_OPT_USER status ${NAME}.service | cat

curl -s http://localhost:8000/info/1 | jq .
