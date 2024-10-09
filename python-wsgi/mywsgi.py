import urllib
import json
import base64


def application(environ, start_response):
    method = environ['REQUEST_METHOD']
    path = environ.get("PATH_INFO", "")
    authz_str = environ.get("HTTP_AUTHORIZATION", "")
    user = None
    passwd = None
    authz_type = None
    if authz_str:
        authz = authz_str.split()
        if len(authz) >= 2:
            authz_type = authz[0]
            authz_token = authz[1]
            if authz_type == 'Basic':
                b = base64.b64decode(authz_token.encode())
                user_pass_str = b.decode()
                user_pass = user_pass_str.split(":")
                if len(user_pass) >= 2:
                    user = user_pass[0]
                    passwd = user_pass[1]
            elif authz_type == 'Bearer':
                passwd = authz_token

    params = ""
    if method == 'GET':
        params = urllib.parse.parse_qs(environ.get('QUERY_STRING'))
    elif method == 'POST':
        wsgi_input = environ["wsgi.input"]
        content_length = int(environ["CONTENT_LENGTH"])
        params = json.loads(wsgi_input.read(content_length))

    start_response('200 OK', [('Content-Type', 'text/html')])
    env_all = str(environ)
    return [("path=" + path + ",METHOD="
             + method + ",params=" + str(params)
             + ",env=" + env_all
             + ",authz_type=" + str(authz_type)
             + ",user=" + str(user) + ",password=" + str(passwd)
             + "\n")
            .encode()]
    # return [("path=" + path + ",METHOD="
    #          + method + ",params=" + str(params) + "\n")
    #         .encode()]
