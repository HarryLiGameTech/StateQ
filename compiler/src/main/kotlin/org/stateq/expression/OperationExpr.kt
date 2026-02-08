package org.stateq.expression

import org.stateq.gates.StandardGate
import org.stateq.module.quantum.OperationDef
import org.stateq.util.*
import kotlin.reflect.KClass

inline val <reified K: ClassicalExpr> KClass<K>.typename: String get() {
    return this.simpleName!!.dropLast("Expr".length)
}

interface QuantumExpr

abstract class OperationExpr : Locatable, Sized {
    abstract val isSizeDetermined: Boolean
}

abstract class OperationExprElementary : OperationExpr() {
    abstract val ident: String
    abstract val classicalArgs: List<ClassicalExpr>
}

class OperationExprStandard(
    val gate: StandardGate,
    override val classicalArgs: List<ClassicalExpr>,
    override val location: Location,
) : OperationExprElementary() {
    override val isSizeDetermined: Boolean = true
    override val ident: String get() = gate.name
    override val size: IntExpr by lazy { IntExpr(gate.targetSize.toInt()) }
}

// TODO: class OperationExprGate

class OperationExprUserDefined(
    val definition: OperationDef,
    override val classicalArgs: List<ClassicalExpr>,
    override val location: Location,
) : OperationExprElementary() {

    init {
        raiseCompileErrorIf(definition.quantumParams.size != 1, location) {
            "Operation ${definition.ident} requires more than one quantum parameters"
        }

        raiseCompileErrorIf(this.classicalArgs.size != definition.classicalParams.size, location) {
            "Operation ${definition.ident} requires ${definition.classicalParams.size} classical parameters, " +
            "got ${this.classicalArgs.size}"
        }
    }

    override val ident: String get() = definition.ident
    override val isSizeDetermined: Boolean = definition.quantumParams[0].sizeInferenceVariable == null
    override val size: IntExpr get() = IntExpr(definition.quantumParams[0].size)
}

// `listOf(U1, U2, U3)` means U3 matmul U2 matmul U1
class OperationExprSequentialMatMul(
    val sequence: List<OperationExpr>, override val location: Location
) : OperationExpr() {

    override val isSizeDetermined: Boolean by lazy {
        sequence.any { it.isSizeDetermined }
    }

    override val size: IntExpr by lazy {
        sequence.firstOrNull { it.isSizeDetermined }?.size ?: sequence[0].size
    }
}

// `U+`: $ U^\dagger $, conjugate transpose
class OperationExprDagger(
    val operation: OperationExpr, override val location: Location
) : OperationExpr() {
    override val isSizeDetermined: Boolean by lazy { operation.isSizeDetermined }
    override val size: IntExpr by lazy { operation.size }
}

// `U1.U2`: $ U_1 \otimes U_2 $, matrix tensor product
class OperationExprCombined(
    val operations: List<OperationExpr>,
    override val location: Location
) : OperationExpr() {

    override val isSizeDetermined: Boolean by lazy {
        operations.all { it.isSizeDetermined }
    }

    override val size: IntExpr by lazy {
        operations.fold(IntExpr(0)) { acc, opExpr -> acc + opExpr.size }
    }
}

// `U@n`: $ U^{\otimes n} $, matrix tensor product power
class OperationExprExtended(
    val operation: OperationExpr, val multiplier: IntExpr,
    override val location: Location
) : OperationExpr() {
    override val isSizeDetermined: Boolean by lazy { operation.isSizeDetermined }
    override val size: IntExpr by lazy { multiplier * operation.size }
    val innerSize: IntExpr get() = operation.size
}

// `a*U`: $ aU $, coefficient multiply.
//  e.g. global phase shifting
class OperationExprCoefficient(
    val operation: OperationExpr, val coefficient: ComplexExpr,
    override val location: Location
) : OperationExpr() {
    override val isSizeDetermined: Boolean by lazy { operation.isSizeDetermined }
    override val size: IntExpr by lazy { operation.size }
}

// qif-apply expression (multi-controlled gate)
//  e.g. `qif &ctrl apply U1 else U2 $phi`
class OperationExprQifApply(
    val ctrl: QrefExpr,
    val ifBranch: OperationExpr,
    val elseBranch: OperationExpr,
    override val location: Location,
) : OperationExpr() {
    override val size: IntExpr by lazy {
        ifBranch.size.also {
            raiseCompileErrorIf(it != elseBranch.size, location) {
                "The size of operation of the `if` branch differs from that of the `else` branch." +
                "\t`if` branch operation size = ${ifBranch.size}, `else` branch operation size = ${elseBranch.size}"
            }
        }
    }
    override val isSizeDetermined: Boolean by lazy {
        ifBranch.isSizeDetermined.also {
            raiseCompileErrorIf(it != elseBranch.isSizeDetermined, location) {
                "The size of operation of the `if` branch differs from that of the `else` branch." +
                "\t`if` branch operation size is determined = ${ifBranch.isSizeDetermined}, " +
                "`else` branch operation size is determined = ${elseBranch.isSizeDetermined}"
            }
        }
    }
}

