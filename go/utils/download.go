package utils

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"net/url"
	"os"
	"strings"

	"github.com/grafov/m3u8"
)

const proxy = "socks5://127.0.0.1:10808"
const goroutine_number = 10
const retry_number = 10

var errLog = log.New(os.Stdout, "error ", log.Ldate|log.Lshortfile|log.Ltime)

func Download(rec Rec) {

	fmt.Printf("begin downloading %v\n", rec.Title)

	r, err := http.NewRequest("GET", rec.Url, nil)
	if err != nil {
		errLog.Println(err)
		return
	}

	for k, v := range rec.Headers {
		r.Header.Add(k, v)
	}

	proxy_url, _ := url.Parse(proxy)
	client := &http.Client{
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

	var len1 = mediapl.Count()
	var complete = 0
	var goroutine_limiter = make(chan bool, goroutine_number)
	for _, v := range mediapl.Segments {
		if v == nil {
			break
		}
		url1, _ := proxy_url.Parse(rec.Url[:idx] + "/" + v.URI)

		chanel := make(chan []byte, 1)
		channels = append(channels, chanel)

		goroutine_limiter <- true
		go func(url *url.URL, chanel chan []byte) {
			var url1 = url.String()
			fmt.Printf("downloading: %v\n", url1[len(url1)-30:])

			r, err := http.NewRequest("GET", url1, nil)
			if err != nil {
				return
			}

			for k, v := range rec.Headers {
				r.Header.Add(k, v)
			}

			client := &http.Client{
				Transport: &http.Transport{
					Proxy: http.ProxyURL(proxy_url),
				},
			}

			var res *http.Response
			for i := 0; i < retry_number; i++ {
				res, err = client.Do(r)
				if err == nil {
					break
				}
			}
			if res == nil {
				log.Println("retry failed")
				return
			}

			// return
			b2, err := ioutil.ReadAll(res.Body)
			if err != nil {
				errLog.Print(err)
				return
			}

			chanel <- b2
			close(chanel)
			<-goroutine_limiter
			complete++
			fmt.Printf("...%s complete [%v/%v]\n", rec.shortName(), complete, len1)
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
