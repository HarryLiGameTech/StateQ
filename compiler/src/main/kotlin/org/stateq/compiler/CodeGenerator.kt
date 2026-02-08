package org.stateq.compiler

import org.stateq.exception.unreachable
import org.stateq.expression.*
import org.stateq.gates.StandardGate
import org.stateq.module.classical.ConstantDef
import org.stateq.parameter.ClassicalVariable
import org.stateq.parameter.QuantumVariable
import org.stateq.parameter.Variable
import org.stateq.qubit.*
import org.stateq.type.ClassicalTrait
import org.stateq.type.ReturnType
import org.stateq.datastructure.ScopedSet

abstract class CodeGenerator(
    private var codeBuilder: StringBuilder = StringBuilder(),
) {
    protected abstract val beginCodeBlockToken: String
    protected abstract val endCodeBlockToken: String
    protected abstract val indentToken: String
    protected abstract val statementEndingToken: String

    private var indents: Int = 0

    private val declaredQubitAccessors = ScopedSet<QubitAccessor>()

    fun dumpCode() = codeBuilder.toString()

    abstract fun beginFile()

    abstract fun endFile()

    protected fun line(code: String) {
        codeBuilder.append(indentToken.repeat(indents)).append(code).appendLine()
    }

    protected fun emptyLine() = codeBuilder.appendLine()

    protected fun statement(code: String, action: (() -> Unit)? = null) {
        val foldedCode = code.split("\n").fold("") { acc, s -> acc + s.trim() }
        action?.let {
            this.token(indentToken.repeat(indents) + foldedCode)
            codeBlock(it)
        } ?: run {
            this.line(foldedCode + statementEndingToken)
        }
    }

    private fun token(token: String) {
        codeBuilder.append(token)
    }

    protected fun List<String>.toCommaSeperatedString(): String {
        return this.fold("") { acc, s -> "$acc, $s" }.trimStart(',').trim()
    }

    protected fun ClassicalExpr.emit() = when (this) {
        is BoolExpr -> emitBoolExpr(this)
        is IntExpr -> emitIntExpr(this)
        is FloatExpr -> emitFloatExpr(this)
        is ComplexExpr -> emitComplexExpr(this)
        is BitsExpr -> emitBitsExpr(this)
        is ListExpr<*> -> emitListExpr(this)
        else -> TODO("Not implemented yet")
    }

    protected abstract fun emitBoolExpr(expr: BoolExpr): String

    protected abstract fun emitIntExpr(expr: IntExpr): String

    protected abstract fun emitFloatExpr(expr: FloatExpr): String

    protected abstract fun emitComplexExpr(expr: ComplexExpr): String

    protected abstract fun emitBitsExpr(expr: BitsExpr): String

    protected abstract fun emitListExpr(expr: ListExpr<*>): String

    protected abstract val ReturnType.ident: String

    protected open val Variable.typename get() = when (this) {
        is ClassicalVariable -> this.type.ident
        is QuantumVariable -> "QubitAccessor"
        else -> unreachable()
    }

    private fun QubitAccessor.isDeclared(): Boolean = declaredQubitAccessors.contains(this)

    private fun QubitAccessor.declareIfNeeded(): Boolean {
        return if (this.isDeclared()) false else {
            this.declare()?.let {
                it.emit(this@CodeGenerator).let { true }
            } ?: false
        }
    }

    protected fun QubitAccessor.use(): QubitAccessor {
        return this.also {
            this.declareIfNeeded()
        }
    }

    inner class CodeBuilder(val action: CodeGenerator.() -> Unit)

    protected fun CodeBuilder.dump() = this.action(this@CodeGenerator)

    protected fun CodeBuilder.dumpCodeBlock() = this@CodeGenerator.codeBlock {
        this.action(this@CodeGenerator)
    }

    private fun <T> codeBlock(action: () -> T) {
        this.token(beginCodeBlockToken)
        this.line("")
        indents += 1
        this.declaredQubitAccessors.enterScope()
        action()
        this.declaredQubitAccessors.exitScope()
        indents -= 1
        this.line(endCodeBlockToken)
    }

    fun defClassicalFunction(
        returnType: ReturnType?, ident: String,
        params: List<ClassicalVariable>, functionBody: CodeGenerator.() -> CodeBuilder?
    ) {
        functionBody()?.also {
            this.defClassicalFunction(returnType, ident, params, it)
        } ?: run {
            this.defExternClassicalFunction(returnType, ident, params)
        }
    }

    protected abstract fun defClassicalFunction(
        returnType: ReturnType?, ident: String,
        params: List<ClassicalVariable>, functionBody: CodeBuilder
    )

    protected abstract fun defExternClassicalFunction(
        returnType: ReturnType?, ident: String, params: List<ClassicalVariable>
    )

    fun defOperation(
        ident: String, doExport: Boolean, classicalParams: List<ClassicalVariable>,
        quantumParams: List<QuantumVariable>, functionBody: CodeGenerator.() -> CodeBuilder?
    ) {
        functionBody()?.also {
            this.defOperation(ident, doExport, classicalParams, quantumParams, it)
        } ?: {
            this.defExternOperation(ident, doExport, classicalParams, quantumParams)
        }
    }

    protected abstract fun defOperation(
        ident: String, doExport: Boolean, classicalParams: List<ClassicalVariable>,
        quantumParams: List<QuantumVariable>, functionBody: CodeBuilder,
    )

    protected abstract fun defExternOperation(
        ident: String, doExport: Boolean, classicalParams: List<ClassicalVariable>,
        quantumParams: List<QuantumVariable>,
    )

    fun defProgram(
        ident: String, classicalParams: List<ClassicalVariable>,
        shots: IntExpr, functionBody: CodeGenerator.() -> CodeBuilder?
    ) {
        functionBody()?.also {
            this.defProgram(ident, classicalParams, shots, it)
        } ?: {
            this.defExternProgram(ident, classicalParams)
        }
    }

    protected abstract fun defExternProgram(
        ident: String, classicalParams: List<ClassicalVariable>
    )

    protected abstract fun defProgram(
        ident: String, classicalParams: List<ClassicalVariable>,
        shots: IntExpr, functionBody: CodeBuilder,
    )

    abstract fun defConstant(constant: ConstantDef)

    abstract fun classicalVariableInitialization(variable: ClassicalVariable, expr: ClassicalExpr)

    fun forLoop(
        loopIter: String,
        from: IntExpr, to: IntExpr, step: IntExpr, inclusive: Boolean,
        loopBody: CodeGenerator.() -> CodeBuilder
    ) {
        this.forLoop(loopIter, from, to, step, inclusive, loopBody())
    }

    protected abstract fun forLoop(
        loopIter: String,
        from: IntExpr, to: IntExpr, step: IntExpr, inclusive: Boolean,
        loopBody: CodeBuilder,
    )

    fun <T: ClassicalTrait> forEachLoop(
        loopIter: ClassicalVariable, iterable: IterableExpr<T>,
        loopBody: CodeGenerator.() -> CodeBuilder,
    ) {
        if (iterable is IntListGenerator) {
            this.forLoop(
                loopIter = loopIter.ident,
                from = iterable.start,
                to = iterable.end,
                step = iterable.step,
                inclusive = iterable.inclusive,
                loopBody = loopBody(),
            )
        } else {
            this.forEachLoop(loopIter, iterable, loopBody())
        }
    }

    abstract fun <T: ClassicalTrait> forEachLoop(
        loopIter: ClassicalVariable, iterable: IterableExpr<T>,
        loopBody: CodeBuilder,
    )

    inner class IfStatementBuilder(
        private val condition: BoolExpr,
        private val ifBranch: CodeGenerator.() -> CodeBuilder
    ) {

        var elseBranch: (CodeGenerator.() -> CodeBuilder)? = null

        fun elseBranch(branch: CodeGenerator.() -> CodeBuilder?) = this.also {
            it.elseBranch = branch()?.let {{ it }}
        }

        fun transpile() {
            this@CodeGenerator.ifStatement(
                condition = condition,
                ifBranch = ifBranch(),
                elseBranch = elseBranch?.let { it() }
            )
        }
    }

    fun ifStatement(condition: BoolExpr, ifBranch: CodeGenerator.() -> CodeBuilder): IfStatementBuilder {
        return IfStatementBuilder(condition, ifBranch)
    }

    protected abstract fun ifStatement(
        condition: BoolExpr,
        ifBranch: CodeBuilder,
        elseBranch: CodeBuilder?,
    )

    abstract fun declareQubitAccessorAllocInner(accessor: QubitAccessorAlloc)

    fun declareQubitAccessorAlloc(accessor: QubitAccessorAlloc) {
        this.declaredQubitAccessors.add(accessor)
        this.declareQubitAccessorAllocInner(accessor)
    }

    abstract fun declareQubitAccessorConcatInner(accessor: QubitAccessorConcat)

    fun declareQubitAccessorConcat(accessor: QubitAccessorConcat) {
        accessor.accessors.forEach { it.declareIfNeeded() }
        this.declaredQubitAccessors.add(accessor)
        this.declareQubitAccessorConcatInner(accessor)
    }

    abstract fun declareQubitAccessorSlicingInner(accessor: QubitAccessorSlicing)

    fun declareQubitAccessorSlicing(accessor: QubitAccessorSlicing) {
        accessor.subject.declareIfNeeded()
        this.declaredQubitAccessors.add(accessor)
        this.declareQubitAccessorSlicingInner(accessor)
    }

    abstract fun declareQubitAccessorIndexingInner(accessor: QubitAccessorIndexing)

    fun declareQubitAccessorIndexing(accessor: QubitAccessorIndexing) {
        accessor.subject.declareIfNeeded()
        this.declaredQubitAccessors.add(accessor)
        this.declareQubitAccessorIndexingInner(accessor)
    }

    protected abstract fun quantumVariableAssignmentInner(
        ident: String, accessor: QubitAccessor
    )

    fun quantumVariableAssignment(ident: String, accessor: QubitAccessor) {
        accessor.declareIfNeeded()
        this.quantumVariableAssignmentInner(ident, accessor)
    }

    protected abstract fun qubitAccessorEncode(ident: String, value: IntExpr)
    
    fun qubitAccessorEncode(accessor: QubitAccessor, value: IntExpr) {
        accessor.declareIfNeeded()
        this.qubitAccessorEncode(accessor.ident, value)
    }

    protected abstract fun beginControl(ctrlQubits: String, condition: Boolean)

    fun beginControl(ctrlQubits: QubitAccessor, condition: Boolean) {
        ctrlQubits.declareIfNeeded()
        this.beginControl(ctrlQubits.ident, condition)
    }

    protected abstract fun endControl(ctrlQubits: String)

    fun endControl(ctrlQubits: QubitAccessor) {
        ctrlQubits.declareIfNeeded()
        this.endControl(ctrlQubits.ident)
    }

    abstract fun beginDagger()
    abstract fun endDagger()

    fun <R> dagger(action: () -> R) {
        this.beginDagger()
        action().also {
            this.endDagger()
        }
    }

    open fun withStatement(
        withExprBuilder: CodeGenerator.() -> CodeBuilder,
        withBody: CodeGenerator.() -> CodeBuilder
    ) {
        withExprBuilder().dump()
        withBody().dump()
        this.beginDagger()
        withExprBuilder().dump()
        this.endDagger()
    }

    protected abstract fun pushStdBuiltinOp(
        gate: StandardGate, args: List<ClassicalExpr>, target: String
    )

    fun pushStdBuiltinOp(gate: StandardGate, args: List<ClassicalExpr>, target: QubitAccessor) {
        target.declareIfNeeded()
        this.pushStdBuiltinOp(gate, args, target.ident)
    }

    protected abstract fun pushCustomBuiltinOp(
        gateIdent: String, args: List<ClassicalExpr>, target: String
    )

    fun pushCustomBuiltinOp(gateIdent: String, args: List<ClassicalExpr>, target: QubitAccessor) {
        target.declareIfNeeded()
        this.pushCustomBuiltinOp(gateIdent, args, target.ident)
    }

    fun measure(target: QubitAccessor) {
        target.declareIfNeeded()
        measureInner(target)
    }

    protected abstract fun measureInner(target: QubitAccessor)

    protected abstract fun classicalFunctionCall(funcIdent: String, args: List<ClassicalExpr>)

    abstract fun operationCall(
        operation: OperationExprElementary, classicalArgs: List<ClassicalExpr>, quantumArgs: List<QubitAccessor>
    )
}
