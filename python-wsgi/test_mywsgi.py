from webtest import TestApp
import mywsgi

class Test_wsgi_app(object):
    def test_get(self):
        app= TestApp(mywsgi.application)
        resp = app.get('/')
        assert resp.status_code == 200
        assert resp.content_type == 'text/html'
        s = resp.body.decode()
        assert s.startswith('Hello world:')

    def test_post(self):
        app= TestApp(mywsgi.application)
        resp = app.post_json('/',
                        { "id": 123, "flag": True })
        assert resp.status_code == 200
        assert resp.content_type == 'text/html'
        s = resp.body.decode()
        print(s)
        assert s.startswith('Hello world:')
