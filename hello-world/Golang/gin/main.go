package main

import "github.com/gin-gonic/gin"

func handle(c *gin.Context) {
	c.String(200, "Hello World")
}

func main() {
	gin.SetMode(gin.ReleaseMode)
	r := gin.Default()

	r.GET("/", handle)

	r.Run() // listen and serve on 0.0.0.0:8080
}
