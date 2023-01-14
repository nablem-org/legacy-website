package main

import (
	"crypto/tls"
	"log"

	"github.com/gofiber/fiber/v2"
)

func main() {
	app := fiber.New()

	app.Static("/", "../html")

	app.Static("/css", "../css")
	app.Static("/fonts", "../fonts")
	app.Static("/images", "../images")
	app.Static("/resources", "../resources")
	app.Static("/js", "../js")

	if false {
		basePath := "/etc/letsencrypt/live/isostudios.org/"
		flgCert := basePath + "fullchain.pem"
		flgKey := basePath + "privkey.pem"
		cert, err := tls.LoadX509KeyPair(flgCert, flgKey)

		if err != nil {
			log.Fatal(err)
		}

		tlsConfig := &tls.Config{
			Certificates:             []tls.Certificate{cert},
			// ALlows to publicly expose Go safely by keeping TLS up-to-date; see [4]
			CipherSuites:             nil,
			PreferServerCipherSuites: true,
			MinVersion:               tls.VersionTLS13,
			CurvePreferences: []tls.CurveID{
				// As per Mozilla recommendation [4], explicitly defines ECC cryptography, which allows for smaller key sizes [7]
				tls.CurveP256,
				tls.X25519,
			},
			NextProtos: []string{
				// Due to fasthttp limitation to HTTP v1 only, explicitly defines only to use v1; see [5]
				"http/1.1", "acme-tls/1",
			},
		}

		// Runs the server under the HTTPS port
		ln, err := tls.Listen("tcp", ":443", tlsConfig)
		if err != nil {
			panic(err)
		}

		log.Fatal(app.Listener(ln))
	} else {
		log.Fatal(app.Listen(":8000"))
	}
}
