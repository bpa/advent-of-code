package org.fortressofgeekdom.utilities

import java.util.concurrent.TimeUnit
import kotlin.time.ExperimentalTime
import kotlin.time.toDuration

/**
 * Time a block
 */
@ExperimentalTime
inline fun <T> solution(name: String, block: () -> T) {
    val start = System.nanoTime()
    val result = block()
    val end = System.nanoTime()
    println("$name: $result\nTime: ${(end - start).toDuration(TimeUnit.NANOSECONDS)}\n")
}