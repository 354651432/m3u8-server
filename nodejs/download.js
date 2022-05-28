const request = require('request')
const fs = require('fs/promises')

const proxy = "http://127.0.0.1:1087"

const m3u8 = require('m3u8-parser')
const { time } = require('console')

async function down({ url, headers = {}, title = "" }) {
    title = title.replace(/\//g, '.')
    console.log(`downloading ${url}`)
    const body = await req(url, headers)

    const parser = new m3u8.Parser();
    parser.push(body)
    parser.end()


    const filename = `downloads/${title}.ts`
    const file = await fs.open(filename, 'w')
    for (arr of trunc(parser.manifest.segments, 6)) {
        const arr1 = []
        for (it of arr) {
            const url1 = url.substr(0, url.lastIndexOf('/')) + '/' + it.uri
            console.log(`begin downloading segments ${url1}`)
            arr1.push(req(url1, headers).catch(err => {
                console.error("retry 1", url1, err)
                setTimeout(() => {
                    req(url1, headers).catch(err => {
                        console.error("retry 2", url1, err)
                        setTimeout(() => {
                            req(url1, headers).catch(console.error)
                        }, 100)
                    })
                }, 100)
            }))
        }

        const arr2 = await Promise.all(arr1)
        console.log('downloading parti complete')
        for (it of arr2) {
            if (!it) {
                console.error("segment error ", it)
                break
            }
            await file.write(it)
        }
    }
    file.close()
    console.log("downloading complete")
}

function req(url, headers) {
    return new Promise((res, rej) => {
        request({
            url,
            headers,
            encoding: null,
            proxy
        }, (error, response, body) => {
            if (error) {
                rej(error)
                return
            }
            res(response.body)
        })
    })
}

function trunc(arr, size) {
    const len = arr.length
    const ret = []
    for (i = 0; i < len / size; i++) {
        ret.push(arr.slice(i, (i + 1) * size))
    }
    return ret
}

module.exports = { down, req }