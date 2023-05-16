NAME=rustweb
ENVFILE_TMPL=rustweb.env

uid=$(id -u)
if [ $uid -eq 0 ]; then
    SYSTEMD_OPT_USER=
    SYSTEMD_SYSTEM_DIR=/etc/systemd/system
    ENVFILE=/etc/sysconfig/${NAME}
    USERNAME=root   # TODO USERNAME=_rustweb install.sh
    GROUPNAME=root  # TODO GROUPNAME=_rustweb install.sh
    DISABLE_USERNAME=
    DISABLE_GROUPNAME=
else
    SYSTEMD_OPT_USER=--user
    SYSTEMD_SYSTEM_DIR=${HOME}/.config/systemd/user/
    ENVFILE=${SYSTEMD_SYSTEM_DIR}/${NAME}
    USERNAME=$(id -un)
    GROUPNAME=$(id -gn)
    DISABLE_USERNAME='# '
    DISABLE_GROUPNAME='# '
fi
