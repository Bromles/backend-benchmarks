package com.example.springbenchmark

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication

@SpringBootApplication
class SpringBenchmarkApplication

fun main(args: Array<String>) {
    runApplication<SpringBenchmarkApplication>(*args)
}
