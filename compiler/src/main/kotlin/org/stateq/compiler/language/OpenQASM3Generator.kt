package org.stateq.compiler.language

import org.stateq.compiler.CodeGenerator
import org.stateq.exception.unreachable
import org.stateq.expression.*
import org.stateq.gates.StandardGate
import org.stateq.module.classical.ConstantDef
import org.stateq.parameter.ClassicalVariable
import org.stateq.parameter.IntVariable
import org.stateq.parameter.QuantumVariable
import org.stateq.parameter.Variable
import org.stateq.polynomial.*
import org.stateq.qubit.*
import org.stateq.type.ClassicalListType
import org.stateq.type.ClassicalTrait
import org.stateq.type.ClassicalType
import org.stateq.type.ReturnType
import org.stateq.util.format

class OpenQASM3Generator : CodeGenerator() {

    override val beginCodeBlockToken: String = "{"
    override val endCodeBlockToken: String = "}"
    override val indentToken: String = "    "
    override val statementEndingToken: String = ";"

    override fun beginFile() {
        TODO("Not yet implemented")
    }

    override fun endFile() {
        TODO("Not yet implemented")
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
                is IndeterminateLikePower -> "(${indeterminate.lhs.emit()} ** ${indeterminate.rhs.emit()})"
                is IndeterminateLikeAnd -> "(${indeterminate.lhs.emit()} & ${indeterminate.rhs.emit()})"
                is IndeterminateLikeOr -> "(${indeterminate.lhs.emit()} | ${indeterminate.rhs.emit()})"
                is IndeterminateLikeXor -> "(${indeterminate.lhs.emit()} ^ ${indeterminate.rhs.emit()})"
                is IndeterminateLikeModulo -> "(${indeterminate.lhs.emit()} % ${indeterminate.rhs.emit()})"
                is IndeterminateLikeShiftLeft -> "(${indeterminate.lhs.emit()} << ${indeterminate.rhs.emit()})"
                is IndeterminateLikeShiftRight -> "(${indeterminate.lhs.emit()} >> ${indeterminate.rhs.emit()})"
                is IndeterminateLikeLogicalShiftRight -> TODO("logical shift")
                is IndeterminateLikeFuncCall -> "${indeterminate.function.ident}(${
                    indeterminate.args.map { it.emit() }.toCommaSeperatedString()
                })"
                else -> unreachable()
            }.let { base ->
                if (exponent == 1u) base else "($base ** $exponent)"
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
                FloatExprBinary.Operator.Div -> "(${expr.lhs.emit()} * ${expr.rhs.emit()})"
                FloatExprBinary.Operator.Mul -> "(${expr.lhs.emit()} / ${expr.rhs.emit()})"
                FloatExprBinary.Operator.Pow -> "(${expr.lhs.emit()} ** ${expr.rhs.emit()})"
            }
            is FloatExprFromInt -> "float(${expr.inner.emit()})"
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
        TODO("Not yet implemented")
    }

    override fun emitListExpr(expr: ListExpr<*>): String {
        TODO("Not yet implemented")
    }

    override val ReturnType.ident: String get() = when (this) {
        ClassicalType.Bool -> "bool"
        ClassicalType.Int -> "int"
        ClassicalType.Float -> "float"
        ClassicalType.Bits -> "bit"
        ClassicalType.Complex -> "complex"
        is ClassicalListType -> "array[${this.elementType.ident}]"
        else -> unreachable()
    }

    override fun defClassicalFunction(
        returnType: ReturnType?,
        ident: String,
        params: List<ClassicalVariable>,
        functionBody: CodeBuilder
    ) {
        val paramList = params.format {
            "${it.type.ident} ${it.ident}"
        }
        val returnStr = returnType?.let { " -> ${it.ident}" } ?: ""
        statement("def $ident($paramList)$returnStr") {
            functionBody.dump()
        }
    }

    override fun defExternClassicalFunction(returnType: ReturnType?, ident: String, params: List<ClassicalVariable>) {
        val paramList = params.format {
            "${it.type.ident} ${it.ident}"
        }
        val returnStr = returnType?.let { " -> ${it.ident}" } ?: ""
        statement("extern $ident($paramList)$returnStr")
    }

    override fun defExternOperation(
        ident: String,
        doExport: Boolean,
        classicalParams: List<ClassicalVariable>,
        quantumParams: List<QuantumVariable>
    ) {
        val paramList = (classicalParams + quantumParams).format {
            "${it.typename} ${it.ident}"
        }
        statement("extern $ident($paramList)")
    }

    override fun defProgram(
        ident: String,
        classicalParams: List<ClassicalVariable>,
        shots: IntExpr,
        functionBody: CodeBuilder
    ) {
        val paramList = classicalParams.format {
            "${it.type.ident} ${it.ident}"
        }
        statement("// Program: $ident with ${shots.emit()} shots")
        statement("def $ident($paramList)") {
            functionBody.dump()
        }
    }

    override fun defExternProgram(ident: String, classicalParams: List<ClassicalVariable>) {
        val paramList = classicalParams.format {
            "${it.type.ident} ${it.ident}"
        }
        statement("extern $ident($paramList)")
    }

    override fun defOperation(
        ident: String,
        doExport: Boolean,
        classicalParams: List<ClassicalVariable>,
        quantumParams: List<QuantumVariable>,
        functionBody: CodeBuilder
    ) {
        val paramList = (classicalParams + quantumParams).format {
            "${it.typename} ${it.ident}"
        }
        statement("def $ident($paramList)") {
            functionBody.dump()
        }
    }

    fun defFunction(
        returnType: ReturnType?,
        ident: String,
        params: List<Variable>,
        functionBody: CodeBuilder
    ) {
        val paramList = params.format {
            "${it.typename} ${it.ident}"
        }
        val returnStr = returnType?.let { " -> ${it.ident}" }
        statement("def $ident($paramList)${returnStr ?: ""}") {
            functionBody.dump()
        }
    }

    override fun defConstant(constant: ConstantDef) {
        statement("${constant.variable.type.ident} ${constant.variable.ident} = ${constant.value.emit()}")
    }

    override fun classicalVariableInitialization(variable: ClassicalVariable, expr: ClassicalExpr) {
        statement("${variable.type.ident} ${variable.ident} = ${expr.emit()}")
    }

    override fun forLoop(
        loopIter: String,
        from: IntExpr,
        to: IntExpr,
        step: IntExpr,
        inclusive: Boolean,
        loopBody: CodeBuilder
    ) {
        val endOp = if (inclusive) " + 1" else ""
        statement("for int $loopIter in [${from.emit()}:${step.emit()}:${to.emit()}$endOp]") {
            loopBody.dump()
        }
    }

    override fun <T : ClassicalTrait> forEachLoop(
        loopIter: ClassicalVariable,
        iterable: IterableExpr<T>,
        loopBody: CodeBuilder
    ) {
        statement("for ${loopIter.ident} in ${(iterable as ClassicalExpr).emit()}") {
            loopBody.dump()
        }
    }

    override fun ifStatement(condition: BoolExpr, ifBranch: CodeBuilder, elseBranch: CodeBuilder?) {
        statement("if (${condition.emit()})") {
            ifBranch.action(this)
        }
        elseBranch?.let {
            statement("else") {
                elseBranch.action(this)
            }
        }
    }

    override fun declareQubitAccessorAllocInner(accessor: QubitAccessorAlloc) {
        statement("qubit[${accessor.size.emit()}] ${accessor.ident}")
        accessor.init?.also { value ->
            qubitAccessorEncode(accessor.ident, value)
        }
    }

    override fun declareQubitAccessorConcatInner(accessor: QubitAccessorConcat) {
        statement("""
            let ${accessor.ident} = ${
                accessor.accessors.fold("") { acc, element ->
                    "$acc ++ ${element.ident}"
                }.trim('+', ' ')
            }
        """.trimIndent())
    }

    override fun declareQubitAccessorSlicingInner(accessor: QubitAccessorSlicing) {
        val start = accessor.start.emit()
        val end = accessor.end.emit()
        val step = accessor.step.emit()
        if (accessor.inclusive) {
            val inclusiveVar = "sliceInc_${accessor.ident}"
            statement("int $inclusiveVar = 1")
            statement("if ($inclusiveVar < 0)") {
                statement("$inclusiveVar = -1")
            }
            statement("""
                let ${accessor.ident} = ${accessor.subject.ident}[$start : $end : $step + $inclusiveVar]
            """.trimIndent())
        } else {
            statement("""
                let ${accessor.ident} = ${accessor.subject.ident}[$start : $end : $step]
            """.trimIndent())
        }
    }

    override fun declareQubitAccessorIndexingInner(accessor: QubitAccessorIndexing) {
        statement("""
            let ${accessor.index} = ${accessor.subject.ident}[${accessor.index.emit()}]
        """.trimIndent())
    }

    override fun quantumVariableAssignmentInner(ident: String, accessor: QubitAccessor) {
        statement("let $ident = ${accessor.ident}")
    }

    override fun qubitAccessorEncode(ident: String, value: IntExpr) {
        TODO()
    }

    override fun beginControl(ctrlQubits: String, condition: Boolean) {
        if (condition) {
            statement("ctrl @ {")
            statement("    // Control qubits: $ctrlQubits")
        } else {
            statement("negctrl @ {")
            statement("    // Negative control qubits: $ctrlQubits")
        }
    }

    override fun endControl(ctrlQubits: String) {
        statement("}")
    }

    override fun beginDagger() {
        statement("inv @ {")
    }

    override fun endDagger() {
        statement("}")
    }

    override fun pushStdBuiltinOp(gate: StandardGate, args: List<ClassicalExpr>, target: String) {
        val gateNameMap = mapOf(
            StandardGate.I to "id",
            StandardGate.H to "h",
            StandardGate.X to "x",
            StandardGate.Y to "y",
            StandardGate.Z to "z",
            StandardGate.S to "s",
            StandardGate.SD to "sdg",
            StandardGate.T to "t",
            StandardGate.TD to "tdg",
            StandardGate.V to "sx",
            StandardGate.VD to "sxdg",
            StandardGate.P to "p",
            StandardGate.RX to "rx",
            StandardGate.RY to "ry",
            StandardGate.RZ to "rz",
            StandardGate.U to "u",
            StandardGate.SWP to "swap",
            StandardGate.ISWP to "iswap"
        )

        val gateName = gateNameMap[gate] ?: gate.name.lowercase()
        val argStr = if (args.isNotEmpty()) {
            "(${args.joinToString(", ") { it.emit() }})"
        } else {
            ""
        }
        statement("$gateName$argStr $target")
    }

    override fun pushCustomBuiltinOp(gateIdent: String, args: List<ClassicalExpr>, target: String) {
        val argStr = if (args.isNotEmpty()) {
            "(${args.joinToString(", ") { it.emit() }})"
        } else {
            ""
        }
        statement("$gateIdent$argStr $target")
    }

    override fun measureInner(target: QubitAccessor) {
        statement("measure ${target.ident}")
    }

    override fun classicalFunctionCall(funcIdent: String, args: List<ClassicalExpr>) {
        statement("$funcIdent(${args.map { it.emit() }.joinToString(", ")})")
    }

    override fun operationCall(
        operation: OperationExprElementary,
        classicalArgs: List<ClassicalExpr>,
        quantumArgs: List<QubitAccessor>
    ) {
        val allArgs = (classicalArgs.map { it.emit() } + quantumArgs.map { it.use().ident }).joinToString(", ")
        statement("${operation.ident}($allArgs)")
    }
}
