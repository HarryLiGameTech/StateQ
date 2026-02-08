package org.stateq.util

infix fun Int.pow(exponent: Int): Int = power(this, exponent)

private tailrec fun power(base: Int, exponent: Int, result: Int = 1): Int = when {
    exponent == 0 -> result
    exponent % 2 == 0 -> power(base * base, exponent / 2, result)
    else -> power(base * base, exponent / 2, result * base)
}
