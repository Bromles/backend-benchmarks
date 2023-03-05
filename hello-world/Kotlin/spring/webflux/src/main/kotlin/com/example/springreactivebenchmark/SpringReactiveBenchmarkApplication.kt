package com.example.springreactivebenchmark

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication

@SpringBootApplication
class SpringReactiveBenchmarkApplication

fun main(args: Array<String>) {
    runApplication<SpringReactiveBenchmarkApplication>(*args)
}
