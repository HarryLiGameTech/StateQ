package org.stateq.evaluator

import org.stateq.circuit.CircuitTranspiler
import org.stateq.circuit.Qubits
import org.stateq.datastructure.ScopedIdentMap
import org.stateq.exception.unreachable
import org.stateq.expression.*
import org.stateq.intermediate.DecomposedInstruction
import org.stateq.intermediate.decomposed.*
import org.stateq.math.pow
import org.stateq.module.classical.ConstantDef
import org.stateq.module.quantum.OperationDef
import org.stateq.module.quantum.ProgramDef
import org.stateq.parameter.ClassicalVariable
import org.stateq.polynomial.*
import org.stateq.qubit.*
import org.stateq.type.ClassicalBasicType
import org.stateq.type.ClassicalTrait
import org.stateq.util.cast
import org.stateq.util.slice
import java.util.function.Function

class IntermediateEvaluator(val module: org.stateq.module.Module) {

    // TODO: isolation stack frames
    private val classicalVariables = ScopedIdentMap<RuntimeVariable<Any>>()
    private val stateVariables = ScopedIdentMap<RuntimeQubitsVariable>()
    private val operations = ScopedIdentMap<OperationDef>()
    private val circuitTranspiler = CircuitTranspiler()
    private val mainProgram by lazy {
        module.definitions.filterIsInstance<ProgramDef>().firstOrNull { it.ident == "Main" }?.also {
            if (it.params.isNotEmpty()) {
                throw RuntimeException("Main program cannot have parameters")
            }
        } ?: run {
            throw RuntimeException("Main program not found")
        }
    }

    init {
        classicalVariables.enterScope()
        stateVariables.enterScope()
        operations.enterScope()

        module.definitions.forEach { definition ->
            when (definition) {
                is OperationDef -> operations[definition.ident] = definition
                is ConstantDef -> classicalVariables[definition.variable.ident] = definition.value.eval()
                else -> unreachable()
            }
        }
    }

    fun evaluate() {
        mainProgram.body.transpile().decompose().eval()
    }

    private inner class RuntimeVariable<out T: Any>(
        val ident: String,
        val value: T,
    ) {
        inline fun <reified R: Any> getVal(): R {
            if (value is R) {
                return value
            } else {
                throw RuntimeException("Type mismatch: ${value::class} is not ${R::class}")
            }
        }
    }

    private inner class RuntimeQubitsVariable(
        val ident: String,
        val qubits: Qubits,
    ) {
        fun addQubit(qubit: UInt) {
            if (qubit !in qubits) {
                qubits + qubit
            } else {
                throw RuntimeException("Qubit $qubit already exists")
            }
        }

        operator fun plusAssign(qubit: UInt) {
            addQubit(qubit)
        }

        operator fun plusAssign(qubits: Qubits) {
            qubits.forEach { addQubit(it) }
        }
    }

    private inline fun <reified T : Any> ClassicalVariable.assign(value: T): RuntimeVariable<T> {
        assert(this.type == ClassicalBasicType.from<T>())
        // TODO: Check if variable is some specific classical type
        return RuntimeVariable(this.ident, value).also { variable ->
            if (classicalVariables.put(this.ident, variable) != variable) {
                throw RuntimeException("Variable ${this.ident} already exists")
            }
        }
    }

    private inline fun <reified T : Any> ClassicalVariable.shadow(value: T): RuntimeVariable<T> {
        assert(this.type == ClassicalBasicType.from<T>())
        return RuntimeVariable(this.ident, value).also { classicalVariables[this.ident] = it }
    }

    private inline fun <reified T: Any> getVariableValue(ident: String): T {
        return classicalVariables[ident]?.getVal<T>() ?: run {
            throw RuntimeException("Variable $ident not found")
        }
    }

    private fun <T> withInScope(action: () -> T) {
        classicalVariables.enterScope()
        stateVariables.enterScope()
        action()
        classicalVariables.exitScope()
        stateVariables.exitScope()
    }

    private fun <T> withInIsolatedScope(action: () -> T) {
        classicalVariables.enterScopeWithBarrier()
        stateVariables.enterScopeWithBarrier()
        action()
        classicalVariables.exitScope()
        stateVariables.exitScope()
    }

    private fun BeginDaggerInstruction.eval() = circuitTranspiler.beginDagger()

    private fun EndDaggerInstruction.eval() = circuitTranspiler.beginDagger()

    private fun BeginControl.eval() = circuitTranspiler.beginControl(
        this.ctrlQubits.eval().toSet(), this.condition
    )

    private fun EndControl.eval() = circuitTranspiler.endCtrl(
        this.ctrlQubits.eval().toSet()
    )

    inline fun <reified T> ClassicalExpr.eval(): T {
        return when (this) {
            is BoolExpr     ->  this.eval()
            is IntExpr      ->  this.eval()
            is FloatExpr    ->  this.eval()
            // is ComplexExpr  ->  this.eval()
            else -> unreachable()
        }.cast<T>() ?: run {
            throw RuntimeException("Type mismatch: $this is not ${T::class}")
        }
    }

