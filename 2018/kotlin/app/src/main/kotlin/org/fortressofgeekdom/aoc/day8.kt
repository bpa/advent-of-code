package org.fortressofgeekdom.aoc

import org.fortressofgeekdom.utilities.Aoc
import org.fortressofgeekdom.utilities.nextInt
import java.io.BufferedInputStream

@Aoc(day = 8, part = 1)
fun part1(input: BufferedInputStream) = metadataSum(input)

fun metadataSum(input: BufferedInputStream): Int {
    val children = input.nextInt()
    val metadatas = input.nextInt()
    var sum = 0
    for (i in 1..children) {
        sum += metadataSum(input)
    }
    for (i in 1..metadatas) {
        sum += input.nextInt()
    }
    return sum
}

@Aoc(day = 8, part = 2)
fun part2(input: BufferedInputStream) = nodeValue(input)

fun nodeValue(input: BufferedInputStream): Int {
    val children = input.nextInt()
    val metadatas = input.nextInt()
    var sum = 0

    if (children == 0) {
        for (i in 1..metadatas) {
            sum += input.nextInt()
        }
        return sum
    }

    val childValues = (1..children).map { nodeValue(input) }.toList()
    for (i in 1..metadatas) {
        val index = input.nextInt() - 1;
        sum += childValues.getOrNull(index) ?: 0;
    }
    return sum
}

