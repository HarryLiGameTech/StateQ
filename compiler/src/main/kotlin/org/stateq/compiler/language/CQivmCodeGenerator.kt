package org.stateq.compiler.language

import org.stateq.compiler.qivm.QivmCodeGenerator
import org.stateq.exception.unreachable
import org.stateq.expression.*
import org.stateq.gates.StandardGate
import org.stateq.compiler.CodeGenerator
import org.stateq.compiler.qivm.ProgramContextVariable
import org.stateq.module.classical.ConstantDef
import org.stateq.parameter.*
import org.stateq.polynomial.*
import org.stateq.qubit.*
import org.stateq.type.*

class CQivmCodeGenerator : QivmCodeGenerator() {

    override val beginCodeBlockToken: String = " {"
    override val endCodeBlockToken: String = "}"
    override val indentToken: String = "    "
    override val statementEndingToken: String = ";"
    private var counter = 0

    override fun beginFile() {
        line("/* Stateq Generated Code Begin */")
    }

    override fun endFile() {
        line("/* Stateq Generated Code End */")
    }

    override fun emitBoolExpr(expr: BoolExpr): String {
        return when (expr) {
            is BoolExprLiteralTrue -> "true"
            is BoolExprLiteralFalse -> "false"
            is BoolExprVariable -> expr.variable.ident
            is BoolExprNot -> "!(${expr.inner.emit()})"
            is BoolExprBinary -> when (expr.op) {
                BoolExprBinary.Operator.And -> "(${expr.lhs.emit()} && ${expr.rhs.emit()})"
                BoolExprBinary.Operator.Or  -> "(${expr.lhs.emit()} || ${expr.rhs.emit()})"
            }
            is BoolExprCompare<*> -> {
                when (expr.op) {
                    CompareOperator.Greater       ->  "(${expr.lhs.emit()} > ${expr.rhs.emit()})"
                    CompareOperator.Less          ->  "(${expr.lhs.emit()} < ${expr.rhs.emit()})"
                    CompareOperator.GreaterEqual  ->  "(${expr.lhs.emit()} >= ${expr.rhs.emit()})"
                    CompareOperator.LessEqual     ->  "(${expr.lhs.emit()} <= ${expr.rhs.emit()})"
                    CompareOperator.Equal         ->  "(${expr.lhs.emit()} == ${expr.rhs.emit()})"
                    CompareOperator.NotEqual      ->  "(${expr.lhs.emit()} != ${expr.rhs.emit()})"
                }
            }
            else -> unreachable()
        }
    }

