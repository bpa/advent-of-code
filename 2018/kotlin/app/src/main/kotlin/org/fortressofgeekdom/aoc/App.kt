package org.fortressofgeekdom.aoc

import org.fortressofgeekdom.utilities.solution
import java.io.File
import kotlin.time.ExperimentalTime

@ExperimentalTime
fun main(args: Array<String>) {
//    solution("Part 1") {
//        part1(BufferedInputStream(File("../../input/2018/day8.txt").inputStream()))
//    }
//    solution("Part 2") {
//        part2(BufferedInputStream(File("../../input/2018/day8.txt").inputStream()))
//    }
    lateinit var input: Pair<Int, Int>
    solution("Day 9 Input") {
        input = gameConfig(File("../../input/2018/day9.txt"))
        input
    }
    solution("Part 1") {
        part1(input)
    }
    solution("Part 2") {
        part2(input)
    }
}
