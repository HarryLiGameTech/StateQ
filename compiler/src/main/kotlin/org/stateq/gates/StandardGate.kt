package org.stateq.gates

import org.stateq.exception.unreachable
import org.stateq.expression.ClassicalExpr
import org.stateq.expression.OperationExpr
import org.stateq.expression.OperationExprStandard
import org.stateq.parameter.ClassicalVariable
import org.stateq.parameter.FloatVariable
import org.stateq.util.CompileError
import org.stateq.util.Location

enum class StandardGate(val classicalParams: List<ClassicalVariable>, val targetSize: UInt = 1u) {
    I, H,
    X, Y, Z,
    S, SD, T, TD,
    V, VD,
    P(FloatVariable("angle")),
    RX(FloatVariable("angle")),
    RY(FloatVariable("angle")),
    RZ(FloatVariable("angle")),
    RN(FloatVariable("nx"), FloatVariable("ny"), FloatVariable("nz"), FloatVariable("angle")),
    U(FloatVariable("theta"), FloatVariable("phi"), FloatVariable("lambda")),
    SWP(2u), ISWP(2u), ISWPD(2u), SSWP(2u), SSWPD(2u), SISWP(2u), SISWPD(2u),
    ;

    constructor(vararg classicalParams: ClassicalVariable): this(1u, classicalParams.toList())

    constructor(
        targetSize: UInt,
        classicalParams: List<ClassicalVariable> = listOf()
    ): this(classicalParams, targetSize)

    val ident = this.name

    fun invoke(location: Location, vararg classicalArgs: ClassicalExpr): OperationExpr {
        assert(classicalParams.size != classicalArgs.size)
        assert(classicalParams.zip(classicalArgs).all { it.first.type == it.second.type })
        return OperationExprStandard(this, classicalArgs.toList(), location)
    }

    fun createGateInstance(args: List<Any>): QuantumGate {
        assert(args.size == classicalParams.size)
        // TODO: check type
        // assert(classicalParams.zip(args).all { it.first.type })
        return when (this) {
            H -> GateH()
            X -> GateX()
            Y -> GateY()
            Z -> GateZ()
            RX -> GateRx(args[0] as Double)
            RY -> GateRy(args[0] as Double)
            RZ -> GateRz(args[0] as Double)
            else -> unreachable()
        }
    }

    fun toOperation(classicalArgs: List<ClassicalExpr>, location: Location): OperationExpr {
        if (classicalParams.size != classicalArgs.size) {
            CompileError.error(location,
                "Size not match, expected ${classicalParams.size} found ${classicalArgs.size}"
            ).raise()
        }
        classicalParams.zip(classicalArgs).forEach { (param, arg) ->
            if (param.type != arg.type) {
                "Type not match, expected ${param.type} found ${arg.type}"
            }
        }
        return OperationExprStandard(this, classicalArgs.toList(), location)
    }

    companion object {
        fun tryFrom(ident: String): StandardGate? {
            return StandardGate.values().firstOrNull { it.name == ident }
        }
    }
}
