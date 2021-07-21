package main

import (
	"time"

	"github.com/gin-gonic/gin"
)

func main() {
	r := gin.Default()
	r.GET("/", func(c *gin.Context) {
		time.Sleep(time.Millisecond * 500)
		c.JSON(200, gin.H{
			"message": "pong",
		})
	})
	r.Run(":12345")
}
