package org.stateq.polynomial

abstract class IndeterminateBase {
    abstract val name: String
    fun pow(exponent: UInt) = IntMonomial(this, exponent)
    operator fun times(coefficient: Int) = this.pow(1u) * coefficient
    override fun toString() = name
}

open class Indeterminate(override val name: String) : IndeterminateBase() {

    constructor() : this("")

//    override fun hashCode() = name.hashCode()

//    override fun equals(other: Any?): Boolean {
//        return other is Indeterminate && this.name == other.name
//    }
}
