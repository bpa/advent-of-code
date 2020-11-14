package org.fortressofgeekdom.aoc
import org.fortressofgeekdom.utilities.Aoc
import org.fortressofgeekdom.utilities.AocGenerator
import java.io.File

@AocGenerator(day=9)
fun gameConfig(input: File): Pair<Int, Int> {
    val input = input.readText().split(" ")
    val players = input[0].toInt()
    val last = input[6].toInt()
    return Pair(players, last)
}

@Aoc(day=9, part=1)
fun part1(input: Pair<Int, Int>)= highScore(input.first, input.second)

@Aoc(day=9, part=2)
fun part2(input: Pair<Int, Int>)= highScore(input.first, input.second * 100)

class Marble(val value: Int) {
    var left = this
    var right = this
}

fun highScore(players: Int, last: Int): Long? {
    val marbles = Array(last + 1, ::Marble)
    var scores = LongArray(players)
    var current = marbles[0]

    for (marble in 1..last) {
        val player = marble % players
        if (marble % 23 == 0) {
            scores[player] += marble.toLong()
            for(_i in 1..7) {
                current = current.left
            }
            scores[player] += current.value.toLong()
            current.left.right = current.right
            current.right.left = current.left
            current = current.right
        } else {
            current = current.right
            val newCurrent = marbles[marble]
            newCurrent.right = current.right
            current.right.left = newCurrent
            current.right = newCurrent
            newCurrent.left = current
            current = newCurrent
        }
    }
    return scores.max()
}