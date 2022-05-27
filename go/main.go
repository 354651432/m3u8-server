package main

import (
	"log"
	"work/utils"

	"github.com/gin-gonic/gin"
)

func main() {

	gin.SetMode(gin.ReleaseMode)
	r := gin.Default()
	r.GET("/", func(ctx *gin.Context) {
		ctx.JSON(200, gin.H{"message": "success"})
	})

	var cache = make(map[string]bool, 0)
	r.POST("/m3u8", func(ctx *gin.Context) {
		var rec utils.Rec
		err := ctx.BindJSON(&rec)
		if err != nil {
			log.Println(err)
			return
		}

		if _, exist := cache[rec.Url]; exist {
			ctx.JSON(200, gin.H{
				"code": 1,
			})
			return
		}
		cache[rec.Url] = true

		go utils.Download(rec)

		ctx.JSON(200, gin.H{
			"code": 0,
		})
	})

	r.Run("127.0.0.1:2022")
}
