import urllib

def application(environ, start_response):
    method = environ.get('REQUEST_METHOD')
    params = ""
    if method == 'GET':
        params = urllib.parse.parse_qsl(environ.get('QUERY_STRING'))
    elif method == 'POST':
        #TODO json
        pass

    start_response('200 OK', [('Content-Type','text/html')])
    return [("Hello world:" + method + ",params=" + str(params)).encode()]
