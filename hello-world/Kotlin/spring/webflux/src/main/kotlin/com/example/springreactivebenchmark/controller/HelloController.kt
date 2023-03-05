package com.example.springreactivebenchmark.controller

import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.RestController
import reactor.core.publisher.Mono

@RestController
class HelloController {
    @GetMapping("/")
    fun hello(): Mono<String> {
        return Mono.just("Hello World")
    }
}