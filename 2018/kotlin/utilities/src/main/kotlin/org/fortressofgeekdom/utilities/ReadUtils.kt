package org.fortressofgeekdom.utilities

import java.io.BufferedInputStream

val ZERO = '0'.toInt()
val NINE = '9'.toInt()

/**
 * Extension function to grab next int from a BufferedInputStream
 */
fun BufferedInputStream.nextInt(): Int {
    var value = 0
    var c = this.read()
    while (c >= 0 && c >= ZERO && c <= NINE) {
        value = value * 10 + c - ZERO
        c = this.read()
    }
    return value
}