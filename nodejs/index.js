const Koa = require('koa');
const Router = require('@koa/router');
const koaBody = require('koa-body');

const download = require("./download")

const app = new Koa();
const router = new Router();

router.get('/', ctx => {
    ctx.body = "<h1>it works!!"
});

const urls = []
router.post('/m3u8', ctx => {
    // console.log(ctx.request.body)
    const { url } = ctx.request.body
    if (url in urls) {
        ctx.body = { code: 1 }
        return
    }

    urls.push(url)
    ctx.body = { code: 0 }
    download.down(ctx.request.body).catch(console.error)
})

app
    .use(koaBody())
    .use(router.routes())
    .use(router.allowedMethods());

app.listen(2022);