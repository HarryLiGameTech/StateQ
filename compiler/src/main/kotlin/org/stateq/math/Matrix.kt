package org.stateq.math

import org.jetbrains.kotlinx.multik.api.d2arrayIndices
import org.jetbrains.kotlinx.multik.api.mk
import org.jetbrains.kotlinx.multik.ndarray.complex.Complex
import org.jetbrains.kotlinx.multik.ndarray.complex.ComplexDouble
import org.jetbrains.kotlinx.multik.ndarray.data.D1Array
import org.jetbrains.kotlinx.multik.ndarray.data.D2Array
import org.jetbrains.kotlinx.multik.ndarray.operations.map
import org.stateq.exception.unreachable
import kotlin.math.cos
import kotlin.math.sin

typealias Matrix = D2Array<ComplexDouble>
typealias Vector = D1Array<ComplexDouble>
typealias ParamMatrix = D2Array<MatrixParam>

interface MatrixParam

class ComplexLiteral(val re: Double, val im: Double): Complex, MatrixParam {
    val data get() = ComplexDouble(re, im)
}

class ComplexPlaceholder(val name: String): MatrixParam

fun complex(re: Double, im: Double = 0.0) = ComplexDouble(re, im)

fun imaginary(im: Double) = ComplexDouble(0.0, im)

fun ComplexDouble.exp(): ComplexDouble = ComplexDouble(
    re = kotlin.math.exp(this.re) * cos(this.im),
    im = kotlin.math.exp(this.re) * sin(this.im),
)

fun Matrix.adjoin(): Matrix = this.deepCopy().transpose().map { it.conjugate() }

fun Matrix.isUnitary(): Boolean {
    return TODO()
}

fun Matrix.isHermitain(): Boolean = TODO()

fun Vector.toMatrix(): Matrix = TODO()

fun ParamMatrix.instantiate(vararg args: Pair<String, ComplexDouble>) {

}

fun ParamMatrix.instantiate(args: Map<String, ComplexDouble>): Matrix {
    return this.map {
        when (it) {
            is ComplexLiteral -> it.data
            is ComplexPlaceholder -> args[it.name] ?: throw IllegalArgumentException()
            else -> unreachable()
        }
    }
}

object Matrices {

    fun identity(size: UInt): Matrix {
        TODO()
    }

    fun square(vararg elements: ComplexDouble): Matrix {
        if (!elements.size.isPerfectSquare()) {
            throw IllegalArgumentException("Size of elements must be a perfect square")
        } else {
            val size = elements.size.sqrt()
            return mk.d2arrayIndices(size, size) { i, j ->
                elements[i * size + j]
            }
        }
    }
    
}