    fun BoolExpr.eval(): Boolean {
        return when (this) {
            is BoolExprLiteralTrue -> true
            is BoolExprLiteralFalse -> false
            is BoolExprVariable -> getVariableValue<Boolean>(this.variable.ident)
            is BoolExprNot -> this.inner.eval().not()
            is BoolExprBinary -> when (this.op) {
                BoolExprBinary.Operator.And -> this.lhs.eval() && this.rhs.eval()
                BoolExprBinary.Operator.Or  -> this.lhs.eval() || this.rhs.eval()
            }
            is BoolExprCompare<*> -> {
                when (this.op) {
                    CompareOperator.Greater       ->  this.lhs > this.rhs
                    CompareOperator.Less          ->  this.lhs < this.rhs
                    CompareOperator.GreaterEqual  ->  this.lhs >= this.rhs
                    CompareOperator.LessEqual     ->  this.lhs <= this.rhs
                    CompareOperator.Equal         ->  this.lhs == this.rhs
                    CompareOperator.NotEqual      ->  this.lhs != this.rhs
                }
            }
            else -> unreachable()
        }
    }

    fun IntExpr.eval(): Int {
        return this.terms.fold(0) { sum, term ->
            sum + term.coefficient * term.terms.entries.fold(1) {
                product, (term, exponent) -> product * (when (term) {
                    is IndeterminateLikeDivision            ->  term.lhs.eval() / term.rhs.eval()
                    is IndeterminateLikePower               ->  term.lhs.eval() pow term.rhs.eval()
                    is IndeterminateLikeAnd                 ->  term.lhs.eval() and term.rhs.eval()
                    is IndeterminateLikeOr                  ->  term.lhs.eval() or term.rhs.eval()
                    is IndeterminateLikeXor                 ->  term.lhs.eval() xor term.rhs.eval()
                    is IndeterminateLikeModulo              ->  term.lhs.eval() % term.rhs.eval()
                    is IndeterminateLikeShiftLeft           ->  term.lhs.eval() shl term.rhs.eval()
                    is IndeterminateLikeShiftRight          ->  term.lhs.eval() shr term.rhs.eval()
                    is IndeterminateLikeLogicalShiftRight   ->  term.lhs.eval() ushr term.rhs.eval()
                    is IndeterminateLikeFuncCall -> {
                        TODO()
                    }
                    else -> unreachable()
                } pow exponent.toInt())
            }
        }
    }

    fun FloatExpr.eval(): Double {
        return when (this) {
            is FloatExprLiteral -> value
            is FloatExprVariable -> getVariableValue<Double>(this.variable.ident)
            is FloatExprNegative -> -this.eval()
            is FloatExprBinary -> when (this.op) {
                FloatExprBinary.Operator.Add -> this.lhs.eval() + this.rhs.eval()
                FloatExprBinary.Operator.Sub -> this.lhs.eval() - this.rhs.eval()
                FloatExprBinary.Operator.Div -> this.lhs.eval() / this.rhs.eval()
                FloatExprBinary.Operator.Mul -> this.lhs.eval() * this.rhs.eval()
                FloatExprBinary.Operator.Pow -> this.lhs.eval() pow this.rhs.eval()
            }
            is FloatExprFromInt -> this.inner.eval().toDouble()
            is FloatExprFuncCall -> TODO()
            else -> unreachable()
        }
    }

    private operator fun <T: ClassicalTrait> BoolComparableExpr<T>.compareTo(
        other: BoolComparableExpr<ClassicalTrait>
    ): Int {
        return when (this) {
            is IntExpr -> when (other) {
                is IntExpr -> this.eval().compareTo(other.eval())
                is FloatExpr -> this.eval().compareTo(other.eval())
                else -> null
            }
            is FloatExpr -> when (other) {
                is IntExpr -> this.eval().compareTo(other.eval())
                is FloatExpr -> this.eval().compareTo(other.eval())
                else -> null
            }
            else -> null
        } ?: run {
            throw RuntimeException("Type mismatch: $this is not comparable with $other")
        }
    }

    private fun DecomposedBasicBlock.eval() {
        this.instructions.forEach { it.eval() }
    }

    private fun DecomposedInstruction.eval() {
        when (this) {
            is DecomposedForLoop<ClassicalTrait> -> when (val type = this.iterator.type) {
                is ClassicalBasicType -> {
                    when (type) {
                        ClassicalBasicType.Int -> this.eval<Int>()
                        ClassicalBasicType.Float -> this.eval<Double>()
                        else -> TODO()
                    }
                }
                else -> TODO()
            }
            is DecomposedIfStatement -> this.eval()
        }
    }

    private fun DecomposedIfStatement.eval() {
        if (this.condition.eval()) {
            ifBranch.eval()
        } else {
            elseBranch?.eval()
        }
    }

    private inline fun <reified T : Any> DecomposedForLoop<ClassicalTrait>.eval() {
        assert(this.iterator.type == ClassicalBasicType.from<T>())
        val iterable = this.iterableExpr.eval<T>()
        for (value in iterable) {
            classicalVariables.withInScope {
                this.iterator.assign(value)
                loopBody.eval()
            }
        }
    }

