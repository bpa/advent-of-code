package org.fortressofgeekdom.utilities

@Target(AnnotationTarget.FUNCTION)
@Retention(AnnotationRetention.SOURCE)
annotation class Aoc(val day: Int, val part: Int, val alt: Int = 0)

@Target(AnnotationTarget.FUNCTION)
@Retention(AnnotationRetention.SOURCE)
annotation class AocGenerator(val day: Int)