    override fun emitIntExpr(expr: IntExpr): String {
        return expr.format { indeterminate, exponent ->
            when (indeterminate) {
                is IntVariable -> indeterminate.ident
                is IndeterminateLikeDivision -> "(${indeterminate.lhs.emit()} / ${indeterminate.rhs.emit()})"
                is IndeterminateLikePower -> "powi(${indeterminate.lhs.emit()}, ${indeterminate.rhs.emit()})"
                is IndeterminateLikeAnd -> "(${indeterminate.lhs.emit()} & ${indeterminate.rhs.emit()})"
                is IndeterminateLikeOr -> "(${indeterminate.lhs.emit()} | ${indeterminate.rhs.emit()})"
                is IndeterminateLikeXor -> "(${indeterminate.lhs.emit()} ^ ${indeterminate.rhs.emit()})"
                is IndeterminateLikeModulo -> "(${indeterminate.lhs.emit()} % ${indeterminate.rhs.emit()})"
                is IndeterminateLikeShiftLeft -> "(${indeterminate.lhs.emit()} << ${indeterminate.rhs.emit()})"
                is IndeterminateLikeShiftRight -> "(${indeterminate.lhs.emit()} >> ${indeterminate.rhs.emit()})"
                is IndeterminateLikeLogicalShiftRight -> "((uint64_t) ${indeterminate.lhs.emit()} >> ${indeterminate.rhs.emit()})"
                is IndeterminateLikeFuncCall -> "${indeterminate.function.ident}(${
                    indeterminate.args.map { it.emit() }.toCommaSeperatedString()
                })"
                else -> unreachable()
            }.let { base ->
                if (exponent == 1u) base else "powi($base, $exponent)"
            }
        }
    }

    override fun emitFloatExpr(expr: FloatExpr): String {
        return when (expr) {
            is FloatExprLiteral -> expr.value.toString()
            is FloatExprVariable -> expr.variable.ident
            is FloatExprNegative -> "(-${expr.emit()})"
            is FloatExprBinary -> when (expr.op) {
                FloatExprBinary.Operator.Add -> "(${expr.lhs.emit()} + ${expr.rhs.emit()})"
                FloatExprBinary.Operator.Sub -> "(${expr.lhs.emit()} - ${expr.rhs.emit()})"
                FloatExprBinary.Operator.Div -> "(${expr.lhs.emit()} / ${expr.rhs.emit()})"
                FloatExprBinary.Operator.Mul -> "(${expr.lhs.emit()} * ${expr.rhs.emit()})"
                FloatExprBinary.Operator.Pow -> "powl(${expr.lhs.emit()}, ${expr.rhs.emit()})"
            }
            is FloatExprFromInt -> "((float) (${expr.inner.emit()}))"
            is FloatExprFuncCall -> "${expr.function.ident}(${
                expr.args.map { it.emit() }.toCommaSeperatedString()
            })"
            else -> unreachable()
        }
    }

    override fun emitComplexExpr(expr: ComplexExpr): String {
        TODO("Not yet implemented")
    }

    override fun emitBitsExpr(expr: BitsExpr): String {
        return when (expr) {
            is BitsExprVariable -> expr.variable.ident
            else -> TODO("Not yet implemented")
        }
    }

    override fun emitListExpr(expr: ListExpr<*>): String {
        val elementTypeIdent = expr.type.elementType.ident
        return when (expr) {
            is ListExprVariable<*> -> expr.variable.ident
            is ListExprLiteral<*> -> {
                expr.elements.map { it.emit() }.let { elements ->
                    elements.fold("stateq_list_new(${elements.size}, sizeof($elementTypeIdent), ") {
                        acc, element -> "$acc, ($elementTypeIdent) $element"
                    } + ")"
                }
            }
            is ListExprEmpty<*> -> "stateq_empty_list()"
            is ListExprSlicing<*> -> "stateq_list_slice(" +
                "list, sizeof($elementTypeIdent), " +
                "${expr.start.emit()}, ${expr.end?.emit() ?: -1}, ${expr.step.emit()})"
            else -> TODO()
        }
    }

    override val ReturnType.ident get() = when (this) {
        ClassicalType.Bool -> "bool"
        ClassicalType.Int -> "int64_t"
        ClassicalType.Float -> "double"
        ClassicalType.Bits -> "StateqBits"
        ClassicalType.Complex -> "StateqComplex"
        ClassicalType.Mat -> "StateqMat"
        is ClassicalListType -> "StateqList"
        is MeasurementResultType -> "RawMeasurementResult"
        else -> TODO("not implemented yet")
    }

    val ClassicalType.simpleIdent get() = when (this) {
        is ClassicalListType -> "list"
        else -> this.toString().lowercase()
    }

    override val Variable.typename get() = when (this) {
        is ClassicalVariable -> this.type.ident
        is QuantumVariable -> "QubitAccessor*"
        is ProgramContextVariable -> "QuantumProgramContext*"
        else -> unreachable()
    }

    override fun defClassicalFunction(
        returnType: ReturnType?,
        ident: String,
        params: List<ClassicalVariable>,
        functionBody: CodeBuilder
    ) {
        TODO("Not yet implemented")
    }

    override fun defExternClassicalFunction(returnType: ReturnType?, ident: String, params: List<ClassicalVariable>) {
        functionDefHelper(returnType, params) { type, paramList ->
            statement("extern $type $ident($paramList)")
        }
    }

    override fun defOperation(
        ident: String,
        doExport: Boolean,
        classicalParams: List<ClassicalVariable>,
        quantumParams: List<QuantumVariable>,
        functionBody: CodeBuilder
    ) {
        defFunction(null, ident, listOf(ProgramContextVariable) + classicalParams + quantumParams, CodeBuilder {
            enterStackFrame()
            functionBody.dump()
            exitStackFrame()
        })
    }

    override fun defExternOperation(
        ident: String,
        doExport: Boolean,
        classicalParams: List<ClassicalVariable>,
        quantumParams: List<QuantumVariable>
    ) {
        TODO("Not yet implemented")
    }

    override fun defProgram(
        ident: String, classicalParams: List<ClassicalVariable>,
        shots: IntExpr, functionBody: CodeBuilder
    ) {
        defFunction(MeasurementResultType, ident, classicalParams, CodeBuilder {
            getProgramCtx()
            enterStackFrame()
            functionBody.dump()
            exitStackFrame()
            executeProgram(shots)
            destroyProgramAndReturnResult()
        })
    }

    override fun defExternProgram(ident: String, classicalParams: List<ClassicalVariable>) {
        TODO("Not yet implemented")
    }

    private fun functionDefHelper(
        returnType: ReturnType?, params: List<Variable>,
        action: (paramList: String, mangledName: String) -> Unit
    ) {
        val type = returnType?.ident ?: "void"
        val paramList = params.map { "${it.typename} ${it.ident}" }.toCommaSeperatedString()
        action(type, paramList)
        emptyLine()
    }

    private fun defFunction(
        returnType: ReturnType?, ident: String,
        params: List<Variable>, functionBody: CodeBuilder
    ) {
        functionDefHelper(returnType, params) { type, paramList ->
            statement("$type $ident($paramList)") {
                functionBody.dump()
            }
        }
    }

    override fun defConstant(constant: ConstantDef) {
        statement("const ${constant.variable.typename} ${constant.variable.ident} = ${constant.value.emit()}")
        emptyLine()
    }

    override fun classicalVariableInitialization(variable: ClassicalVariable, expr: ClassicalExpr) {
        statement("${variable.type.ident} ${variable.ident} = ${expr.emit()}")
    }

    private val ClassicalVariable.indexIterIdent get() = "iterIndex_${this.ident}"

    private fun <T: ClassicalTrait> IterableExpr<T>.toClassicalExpr(): ClassicalExpr {
        return if (this is ClassicalExpr) this else unreachable()
    }

    override fun forLoop(
        loopIter: String,
        from: IntExpr, to: IntExpr, step: IntExpr, inclusive: Boolean,
        loopBody: CodeBuilder
    ) {
        val cmp = if (inclusive) "<=" else "<"
        statement("for (int $loopIter = ${from.emit()}; $loopIter $cmp ${to.emit()}; $loopIter += ${step.emit()})") {
            loopBody.dump()
        }
    }

    private var iterableCounter = 0

    override fun <T : ClassicalTrait> forEachLoop(
        loopIter: ClassicalVariable, iterable: IterableExpr<T>, loopBody: CodeBuilder
    ) {
        val indexIter = loopIter.indexIterIdent
        val iterableExpr = iterable.toClassicalExpr()
        val iterableHash = (++iterableCounter * iterableExpr.hashCode()).toUInt()
        val iterableIdent = "_iterable_${iterable.iterableType}_${iterableHash.toString(16)}"
        val iterableSize = "stateq_get_size_of_${iterableExpr.type.simpleIdent}(${iterableIdent})"
        statement("${iterableExpr.type.ident} $iterableIdent = ${iterableExpr.emit()}")
        statement("for (int $indexIter = 0; $indexIter < $iterableSize; $indexIter++)") {
            val indexing = "stateq_get_index_of_${iterableExpr.type.simpleIdent}($iterableIdent, $indexIter)"
            statement("${loopIter.type.ident} ${loopIter.ident} = $indexing")
            loopBody.dump()
        }
    }

    override fun ifStatement(condition: BoolExpr, ifBranch: CodeBuilder, elseBranch: CodeBuilder?) {
        statement("if (${condition.emit()})") { ifBranch.dump() }
        statement("else") { elseBranch?.dump() }
    }

    override fun withStatement(
        withExprBuilder: CodeGenerator.() -> CodeBuilder,
        withBody: CodeGenerator.() -> CodeBuilder
    ) {
        pauseCtrl()
        withExprBuilder().dump()
        restoreCtrl()

        withBody().dump()

        pauseCtrl()
        this.beginDagger()
        withExprBuilder().dump()
        this.endDagger()
        restoreCtrl()
    }

    override fun declareQubitAccessorAllocInner(accessor: QubitAccessorAlloc) {
        statement("QubitAccessor* ${accessor.ident} = qivm_alloc_qubits($ctx, ${accessor.size.emit()})")
        accessor.init?.also { value ->
            qubitAccessorEncode(accessor.ident, value)
        }
    }

    override fun declareQubitAccessorConcatInner(accessor: QubitAccessorConcat) {
        assert(accessor.accessors.size > 1)
        statement("QubitAccessor* ${accessor.ident} = ${accessor.accessors[0].ident}")
        accessor.accessors.drop(1).forEach {
            statement("""
                ${accessor.ident} = qivm_qubit_accessor_concat(
                    $ctx, ${accessor.ident}, ${it.ident}
                )
            """.trimIndent())
        }
    }

    override fun declareQubitAccessorSlicingInner(accessor: QubitAccessorSlicing) {
        val start = accessor.start
        val end = accessor.end + if (accessor.inclusive) 1 else 0
        val step = accessor.step
        statement("""
            QubitAccessor* ${accessor.ident} = qivm_qubit_accessor_slicing(
                $ctx, ${accessor.subject.ident}, ${start.emit()}, ${(end - 1).emit()}, ${step.emit()}
            )
        """.trimIndent())
    }

    override fun declareQubitAccessorIndexingInner(accessor: QubitAccessorIndexing) {
        statement("""
            QubitAccessor* ${accessor.ident} = qivm_qubit_accessor_indexing(
                $ctx, ${accessor.subject.ident}, ${accessor.index.emit()}
            )
        """.trimIndent())
    }

    override fun quantumVariableAssignmentInner(ident: String, accessor: QubitAccessor) {
        statement("QubitAccessor* $ident = ${accessor.ident}")
    }

    override fun qubitAccessorEncode(ident: String, value: IntExpr) {
        statement("qivm_qubit_accessor_encode($ctx, $ident, ${value.emit()})")
    }

    override fun beginControl(ctrlQubits: String, condition: Boolean) {
        statement("qivm_program_begin_ctrl($ctx, $ctrlQubits, $condition)")
    }

    override fun endControl(ctrlQubits: String) {
        statement("qivm_program_end_ctrl($ctx, $ctrlQubits)")
    }

    override fun beginDagger() {
        statement("qivm_program_begin_dagger($ctx)")
    }

    override fun endDagger() {
        statement("qivm_program_end_dagger($ctx)")
    }

    override fun getProgramCtx() {
        statement("QuantumProgramContext* $ctx = qivm_get_program_ctx()")
    }

    override fun destroyProgramCtx() {
        statement("qivm_destroy_program_ctx($ctx)")
    }

    override fun destroyProgramAndReturnResult() {
        statement("return stateq_program_get_result_and_destroy($ctx)")
    }

    override fun executeProgram(shots: IntExpr) {
        statement("qivm_exec_program($ctx, ${shots.emit()})")
    }

    override fun pauseCtrl() {
        statement("qivm_program_pause_ctrl($ctx)")
    }

    override fun restoreCtrl() {
        statement("qivm_program_restore_ctrl($ctx)")
    }

    override fun enterStackFrame() {
        statement("qivm_stack_enter($ctx)")
    }

    override fun exitStackFrame() {
        statement("qivm_stack_exit($ctx)")
    }

    override fun pushStdBuiltinOp(gate: StandardGate, args: List<ClassicalExpr>, target: String) {
        val argListIdent = "gateArgs_${gate.name}_${counter++}"
        statement("GateArgument $argListIdent[${args.size}] = {${
            args.mapIndexed { index, expr -> 
                when (expr) {
                    is IntExpr -> "[$index].int_val = ${expr.emit()}"
                    is FloatExpr -> "[$index].float_val = ${expr.emit()}"
                    else -> unreachable("Unsupported argument type ${expr.type}")
                }
            }.joinToString(", ")
        }}")
        statement("qivm_program_push_op($ctx, \"${gate.name}\", $target, (uint64_t*) $argListIdent, ${args.size})")
    }

    override fun pushCustomBuiltinOp(gateIdent: String, args: List<ClassicalExpr>, target: String) {
        TODO("Not yet implemented")
    }

    override fun measureInner(target: QubitAccessor) {
        statement("qivm_measure($ctx, ${target.ident})")
    }

    override fun classicalFunctionCall(funcIdent: String, args: List<ClassicalExpr>) {
        statement("$funcIdent(${args.map { it.emit() }.toCommaSeperatedString()})")
    }

    override fun operationCall(
        operation: OperationExprElementary,
        classicalArgs: List<ClassicalExpr>,
        quantumArgs: List<QubitAccessor>
    ) {
        statement("${operation.ident}($ctx, ${
            classicalArgs.map { it.emit() }.toCommaSeperatedString()
        }, ${
            quantumArgs.map { it.use().ident }.toCommaSeperatedString()
        })")
    }
}
