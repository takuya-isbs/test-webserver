## use pyenv

TODO test in container...

```
$ mkdir ~/.local ~/env
$ git clone https://github.com/yyuu/pyenv.git ~/.local/pyenv
$ cat > ~/env/pyenv.sh
export PYENV_ROOT="${HOME}/.local/pyenv"
export PATH="${PYENV_ROOT}/bin:${PATH}"
eval "$(pyenv init -)"
$ . ~/env/pyenv.sh

$ pyenv install --list
$ V=$(cat .python-version)
$ pyenv install -v $V
$ pyenv local $V
$ python3 --version
```

## test

```
pip install -r requirements.txt
pytest test_mywsgi.py
```
