package org.stateq.math


infix fun Int.pow(exponent: Int): Int = power(this, exponent.toUInt(), 1, Int::times)

infix fun UInt.pow(exponent: UInt): UInt = power(this, exponent, 1u, UInt::times)

infix fun Double.pow(exponent: UInt): Double = power(this, exponent, 1.0, Double::times)

private tailrec fun <T> power(base: T, exponent: UInt, init: T, multiply: (T, T) -> T): T = when {
    exponent == 0u -> init
    exponent % 2u == 0u -> power(multiply(base, base), exponent / 2u, init, multiply)
    else -> power(multiply(base, base), exponent / 2u, multiply(init, base), multiply)
}

infix fun Double.pow(exponent: Double): Double = Math.pow(this, exponent)

fun UInt.sqrt(): UInt = kotlin.math.sqrt(this.toDouble()).toUInt()

fun UInt.isPowerOfTwo(): Boolean = this != 0u && (this and (this - 1u)) == 0u

fun UInt.isPerfectSquare(): Boolean = this.sqrt() pow 2u == this

fun Int.sqrt(): Int = kotlin.math.sqrt(this.toDouble()).toInt()

fun Int.isPowerOfTwo(): Boolean = this != 0 && (this and (this - 1)) == 0

fun Int.isPerfectSquare(): Boolean = this.sqrt() pow 2 == this

fun Double.sqrt(): Double = kotlin.math.sqrt(this)