    private inline fun <reified T : Any> IterableExpr<ClassicalTrait>.eval(): Iterable<T> {
        assert(this.iterableType == ClassicalBasicType.from<T>())
        return when (this) {
            is ListExpr<*> -> this.eval()
            is BitsExpr -> TODO()
            else -> unreachable()
        }
    }

    private inline fun <reified T> ListExpr<ClassicalTrait>.eval(): List<T> {
        return object : Function<ListExpr<ClassicalTrait>, List<T>> {
            override fun apply(listExpr: ListExpr<ClassicalTrait>): List<T> {
                return when (listExpr) {
                    is ListExprLiteral -> listExpr.elements.map { it.eval<T>() }
                    is ListExprEmpty -> emptyList()
                    is ListExprSlicing -> apply(listExpr.inner).let { list ->
                        list.slice(
                            listExpr.start.eval(),
                            listExpr.end?.eval()?.plus(if (listExpr.inclusive) 1 else 0) ?: list.size,
                            listExpr.step.eval().toUInt(),
                            listExpr.inclusive,
                        )
                    }
                    is IntListGenerator -> mutableListOf<T>().also { list ->
                        val start = listExpr.start.eval()
                        val end = listExpr.end.eval()
                        val stp = listExpr.step.eval()
                        if (listExpr.inclusive) {
                            list.addAll((start .. end step stp).map { it as T })
                        } else {
                            list.addAll((start until end step stp).map { it as T })
                        }
                    }
                    else -> unreachable()
                }
            }
        }.apply(this)
    }

    private fun DecomposedWithBlock.eval() {
        circuitTranspiler.pauseCtrl()
        this.withBody.eval()
        circuitTranspiler.restoreCtrl()
    }

    private fun QubitAccessor.eval(): Qubits {
        return when (this) {
            is QubitAccessorDeclarable -> this.eval()
            is QubitAccessorVariable -> this.eval()
            else -> unreachable()
        }
    }

    private fun QubitAccessorVariable.eval(): Qubits {
        return stateVariables[this.ident]?.qubits ?: run {
            throw RuntimeException("Qubit ${this.ident} not found")
        }
    }

    private fun QubitAccessorDeclarable.eval(): Qubits {
        return when (this) {
            is QubitAccessorAlloc -> this.eval()
            is QubitAccessorConcat -> this.eval()
            is QubitAccessorIndexing -> this.eval()
            is QubitAccessorSlicing -> this.eval()
            else -> unreachable()
        }
    }

    private fun QubitAccessorAlloc.eval(): Qubits {
        return circuitTranspiler.allocQubits(
            this.size.eval().toUInt(),
            (this.init?.eval() ?: 0).toUInt()
        )
    }

    private fun QubitAccessorConcat.eval(): Qubits {
        return this.accessors.map { it.eval() }.reduce {
            qubits, accessor -> qubits + accessor
        }
    }

    private fun QubitAccessorIndexing.eval(): Qubits {
        return this.subject.eval().let { qubits ->
            val index = this.index.eval()
            if (index in qubits.indices) {
                listOf(qubits[index])
            } else {
                throw RuntimeException("Index $index out of bounds")
            }
        }
    }

    private fun QubitAccessorSlicing.eval(): Qubits {
        return this.subject.eval().slice(
            this.start.eval(),
            this.end.eval(),
            this.step.eval().toUInt(),
            this.inclusive
        )
    }

    private fun QuantumVariableAssignment.eval() {
        RuntimeQubitsVariable(this.variable.ident, this.value.eval()).let {
            if (stateVariables.put(this.variable.ident, it) != it) {
                throw RuntimeException("Variable ${this.variable.ident} already exists")
            }
        }
    }

    private fun ClassicalVariableAssignment<ClassicalTrait>.eval() {
        this.variable.assign(this.expr.eval())
    }

    private fun ElementaryOperationCall.eval() {
        val target = this.target.eval()
        when (val op = this.op) {
            is OperationExprStandard -> {
                val gate = op.gate.createGateInstance(op.gate.classicalParams)
                circuitTranspiler.applyGate(gate, target)
            }
            // is OperationExprUnitary
            is OperationExprUserDefined -> withInIsolatedScope {
                val operation = operations[op.ident] ?: run {
                    throw RuntimeException("Operation ${op.ident} not found")
                }
                val classicalArgs = op.classicalArgs.map { it.eval<Any>() }
                if (classicalArgs.size != operation.classicalParams.size) {
                    throw RuntimeException(
                        "Incorrect number of classical args, " +
                        "expected ${operation.classicalParams.size} found ${classicalArgs.size}"
                    )
                }
                // Declare args
                classicalArgs.forEachIndexed { index, arg ->
                    classicalVariables[operation.classicalParams[index].ident] = RuntimeVariable(
                        operation.classicalParams[index].ident, arg
                    )
                }
                operation.body!!.transpile().decompose().eval()
            }
        }
    }
}
