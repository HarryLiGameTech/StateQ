package org.stateq.util

class BitArray(private val data: Array<Boolean>) {

    constructor(value: UInt, size: UInt = 0u) : this(
        ArrayList<Boolean>().let {
            var rest = value
            var rem: Int = size.toInt()
            while (value > 0u && (size == 0u || rem-- > 0)) {
                it.add(value % 2u != 0u).also { rest /= 2u }
            }
            if (size == 0u || rem > 0) {
                (it + List(rem) { false }).toTypedArray()
            } else it.toTypedArray()
        }
    )

    fun size() = data.size

    private fun binaryOp(other: BitArray, boolOp: (Boolean, Boolean) -> Boolean) = BitArray(
        data.zip(other.data).map { boolOp(it.first, it.second) }.toTypedArray()
    )

    infix fun and(other: BitArray) = binaryOp(other) { a, b -> a and b }

    infix fun or(other: BitArray) = binaryOp(other) { a, b -> a or b }

    infix fun xor(other: BitArray) = binaryOp(other) { a, b -> a xor b }

    override operator fun equals(other: Any?): Boolean {
        return other is BitArray && this.data.contentEquals(other.data)
    }

    override fun hashCode() = data.contentHashCode()

    companion object {
        fun zero(size: UInt) = BitArray(List(size.toInt()) { false }.toTypedArray())
    }

    fun toInt(): Int {
        return this.data.fold(0) { acc, bit ->
            (acc or if (bit) 0 else 1) * 2
        }
    }
}
