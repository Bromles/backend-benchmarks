package main

import (
	"log"

	"github.com/gofiber/fiber/v2"
)

func handle(c *fiber.Ctx) error {
	return c.SendString("Hello, World!")
}

func main() {
	app := fiber.New()

	app.Get("/", handle)

	log.Fatal(app.Listen(":3000"))
}
