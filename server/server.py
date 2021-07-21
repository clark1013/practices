import tornado.ioloop
import tornado.web
import tornado.options
import requests
import httpx

class MainHandler(tornado.web.RequestHandler):
    async def get(self):
        # resp = requests.get("http://localhost:12345/")
        async with httpx.AsyncClient() as client:
            resp = await client.get("http://localhost:12345/")
        self.write("done")

def make_app():
    return tornado.web.Application([
        (r"/test", MainHandler),
    ])

if __name__ == "__main__":
    tornado.options.define("port", default=10080, help="run on the given port", type=int)
    tornado.options.parse_command_line()
    app = make_app()
    app.listen(tornado.options.options.port)
    tornado.ioloop.IOLoop.current().start()
