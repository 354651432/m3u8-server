package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"net/url"
	"os"
	"strings"

	"github.com/gin-gonic/gin"
	"github.com/grafov/m3u8"
)

const proxy = "http://127.0.0.1:1087"

var errLog = log.New(os.Stdout, "error ", log.Ldate|log.Lshortfile|log.Ltime)
var goroutine_limiter = make(chan bool, 10)

func main() {
	r := gin.Default()
	r.GET("/", func(ctx *gin.Context) {
		ctx.JSON(200, gin.H{"message": "success"})
	})

	var cache = make(map[string]bool, 0)
	r.POST("/m3u8", func(ctx *gin.Context) {
		var rec Rec
		err := ctx.BindJSON(&rec)
		if err != nil {
			errLog.Println(err)
			return
		}

		if _, exist := cache[rec.Url]; exist {
			ctx.JSON(200, gin.H{
				"code": 1,
			})
			return
		}
		cache[rec.Url] = true

		go download(rec)

		ctx.JSON(200, gin.H{
			"code": 0,
		})
	})

	r.Run("127.0.0.1:2022")
}

func download(rec Rec) {

	r, err := http.NewRequest("GET", rec.Url, nil)
	if err != nil {
		errLog.Println(err)
		return
	}

	for k, v := range rec.Headers {
		r.Header.Add(k, v)
	}

	proxy_url, _ := url.Parse(proxy)
	client := http.Client{
		Transport: &http.Transport{
			Proxy: http.ProxyURL(proxy_url),
		},
	}
	r2, err := client.Do(r)
	if err != nil {
		errLog.Println(err)
		return
	}

	b, err := ioutil.ReadAll(r2.Body)
	if err != nil {
		log.Println(err)
		return
	}
	p, listType, err := m3u8.Decode(*bytes.NewBuffer(b), true)
	if err != nil {
		errLog.Printf("%v %#v %#v", err, r, r2)
		return
	}

	if listType != m3u8.MEDIA {
		return
	}
	mediapl := p.(*m3u8.MediaPlaylist)
	idx := strings.LastIndex(rec.Url, "/")
	fileName := fmt.Sprintf("downloads/%s.ts", rec.Title)
	tmpFileName := fileName + "_downloading"

	f, err := os.OpenFile(tmpFileName, os.O_CREATE|os.O_RDWR|os.O_TRUNC, 0644)
	if err != nil {
		errLog.Print(err)
		return
	}

	defer f.Close()
	defer os.Rename(tmpFileName, fileName)

	var channels []chan []byte

	var len = len(mediapl.Segments)
	var complete = 0
	for _, v := range mediapl.Segments {
		if v == nil {
			break
		}
		url1, _ := proxy_url.Parse(rec.Url[:idx] + "/" + v.URI)

		chanel := make(chan []byte)
		channels = append(channels, chanel)
		go func(url *url.URL, chanel chan []byte) {
			goroutine_limiter <- true
			// fmt.Printf("downloading: %v\n", url)

			r, err := http.NewRequest("GET", url.String(), nil)
			if err != nil {
				return
			}
			for k, v := range rec.Headers {
				r.Header.Add(k, v)
			}

			res, err := client.Do(r)
			if err != nil {
				errLog.Print(err)
				return
			}

			b2, err := ioutil.ReadAll(res.Body)
			if err != nil {
				errLog.Print(err)
				return
			}

			chanel <- b2
			close(chanel)
			<-goroutine_limiter
			complete++
			fmt.Printf("...%s downloading [%v/%v]\n", rec.shortName(), complete, len)
		}(url1, chanel)

	}

	for _, channel := range channels {
		f.Write(<-channel)
	}
}

type Rec struct {
	Url     string
	Title   string
	Headers map[string]string
}

func (this *Rec) shortName() string {
	return this.Url[len(this.Url)-30:]
}
