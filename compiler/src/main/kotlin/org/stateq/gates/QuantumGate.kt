package org.stateq.gates

import org.jetbrains.kotlinx.multik.ndarray.operations.map
import org.jetbrains.kotlinx.multik.ndarray.operations.toList
import org.stateq.math.*
import kotlin.math.cos
import kotlin.math.sin

interface QuantumGate {
    val ident: String
    val matrix: Matrix
    val args: List<Any> get() = listOf()
}

class UnitaryGate(val size: UInt, override val matrix: Matrix): QuantumGate {
    override val ident: String = "UNITARY$size"
    override val args: List<Any> get() = matrix.toList()
}

class GateX: QuantumGate {

    override val ident: String = "X"

    override val matrix: Matrix = Matrices.square(
        complex(0.0), complex(1.0),
        complex(1.0), complex(0.0),
    )
}

class GateY: QuantumGate {

    override val ident: String = "Y"

    override val matrix: Matrix = Matrices.square(
        complex(0.0), complex(0.0, -1.0),
        complex(0.0, 1.0), complex(0.0),
    )
}

class GateZ: QuantumGate {

    override val ident: String = "Z"

    override val matrix: Matrix = Matrices.square(
        complex(1.0), complex(0.0),
        complex(0.0), complex(-1.0),
    )
}

class GateH: QuantumGate {

    override val ident: String = "H"

    override val matrix: Matrix = Matrices.square(
        complex(1.0), complex(1.0),
        complex(1.0), complex(-1.0),
    ).map { it / 2.0.sqrt() }
}

class GateRx(val theta: Double): QuantumGate {

    override val ident: String = "RX"

    override val matrix: Matrix = Matrices.square(
        complex(cos(theta / 2.0)).exp(), complex(-sin(theta / 2.0)).exp(),
        imaginary(-sin(theta / 2.0)).exp(), complex(cos(theta / 2.0)).exp(),
    )

    override val args: List<Any> get() = listOf(theta)
}

class GateRy(val theta: Double): QuantumGate {

    override val ident: String = "RY"

    override val matrix: Matrix = Matrices.square(
        complex(cos(theta / 2.0)).exp(), -complex(sin(theta / 2.0)).exp(),
        complex(sin(theta / 2.0)).exp(), complex(cos(theta / 2.0)).exp(),
    )

    override val args: List<Any> get() = listOf(theta)
}

class GateRz(val theta: Double): QuantumGate {

    override val ident: String = "RZ"

    override val matrix: Matrix = Matrices.square(
        imaginary(theta / 2).exp(), complex(0.0),
        complex(0.0), imaginary(theta / 2).exp(),
    )

    override val args: List<Any> get() = listOf(theta)
}

class GatePhase(val theta: Double): QuantumGate {

    override val ident: String = "PH"

    override val matrix: Matrix = Matrices.square(
        imaginary(theta).exp(), complex(0.0),
        complex(0.0), imaginary(theta).exp(),
    )

    override val args: List<Any> get() = listOf(theta)
}

class GatePhaseShift(val theta: Double): QuantumGate {

    override val ident: String = "P"

    override val matrix: Matrix = Matrices.square(
        complex(1.0), complex(0.0),
        complex(0.0), imaginary(theta).exp(),
    )

    override val args: List<Any> get() = listOf(theta)
}

class GateS: QuantumGate {

    override val ident: String = "S"

    override val matrix: Matrix = Matrices.square(
        complex(1.0), complex(0.0),
        complex(0.0), imaginary(1.0 / 2.0).exp(),
    )
}

class GateT: QuantumGate {

    override val ident: String = "T"

    override val matrix: Matrix = Matrices.square(
        complex(1.0), complex(0.0),
        complex(0.0), imaginary(1.0 / 4.0).exp(),
    )
}

class GateSDagger: QuantumGate {

    override val ident: String = "SD"

    override val matrix: Matrix = Matrices.square(
        complex(1.0), complex(0.0),
        complex(0.0), imaginary(-1.0 / 2.0).exp(),
    )
}

class GateTDagger: QuantumGate {

    override val ident: String = "TD"

    override val matrix: Matrix = Matrices.square(
        complex(1.0), complex(0.0),
        complex(0.0), imaginary(-1.0 / 4.0).exp(),
    )
}

class GateU3(val theta: Double, val phi: Double, val lambda: Double): QuantumGate {

    override val ident: String = "U3"

    override val matrix: Matrix = Matrices.square(
        complex(cos(theta / 2.0)), -imaginary(lambda).exp() * sin(theta / 2.0),
        imaginary(phi).exp() * sin(theta / 2.0), imaginary(phi + lambda).exp() * cos(theta / 2.0),
    )

    override val args: List<Any> get() = listOf(theta, phi, lambda)
}

class GateCX: QuantumGate {

    override val ident: String = "CX"

    override val matrix: Matrix = Matrices.square(
        complex(1.0), complex(0.0), complex(0.0), complex(0.0),
        complex(0.0), complex(1.0), complex(0.0), complex(0.0),
        complex(0.0), complex(0.0), complex(0.0), complex(1.0),
        complex(0.0), complex(0.0), complex(1.0), complex(0.0),
    )
}

class GateCP(val theta: Double): QuantumGate {

    override val ident: String = "CP"

    override val matrix: Matrix = Matrices.square(
        complex(1.0), complex(0.0), complex(0.0), complex(0.0),
        complex(0.0), complex(1.0), complex(0.0), complex(0.0),
        complex(0.0), complex(0.0), complex(1.0), complex(0.0),
        complex(0.0), complex(0.0), complex(0.0), imaginary(theta).exp(),
    )

    override val args: List<Any> get() = listOf(theta)
}
