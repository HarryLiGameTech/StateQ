package org.stateq.visitor

import org.antlr.v4.runtime.ParserRuleContext
import org.stateq.antlr.StateqBaseVisitor
import org.stateq.antlr.StateqParser.*
import org.stateq.builtin.ClassicalConstants
import org.stateq.builtin.ClassicalFunctions
import org.stateq.exception.unreachable
import org.stateq.expression.*
import org.stateq.gates.StandardGate
import org.stateq.datastructure.ScopedIdentMap
import org.stateq.module.*
import org.stateq.module.classical.*
import org.stateq.module.quantum.OperationDef
import org.stateq.module.quantum.ProgramDef
import org.stateq.parameter.*
import org.stateq.polynomial.*
import org.stateq.statement.*
import org.stateq.type.*
import org.stateq.util.*
import java.lang.ClassCastException
import java.nio.file.Path

typealias ClassicalVariableTable = ScopedIdentMap<ClassicalVariable>
typealias QuantumVariableTable = ScopedIdentMap<QuantumVariable>
typealias ClassicalFunctionTable = MutableMap<String, ClassicalFunction<ClassicalExpr>>
typealias OperationTable = MutableMap<String, OperationDef>

class StateqVisitor(val path: Path? = null) : StateqBaseVisitor<Any>() {

    private enum class ScopeType {
        GLOBAL, OPERATION, PROCEDURE, PROGRAM;
    }

    private var currentScopeType: ScopeType = ScopeType.GLOBAL

    private val functions: ClassicalFunctionTable = mutableMapOf()
    private val operations: OperationTable = mutableMapOf()
    private val classicalVariableTable: ClassicalVariableTable = ScopedIdentMap()
    private val quantumVariableTable: QuantumVariableTable = ScopedIdentMap()

    init {
        // Global scope
        classicalVariableTable.enterScope()
        classicalVariableTable.put("pi", ClassicalConstants.pi)
        functions["sin"] = ClassicalFunctions.sin
        functions["cos"] = ClassicalFunctions.cos
        functions["tan"] = ClassicalFunctions.tan
        functions["exp"] = ClassicalFunctions.exp
        functions["log2"] = ClassicalFunctions.log2
        functions["log2i"] = ClassicalFunctions.log2i
        functions["ceil"] = ClassicalFunctions.ceil
        functions["floor"] = ClassicalFunctions.floor
        functions["log"] = ClassicalFunctions.log
        functions["mpowi"] = ClassicalFunctions.mpowi
    }

    private fun <R> doInScope(scopeType: ScopeType, action: () -> R): R {
        val prevScopeType = this.currentScopeType
        this.currentScopeType = scopeType
        return this.doInScope(action).also {
            this.currentScopeType = prevScopeType
        }
    }

    private fun <R> doInScope(action: () -> R): R {
        classicalVariableTable.enterScope()
        quantumVariableTable.enterScope()
        val classicalVariableTableSize = classicalVariableTable.stackSize
        val quantumVariableTableSize = quantumVariableTable.stackSize
        return action().also {
            assert(classicalVariableTable.stackSize == classicalVariableTableSize)
            assert(quantumVariableTable.stackSize == quantumVariableTableSize)
            classicalVariableTable.exitScope()
            quantumVariableTable.exitScope()
        }
    }

    private inline fun <reified R: ClassicalExpr> callDeclaredFunction(
        ident: String, args: List<ClassicalExpr>, location: Location
    ): R {
        return functions[ident]?.let { function ->
            function.call(args, location).let {
                if (it is R) it else {
                    CompileError.error(location,
                        "Return type not match, expected ${R::class.typename} found ${it.type}"
                    ).raise()
                }
            }
        } ?: run {
            CompileError.error(location,
                "Calling undeclared function $ident"
            ).raise()
        }
    }

    private inline fun <reified V: ClassicalVariable> getVariable(ident: String, location: Location): V {
        return this.classicalVariableTable[ident]?.also {
            raiseCompileErrorIf(it !is V, location) {
                "Type not match, expected ${V::class.typename} got ${it.type}"
            }
        }?.let { it as V } ?: run {
            raiseCompileError(location) {
                "Undeclared variable $ident"
            }
        }
    }

    private val ParserRuleContext.location get() = Location(
        path, this.start.line, this.start.startIndex
    )

    // Module

    override fun visitModule(ctx: ModuleContext): Module {
        ctx.externs.forEach(this::visitExternFuncDef)
        return Module(
            ctx.constants.map(this::visitConstantDef) +
            ctx.operations.map(this::visitOperationDef) +
            ctx.programs.map(this::visitProgramDef)
        )
    }

    // Defs

    override fun visitConstantDef(ctx: ConstantDefContext): ConstantDef {
        return ClassicalType.of(ctx.type.text)?.let { type ->
            val valueExpr = ctx.value.visit()
            try {
                when (type) {
                    ClassicalType.Bool -> ConstantDefBool(
                        ctx.ident.text, valueExpr as BoolExpr, ctx.location
                    )
                    ClassicalType.Int -> ConstantDefInt(
                        ctx.ident.text, valueExpr as IntExpr, ctx.location
                    )
                    ClassicalType.Float -> ConstantDefFloat(
                        ctx.ident.text, valueExpr as FloatExpr, ctx.location
                    )
                    else -> {
                        raiseCompileError(ctx.location) {
                            "Unsupported constant type ${ctx.type.text}"
                        }
                    }
                }
            } catch (castException: ClassCastException) {
                raiseCompileError(ctx.location) {
                    "Type not match, expected $type got ${valueExpr.type}"
                }
            }
        }?.also {
            this.classicalVariableTable.put(it.variable.ident, it.variable)
        } ?: run {
            raiseCompileError(ctx.location) {
                "Invalid type ${ctx.type.text}"
            }
        }
    }

    override fun visitProgramDef(ctx: ProgramDefContext): ProgramDef {
        return doInScope(ScopeType.PROGRAM) {
            visitClassicalParamList(ctx.params).onEach { variable: ClassicalVariable ->
                this.classicalVariableTable.put(variable.ident, variable)
            }.let { paramList: List<ClassicalVariable> ->
                ProgramDef(ctx.ident.text, paramList, ctx.shots.visit(), visitStatementsBlock(ctx.body), ctx.location)
            }
        }
    }

    override fun visitOperationDef(ctx: OperationDefContext): OperationDef {
        return doInScope(ScopeType.OPERATION) {
            val classicalParams = ctx.classicalParams?.let {
                visitClassicalParamList(it)
            } ?: emptyList()
            classicalParams.forEach { this.classicalVariableTable.put(it.ident, it) }
            val quantumParams = visitQuantumParamList(ctx.quantumParams).apply {
                raiseCompileErrorIf(this.isEmpty(), ctx.location) {
                    "Operation must have at least one quantum parameter"
                }
            }
            quantumParams.forEach { this.quantumVariableTable.put(it.ident, it) }
            OperationDef(
                ident = ctx.ident.text,
                classicalParams = classicalParams,
                quantumParams = quantumParams,
                body = visitStatementsBlock(ctx.body),
                location = ctx.location
            ).also {
                this.operations[it.ident] = it
            }
        }
    }

    override fun visitExternFuncDef(ctx: ExternFuncDefContext) {
        try {
            this.functions[ctx.ident.text] = when (ClassicalType.notNullOf(ctx.returnType.text)) {
                ClassicalType.Int -> ExternIntFunction(
                    ctx.ident.text,
                    ctx.paramTypes.map { ClassicalType.notNullOf(it.text) },
                    ctx.location,
                )
                ClassicalType.Float -> ExternFloatFunction(
                    ctx.ident.text,
                    ctx.paramTypes.map { ClassicalType.notNullOf(it.text) },
                    ctx.location,
                )
                else -> raiseCompileError(ctx.location) {
                    "Unsupported return type ${ctx.returnType.text}"
                }
            }
        } catch (exception: IllegalArgumentException) {
            raiseCompileError(ctx.location) {
                exception.message ?: "Invalid external function definition"
            }
        }
    }

    // Slice

    private fun SlicingContext.visit(): Range {
        return when (this) {
            is SlicingInclusiveContext -> visitSlicingInclusive(this)
            is SlicingExclusiveContext -> visitSlicingExclusive(this)
            else -> this.exception.let {
                CompileError.error(this.location, it.message ?: "Invalid Slicing").raise()
            }
        }
    }

    override fun visitSlicingExclusive(ctx: SlicingExclusiveContext): Range {
        return Range(ctx.start?.visit() ?: IntExpr(0), ctx.end?.visit(), ctx.step?.visit() ?: IntExpr(1), false)
    }

    override fun visitSlicingInclusive(ctx: SlicingInclusiveContext): Range {
        return Range(ctx.start?.visit() ?: IntExpr(0), ctx.end?.visit(), ctx.step?.visit() ?: IntExpr(1), true)
    }

    // ParamList

    override fun visitClassicalParamList(ctx: ClassicalParamListContext?): List<ClassicalVariable> {
        return ctx?.params?.map {
            ClassicalType.of(it.type.text)?.createVariable(it.ident.text, it.location) ?: run {
                ctx.exception?.let { exception ->
                    raiseCompileError(ctx.location) {
                        exception.message ?: "Invalid type ${it.type.text}"
                    }
                } ?: run {
                    raiseCompileError(ctx.location) {
                        "Type ${it.type.text} is not supported yet"
                    }
                }
            }
        } ?: emptyList()
    }

    override fun visitQuantumParamList(ctx: QuantumParamListContext): List<QuantumParameter> {
        return ctx.params.map {
            val (size: IntExpr, inference: IntVariable?) = it.size?.let { sizeCtx ->
                when (sizeCtx) {
                    is StaticQvarSizeContext -> Pair(sizeCtx.value.visit(), null)
                    is AutoInferQvarSizeContext -> visitAutoInferQvarSize(sizeCtx)
                    else -> raiseCompileError(ctx.location) {
                        ctx.exception.message ?: "Invalid QvarSize"
                    }
                }
            } ?: Pair(IntExpr(1), null)
            inference?.also { variable -> this.classicalVariableTable.put(variable.ident, variable) }
            return@map it.qvar?.let { qvar ->
                QvarParameter(qvar.ident.text, size, inference, qvar.location)
            } ?: it.qref?.let { qref ->
                QrefParameter(qref.ident.text, size, inference, qref.location)
            } ?: unreachable()
        }
    }

    override fun visitAutoInferQvarSize(ctx: AutoInferQvarSizeContext): Pair<IntExpr, IntVariable> {
        return ctx.sizeExpr.let {
            when (it) {
                is QvarSizeAutoExprVariableContext -> {
                    val variable = IntVariable(it.ident.text)
                    Pair(IntExpr(variable), variable)
                }
                else -> {
                    ctx.exception?.let { exception ->
                        CompileError.error(ctx.location, exception.message ?: "Invalid size auto inference expr").raise()
                    } ?: run {
                        CompileError.error(ctx.location, "Size auto inference expr is not supported yet").raise()
                    }
                }
            }
        }

    }

    // StatementsBlock

    private fun StatementsBlockContext.visit(): StatementsBlock {
        return visitStatementsBlock(this)
    }

    override fun visitStatementsBlock(ctx: StatementsBlockContext): StatementsBlock {
        return doInScope {
            StatementsBlock(ctx.statements.map { it.visit() }, ctx.location)
        }
    }

    // Statements

    private fun StatementContext.visit() = visitStatement(this)

    fun visitStatement(ctx: StatementContext): Statement {
        return when (ctx) {
            is QvarOperateAsStatementContext -> visitQvarOperateAsStatement(ctx)
            is QvarMeasurementStatementContext -> visitQvarMeasurementStatement(ctx)
            is QvarCompreOperateStatementContext -> visitQvarCompreOperateStatement(ctx)
            is CifStatementContext -> visitCifStatement(ctx)
            is QifStatementContext -> visitQifStatement(ctx)
            is ForStatementContext -> visitForStatement(ctx)
            is WithStatementContext -> visitWithStatement(ctx)
            is LetStatementContext -> visitLetStatement(ctx)
            else -> ctx.exception.let {
                CompileError.error(ctx.location, it.message ?: "Invalid Statement").raise()
            }
        }
    }

    override fun visitQvarOperateStatement(ctx: QvarOperateStatementContext): OperationStatement {
        return OperationStatement(ctx.expr.visit(), null, ctx.location)
    }

    override fun visitQvarOperateAsStatement(ctx: QvarOperateAsStatementContext): OperationAsStatement {
        return OperationAsStatement(ctx.expr.visit(), ctx.qvar.ident.text, ctx.location).also {
            raiseCompileErrorIf(ctx.qvar.ident.text in this.quantumVariableTable, ctx.location) {
                "Qvar ${ctx.qvar.ident.text} has been declared"
            }
            this.quantumVariableTable.put(it.variable.ident, it.variable)
        }
    }

    override fun visitQvarMeasurementStatement(ctx: QvarMeasurementStatementContext): MeasurementStatement {
        return ctx.expr.visit().let { expr ->
            ctx.slice?.let {
                QvarExprSlice(expr, it.visit(), ctx.location)
            } ?: expr
        }.let { expr ->
            MeasurementStatement(expr, ctx.location)
        }
    }

    override fun visitQvarCompreOperateStatement(ctx: QvarCompreOperateStatementContext): Statement {
        val ctrl = lazy {
            // `ctrl` must be a lazy variable because `ctx.ctrl` may use
            //  iterator variables declared in `ctx.compre`
            ctx.ctrl?.visit()?.let {
                if (it is QrefAtomicExpr) it else {
                    raiseCompileError(ctx.ctrl.location) {
                        ctx.ctrl.exception.message ?: "Invalid QvarCompreOperateStatement"
                    }
                }
            }
        }
        return if (ctx.compre == null) {
            OperationStatement(ctx.expr.visit(), ctrl.value, ctx.location)
        } else ctx.compre.map { compre ->
            this.classicalVariableTable.enterScope()
            when (val expr = compre.expr.visit()) {
                is BitsExpr -> BitsComprehension(
                    iterator = IntVariable(compre.ident.text, compre.expr.location).also {
                        this.classicalVariableTable.put(it.ident, it)
                    },
                    iterable = expr,
                    location = compre.location,
                )
                is ListExpr<*> -> ListComprehension(
                    iterator = expr.createIteratorVariable(compre.ident.text, compre.expr.location).also {
                        this.classicalVariableTable.put(it.ident, it)
                    },
                    iterable = expr,
                    location = compre.location,
                )
                else -> raiseCompileError(compre.location) {
                    compre.exception.message ?: "Invalid QvarCompreOperateStatement"
                }
            }
        }.let {
            OperationStatementWithComprehension(
                expr = ctx.expr.visit(),
                location = ctx.location,
                ctrl = ctrl.value,
                comprehensions = it,
            )
        }.also {
            for (i in it.comprehensions.indices) {
                this.classicalVariableTable.exitScope()
            }
        }
    }

    override fun visitCifStatement(ctx: CifStatementContext): CifStatement {
        return CifStatement(
            condition = ctx.cond.visit(),
            ifBranch = ctx.ifBranch.visit(),
            elseBranch = ctx.elseBranch?.visit(),
            location = ctx.location,
        )
    }

    override fun visitQifStatement(ctx: QifStatementContext): QifStatement {
        return QifStatement(
            ctrl = ctx.ctrl.visit(),
            ifBranch = ctx.ifBranch.visit(),
            elseBranch = ctx.elseBranch?.visit() ?: StatementsBlock(listOf(), ctx.location),
            location = ctx.location,
        )
    }

    override fun visitForStatement(ctx: ForStatementContext): ForLoopStatement<ClassicalTrait> {
        return doInScope {
            val iterable = ctx.iterable.visit()
            val iterator = iterable.iterableType.createVariable(ctx.iterator.text, ctx.location).also {
                this.classicalVariableTable.put(it.ident, it)
            }
            ForLoopStatement(
                iterator = iterator,
                iterable = iterable,
                loopBody = ctx.loopBody.visit(),
                location = ctx.location
            )
        }
    }

    override fun visitWithStatement(ctx: WithStatementContext): WithStatement {
        return WithStatement(ctx.expr.visit(), ctx.body.visit(), ctx.location)
    }

    override fun visitLetStatement(ctx: LetStatementContext): LetStatement<ClassicalTrait> {
        return when (val expr = ctx.value) {
            is ClassicalExprBoolContext -> BoolVariable(ctx.ident.text).let {
                classicalVariableTable.put(it.ident, it)
                LetStatement(it, expr.boolExpr().visit(), ctx.location)
            }
            is ClassicalExprIntContext -> IntVariable(ctx.ident.text).let {
                classicalVariableTable.put(it.ident, it)
                LetStatement(it, expr.intExpr().visit(), ctx.location)
            }
            is ClassicalExprFloatContext -> BoolVariable(ctx.ident.text).let {
                classicalVariableTable.put(it.ident, it)
                LetStatement(it, expr.floatExpr().visit(), ctx.location)
            }
            is ClassicalExprComplexContext -> BoolVariable(ctx.ident.text).let {
                classicalVariableTable.put(it.ident, it)
                LetStatement(it, expr.complexExpr().visit(), ctx.location)
            }
            is ClassicalExprNumericContext -> {
                when (val numericExpr = expr.numericExpr().visit()) {
                    is IntExpr -> IntVariable(ctx.ident.text).let {
                        classicalVariableTable.put(it.ident, it)
                        LetStatement(it, numericExpr, ctx.location)
                    }
                    is FloatExpr -> FloatVariable(ctx.ident.text).let {
                        classicalVariableTable.put(it.ident, it)
                        LetStatement(it, numericExpr, ctx.location)
                    }
                    else -> unreachable()
                }
            }
            else -> ctx.exception.let {
                raiseCompileError(ctx.location) {
                    it?.message ?: unreachable("Invalid LetStatement")
                }
            }
        }
    }

    // AtomicOperation

    private fun AtomicOperationContext.visit() = visitAtomicOperation(this)

    override fun visitAtomicOperation(ctx: AtomicOperationContext): OperationExprElementary {
        val classicalArgs = ctx.argList?.args?.map(::visitClassicalExpr) ?: emptyList()
        return StandardGate.tryFrom(ctx.ident.text)?.let {
            OperationExprStandard(it, classicalArgs, ctx.location)
        } ?: run {
            this.operations[ctx.ident.text]?.let {
                OperationExprUserDefined(it, classicalArgs, ctx.location)
            } ?: run {
                CompileError.error(ctx.location,
                    "Undeclared operation ${ctx.ident.text}"
                ).raise()
            }
        }
    }

    // OperationExpr

    private fun OperationExprContext.visit(): OperationExpr = visitOperationExpr(this)

    fun visitOperationExpr(ctx: OperationExprContext): OperationExpr = when(ctx){
        is OperationExprElementaryContext -> visitOperationExprElementary(ctx)
        is OperationExprMatMulContext -> visitOperationExprMatMul(ctx)
        is OperationExprDaggerContext -> visitOperationExprDagger(ctx)
        is OperationExprExtendedContext -> visitOperationExprExtended(ctx)
        is OperationExprCoefficientContext -> visitOperationExprCoefficient(ctx)
        is OperationExprCombinedContext -> visitOperationExprCombined(ctx)
        is OperationExprAnonymousContext -> visitOperationExprAnonymous(ctx)
        else -> ctx.exception.let {
            CompileError.error(ctx.location, it.message ?: "Invalid OpertionExpr").raise()
        }
    }

    override fun visitOperationExprElementary(ctx: OperationExprElementaryContext): OperationExprElementary {
        return ctx.op.visit()
    }

    override fun visitOperationExprMatMul(ctx: OperationExprMatMulContext): OperationExprSequentialMatMul {
        return OperationExprSequentialMatMul(
            ctx.operations.map { it.visit() },
            ctx.location,
        )
    }

    override fun visitOperationExprCoefficient(ctx: OperationExprCoefficientContext): OperationExprCoefficient {
        return OperationExprCoefficient(ctx.op.visit(), ctx.coefficient.visit(), ctx.location)
    }

    override fun visitOperationExprAnonymous(ctx: OperationExprAnonymousContext): OperationExpr {
        TODO()
    }

    override fun visitOperationExprDagger(ctx: OperationExprDaggerContext): OperationExprDagger {
        return OperationExprDagger(ctx.op.visit(), ctx.location)
    }

    override fun visitOperationExprCombined(ctx: OperationExprCombinedContext): OperationExprCombined {
        return OperationExprCombined(ctx.operations.map { it.visit() }, ctx.location)
    }

    override fun visitOperationExprExtended(ctx: OperationExprExtendedContext): OperationExprExtended {
        return OperationExprExtended(ctx.op.visit(), ctx.exponent.visit(), ctx.location)
    }

    // QvarExpr

    private fun QvarExprContext.visit(): QvarExpr = visitQvarExpr(this)

    fun visitQvarExpr(ctx: QvarExprContext): QvarExpr = when(ctx){
        is QvarExprDefaultContext -> visitQvarExprDefault(ctx)
        is QvarExprIdentContext -> visitQvarExprIdent(ctx)
        is QvarExprInitContext -> visitQvarExprInit(ctx)
        is QvarExprIndexingContext -> visitQvarExprIndexing(ctx)
        is QvarExprMultiIndexingContext -> visitQvarExprMultiIndexing(ctx)
        is QvarExprSlicingContext -> visitQvarExprSlicing(ctx)
        is QvarExprQuotedContext -> visitQvarExprQuoted(ctx)
        is QvarExprConcateContext -> visitQvarExprConcate(ctx)
        is QvarExprMultiTargetsOperationContext -> visitQvarExprMultiTargetsOperation(ctx)
        is QvarExprOperationContext -> visitQvarExprOperation(ctx)
        is QvarExprOperationReversedContext -> visitQvarExprOperationReversed(ctx)
        else -> ctx.exception.let {
            CompileError.error(ctx.location, it.message ?: "Invalid QvarExpr").raise()
        }
    }

    override fun visitQvarExprDefault(ctx: QvarExprDefaultContext): QvarExprVariable {
        TODO()
    }

    override fun visitQvarExprIdent(ctx: QvarExprIdentContext): QvarExprVariable {
        return quantumVariableTable[ctx.qvar.ident.text]?.let {
            if (it is QvarVariable) {
                QvarExprVariable(it, ctx.location)
            } else {
                CompileError.error(ctx.location,
                    "Expected `Qvar`, found `Qref`: $it"
                ).raise()
            }
        } ?: raiseCompileError(ctx.location) {
            "Unknown `Qvar` variable: ${ctx.qvar.text}"
        }
    }

    override fun visitQvarExprInit(ctx: QvarExprInitContext): QvarExprInit {
        return when (val init = ctx.init) {
            is QvarInitBinaryLiteralContext -> {
                val literal = init.literal.text
                val size = literal.length.toUInt()
                try {
                    QvarExprInit.encode(literal.toUInt(2), size, init.location)
                } catch (_: IllegalArgumentException) {
                    raiseCompileError(init.location) {
                        "Invalid binary literal: $literal"
                    }
                }
            }
            is QvarInitClassicalExprContext -> {
                try {
                    val size = init.size.text.toUInt()
                    val value = init.value.text.toUInt()
                    QvarExprInit.encode(value, size, init.location)
                } catch (_: IllegalArgumentException) {
                    val size = init.size.visit()
                    val value = init.value.visit()
                    QvarExprInit.encode(value, size, init.location)
                }
            }
            else -> ctx.exception.let {
                CompileError.error(ctx.location, it.message ?: "Invalid QvarExprInit").raise()
            }
        }
    }

    override fun visitQvarExprQuoted(ctx: QvarExprQuotedContext): QvarExpr {
        return ctx.expr.visit()
    }

    override fun visitQvarExprConcate(ctx: QvarExprConcateContext): QvarExprConcat {
        return QvarExprConcat(ctx.qvarExprs.map { it.visit() }, ctx.location)
    }

    override fun visitQvarExprMultiTargetsOperation(
        ctx: QvarExprMultiTargetsOperationContext
    ): QvarExprOperationMultiTargets {
        return QvarExprOperationMultiTargets(
            operation = ctx.op.visit(),
            targets = ctx.targets.exprs.map { it.visit() },
            location = ctx.location
        )
    }

    override fun visitQvarExprOperation(ctx: QvarExprOperationContext): QvarExprOperation {
        return QvarExprOperation(ctx.op.visit(), ctx.target.visit(), ctx.location)
    }

    override fun visitQvarExprOperationReversed(ctx: QvarExprOperationReversedContext): QvarExprOperation {
        return QvarExprOperation(ctx.op.visit(), ctx.target.visit(), ctx.location)
    }

    override fun visitQvarExprIndexing(ctx: QvarExprIndexingContext): QvarExprIndexing {
        return QvarExprIndexing(ctx.expr.visit(), ctx.index.visit(), ctx.location)
    }

    override fun visitQvarExprMultiIndexing(ctx: QvarExprMultiIndexingContext): QvarExprConcat {
        val inner = ctx.expr.visit()
        return ctx.indexes.map { it.visit() }.let { indexes ->
            QvarExprConcat(indexes.map { QvarExprIndexing(inner, it, ctx.location) }, ctx.location)
        }
    }

    override fun visitQvarExprSlicing(ctx: QvarExprSlicingContext): QvarExprSlice {
        return ctx.slice.visit().let {
            ctx.expr.visit().slice(it.start, it.end, it.step, it.inclusive, ctx.location)
        }
    }

    // QrefExpr

    private fun QrefExprContext.visit() = visitQrefExpr(this)

    fun visitQrefExpr(ctx: QrefExprContext): QrefExpr {
        return when (ctx) {
            is QrefExprIdentContext -> visitQrefExprIdent(ctx)
            is QrefExprIndexingContext -> visitQrefExprIndexing(ctx)
            is QrefExprSlicingContext -> visitQrefExprSlicing(ctx)
            is QrefExprQuotedContext -> visitQrefExprQuoted(ctx)
            is QrefExprAndContext -> visitQrefExprAnd(ctx)
            is QrefExprOrContext -> visitQrefExprOr(ctx)
            is QrefExprNotContext -> visitQrefExprNot(ctx)
            else -> ctx.exception.let {
                CompileError.error(ctx.location, it.message ?: "Invalid QrefExpr").raise()
            }
        }
    }

    override fun visitQrefExprQuoted(ctx: QrefExprQuotedContext): QrefExpr {
        return ctx.expr.visit()
    }

    private fun getQrefVariable(ident: String, location: Location): QrefVariable {
        return quantumVariableTable[ident]?.let { it as QrefVariable } ?: run {
            CompileError.error(location, "Undeclared qref $ident").raise()
        }
    }

    private fun getQrefExprVariable(ident: String, location: Location): QrefExprVariable {
        return QrefExprVariable(getQrefVariable(ident, location), location)
    }

    override fun visitQrefExprIdent(ctx: QrefExprIdentContext): QrefExprVariable {
        return QrefExprVariable(
            variable = getQrefVariable(ctx.qref.ident.text, ctx.location),
            location = ctx.location,
        )
    }

    override fun visitQrefExprNot(ctx: QrefExprNotContext): QrefExprNot {
        return QrefExprNot(ctx.expr.visit(), ctx.location)
    }

    override fun visitQrefExprAnd(ctx: QrefExprAndContext): QrefExprAnd {
        return QrefExprAnd(ctx.lhs.visit(), ctx.rhs.visit(), ctx.location)
    }

    override fun visitQrefExprOr(ctx: QrefExprOrContext): QrefExprOr {
        return QrefExprOr(ctx.lhs.visit(), ctx.rhs.visit(), ctx.location)
    }

    override fun visitQrefExprIndexing(ctx: QrefExprIndexingContext): QrefExprIndexing {
        return QrefExprIndexing(
            variable = getQrefExprVariable(ctx.qref.ident.text, ctx.location),
            index = ctx.index.visit(),
            location = ctx.location,
        )
    }

    override fun visitQrefExprSlicing(ctx: QrefExprSlicingContext): QrefExprSlice {
        return ctx.slice.visit().let { range ->
            getQrefExprVariable(ctx.qref.ident.text, ctx.location).slice(
                range.start, range.end, range.step, range.inclusive, ctx.location
            )
        }
    }

    /* TODO: multi indexing
    override fun visitQrefExprMultiIndexing(ctx: QrefExprMultiIndexingContext): QrefExprConcat {
        val inner = getQrefExprVariable(ctx.qref.ident.text, ctx.location)
        return ctx.indexes.map { it.visit() }.let { indexes ->
            QrefExprConcat(indexes.map { QrefExprIndexing(inner, it, ctx.location) }, ctx.location)
        }
    }
    */

    // IterableExpr

    private fun IterableExprContext.visit() = visitIterableExpr(this)

    fun visitIterableExpr(ctx: IterableExprContext): IterableExpr<ClassicalTrait> {
        val loc = ctx.location
        return when (ctx) {
            is IterableExprListContext -> visitIterableExprList(ctx)
            is IterableExprBitsContext -> visitIterableExprBits(ctx)
            is IterableExprUndeterminedVariableContext -> {
                getVariable<ClassicalVariable>(ctx.ident.text, loc).toClassicalExpr(loc).also {
                    raiseCompileErrorIf(it !is IterableExpr<ClassicalTrait>, loc) {
                        "Variable ${ctx.ident.text} is not iterable"
                    }
                } as IterableExpr<ClassicalTrait>
            }
            else -> raiseCompileError(loc) {
                ctx.exception.message ?: "Invalid IterableExpr"
            }
        }
    }

    override fun visitIterableExprList(ctx: IterableExprListContext): IterableExpr<ClassicalTrait> {
        return ctx.list.visit()
    }

    override fun visitIterableExprBits(ctx: IterableExprBitsContext): IterableExpr<IntTrait> {
        return ctx.bits.visit()
    }

    override fun visitClassicalIntListGenerator(ctx: ClassicalIntListGeneratorContext): IntListGenerator {
        return IntListGenerator(
            start = ctx.start?.visit() ?: IntExpr(0),
            end = ctx.end.visit(),
            inclusive = ctx.inclusive != null,
            step = ctx.step?.visit() ?: IntExpr(1),
            location = ctx.location,
        )
    }

    // ClassicalExpr

    private fun ClassicalExprContext.visit() = visitClassicalExpr(this)

    fun visitClassicalExpr(ctx: ClassicalExprContext): ClassicalExpr {
        return when (ctx) {
            is ClassicalExprUndeterminedVariableContext -> {
                getVariable<ClassicalVariable>(ctx.ident.text, ctx.location).toClassicalExpr(ctx.location)
            }
            is ClassicalExprBoolContext -> visitClassicalExprBool(ctx)
            is ClassicalExprIntContext -> visitClassicalExprInt(ctx)
            is ClassicalExprFloatContext -> visitClassicalExprFloat(ctx)
            is ClassicalExprNumericContext -> visitNumericExpr(ctx.numericExpr())
            is ClassicalExprComplexContext -> visitClassicalExprComplex(ctx)
            is ClassicalExprListContext -> visitClassicalExprList(ctx)
            else -> ctx.exception.let {
                CompileError.error(ctx.location, it.message ?: "Invalid ClassicalExpr").raise()
            }
        }
    }

    // BoolExpr

    private fun BoolExprContext.visit(): BoolExpr = visitBoolExpr(this)

    override fun visitClassicalExprBool(ctx: ClassicalExprBoolContext) = ctx.boolExpr().visit()

    fun visitBoolExpr(ctx: BoolExprContext): BoolExpr = when(ctx){
        is BoolExprIdentContext -> visitBoolExprIdent(ctx)
        is BoolExprLiteralContext -> visitBoolExprLiteral(ctx)
        is BoolExprQuotedContext -> visitBoolExprQuoted(ctx)
        is BoolExprNotContext -> visitBoolExprNot(ctx)
        is BoolExprAndContext -> visitBoolExprAnd(ctx)
        is BoolExprOrContext -> visitBoolExprOr(ctx)
        is BoolExprIntComparisonContext -> visitBoolExprIntComparison(ctx)
        else -> ctx.exception.let {
            CompileError.error(ctx.location, it.message ?: "Invalid BoolExpr").raise()
        }
    }

    override fun visitBoolExprIdent(ctx: BoolExprIdentContext): BoolExprVariable {
        return BoolExprVariable(
            this.getVariable(ctx.ident.text, ctx.location),
            ctx.location
        )
    }

    override fun visitBoolExprLiteral(ctx: BoolExprLiteralContext): BoolExprLiteral {
        return when (ctx.value.text) {
            "true" -> BoolExprLiteralTrue
            "false" -> BoolExprLiteralFalse
            else -> ctx.exception.let {
                CompileError.error(ctx.location, it.message ?: "Invalid BoolExprLiteral").raise()
            }
        }
    }

    override fun visitBoolExprQuoted(ctx: BoolExprQuotedContext): BoolExpr {
        return ctx.expr.visit()
    }

    override fun visitBoolExprNot(ctx: BoolExprNotContext): BoolExprNot {
        return !ctx.expr.visit()
    }

    override fun visitBoolExprOr(ctx: BoolExprOrContext): BoolExprBinary {
        return ctx.lhs.visit() or ctx.rhs.visit()
    }

    override fun visitBoolExprAnd(ctx: BoolExprAndContext): BoolExprBinary {
        return ctx.lhs.visit() and ctx.rhs.visit()
    }

    override fun visitBoolExprIntComparison(ctx: BoolExprIntComparisonContext): BoolExpr {
        return when (ctx.opt.text) {
            "==" -> ctx.lhs.visit() equalTo ctx.rhs.visit()
            "!=" -> ctx.lhs.visit() notEqualTo ctx.rhs.visit()
            "<" -> ctx.lhs.visit() lessThan ctx.rhs.visit()
            "<=" -> ctx.lhs.visit() lessEqualTo ctx.rhs.visit()
            ">" -> ctx.lhs.visit() greaterThan ctx.rhs.visit()
            ">=" -> ctx.lhs.visit() greaterEqualTo ctx.rhs.visit()
            else -> ctx.exception.let {
                CompileError.error(ctx.location, it.message ?: "Invalid BoolExprIntComparison").raise()
            }
        }
    }

    // NumericExpr

    private fun NumericExprContext.visit(): NumericExpr = visitNumericExpr(this)

    fun visitNumericExpr(ctx: NumericExprContext): NumericExpr = when (ctx) {
        is NumericExprIdentContext -> visitNumericExprIdent(ctx)
        is NumericExprFloatLiteralContext -> visitNumericExprFloatLiteral(ctx)
        is NumericExprIntLiteralContext -> visitNumericExprIntLiteral(ctx)
        is NumericExprQuotedContext -> visitNumericExprQuoted(ctx)
        is NumericExprNegativeContext -> visitNumericExprNegative(ctx)
        is NumericExprBitwiseContext -> visitNumericExprBitwise(ctx)
        is NumericExprShiftContext -> visitNumericExprShift(ctx)
        is NumericExprPowContext -> visitNumericExprPow(ctx)
        is NumericExprMulDivModContext -> visitNumericExprMulDivMod(ctx)
        is NumericExprAddSubContext -> visitNumericExprAddSub(ctx)
        is NumericExprFuncCallContext -> visitNumericExprFuncCall(ctx)
        else -> ctx.exception.let {
            CompileError.error(ctx.location, it.message ?: "Invalid NumericExpr").raise()
        }
    }

    override fun visitNumericExprIdent(ctx: NumericExprIdentContext): NumericExpr {
        return when (val variable = getVariable<ClassicalVariable>(ctx.ident.text, ctx.location)) {
            is IntVariable -> variable.toClassicalExpr(ctx.location)
            is FloatVariable -> variable.toClassicalExpr(ctx.location)
            else -> raiseCompileError(ctx.location) {
                "Variable ${ctx.ident.text} is not a numeric variable"
            }
        }
    }

    override fun visitNumericExprFloatLiteral(ctx: NumericExprFloatLiteralContext): FloatExpr {
        return FloatExprLiteral(ctx.value.text.toDouble(), ctx.location)
    }

    override fun visitNumericExprIntLiteral(ctx: NumericExprIntLiteralContext): IntExpr {
        return IntExpr(ctx.value.text.toInt(), ctx.location)
    }

    override fun visitNumericExprQuoted(ctx: NumericExprQuotedContext): NumericExpr {
        return ctx.expr.visit()
    }

    override fun visitNumericExprNegative(ctx: NumericExprNegativeContext): NumericExpr {
        return when (val expr = ctx.expr.visit()) {
            is IntExpr -> -expr
            is FloatExpr -> -expr
            else -> unreachable()
        }
    }

    override fun visitNumericExprBitwise(ctx: NumericExprBitwiseContext): IntExpr {
        val lhs = ctx.lhs.visit()
        val rhs = ctx.rhs.visit()
        if (lhs is IntExpr && rhs is IntExpr) {
            return when (ctx.opt.text) {
                "&" -> lhs and rhs
                "|" -> lhs or rhs
                "^" -> lhs xor rhs
                else -> unreachable()
            }
        } else {
            raiseCompileError(ctx.location) {
                "Bitwise operations are only applied to integer expressions"
            }
        }
    }

    override fun visitNumericExprShift(ctx: NumericExprShiftContext): IntExpr {
        val lhs = ctx.lhs.visit()
        val rhs = ctx.rhs.visit()
        if (lhs is IntExpr && rhs is IntExpr) {
            return when (ctx.opt.text) {
                "<<" -> lhs shl rhs
                ">>" -> lhs shr rhs
                ">>>" -> lhs lshr rhs
                else -> unreachable()
            }
        } else {
            raiseCompileError(ctx.location) {
                "Shift operations are only applied to integer expressions"
            }
        }
    }

    override fun visitNumericExprPow(ctx: NumericExprPowContext): NumericExpr {
        return ctx.base.visit() pow ctx.exponent.visit()
    }

    override fun visitNumericExprAddSub(ctx: NumericExprAddSubContext): NumericExpr {
        return when (ctx.opt.text) {
            "+" -> ctx.lhs.visit() + ctx.rhs.visit()
            "-" -> ctx.lhs.visit() - ctx.rhs.visit()
            else -> unreachable()
        }
    }

    override fun visitNumericExprMulDivMod(ctx: NumericExprMulDivModContext): NumericExpr {
        return when (ctx.opt.text) {
            "*" -> ctx.lhs.visit() * ctx.rhs.visit()
            "/" -> ctx.lhs.visit() / ctx.rhs.visit()
            "%" -> try {
                ctx.lhs.visit() as IntExpr % ctx.rhs.visit() as IntExpr
            } catch (exception: ClassCastException) {
                raiseCompileError(ctx.location) {
                    "Modulo operation is only applied to integer expressions"
                }
            }
            else -> unreachable()
        }
    }

    override fun visitNumericExprFuncCall(ctx: NumericExprFuncCallContext): NumericExpr {
        return functions[ctx.func.text]?.let { function ->
            @Suppress("UNCHECKED_CAST") try {
                function.call(ctx.argList.args.map { it.visit() }, ctx.location) as NumericExpr
            } catch (exception: ClassCastException) {
                raiseCompileError(ctx.location) {
                    "The return type of function ${ctx.func.text} is not a numeric type"
                }
            }
        } ?: run {
            raiseCompileError(ctx.location) {
                "Function ${ctx.func.text} is not defined"
            }
        }
    }

    // IntExpr

    private fun IntExprContext.visit(): IntExpr = visitIntExpr(this)

    override fun visitClassicalExprInt(ctx: ClassicalExprIntContext) = ctx.intExpr().visit()

    fun visitIntExpr(ctx: IntExprContext): IntExpr = when (ctx) {
        is IntExprIdentContext -> visitIntExprIdent(ctx)
        is IntExprLiteralContext -> visitIntExprLiteral(ctx)
        is IntExprQuotedContext -> visitIntExprQuoted(ctx)
        is IntExprNegativeContext -> visitIntExprNegative(ctx)
        is IntExprBitwiseContext -> visitIntExprBitwise(ctx)
        is IntExprShiftContext -> visitIntExprShift(ctx)
        is IntExprPowContext -> visitIntExprPow(ctx)
        is IntExprMulDivModContext -> visitIntExprMulDivMod(ctx)
        is IntExprAddSubContext -> visitIntExprAddSub(ctx)
        is IntExprFuncCallContext -> visitIntExprFuncCall(ctx)
        else -> ctx.exception.let {
            CompileError.error(ctx.location, it.message ?: "Invalid IntExpr").raise()
        }
    }

    override fun visitIntExprLiteral(ctx: IntExprLiteralContext): IntExpr {
        val literalText = ctx.value.text
        return if (literalText.startsWith("0x")) {
            IntExpr(literalText.substring(2).toLong(16).toInt(), ctx.location)
        } else if (literalText.startsWith("0b")) {
            IntExpr(literalText.substring(2).toLong(2).toInt(), ctx.location)
        } else {
            IntExpr(literalText.toInt(), ctx.location)
        }
    }

    override fun visitIntExprIdent(ctx: IntExprIdentContext): IntExpr {
        return IntExpr(
            this.getVariable<IntVariable>(ctx.ident.text, ctx.location),
            ctx.location
        )
    }

    override fun visitIntExprQuoted(ctx: IntExprQuotedContext): IntExpr {
        return ctx.expr.visit()
    }

    override fun visitIntExprNegative(ctx: IntExprNegativeContext): IntExpr {
        return -ctx.expr.visit()
    }

    override fun visitIntExprAddSub(ctx: IntExprAddSubContext): IntExpr {
        return when (ctx.opt.text) {
            "+" -> ctx.lhs.visit() + ctx.rhs.visit()
            "-" -> ctx.lhs.visit() - ctx.rhs.visit()
            else -> ctx.exception.let {
                CompileError.error(ctx.location, it.message ?: "Invalid IntExpr").raise()
            }
        }
    }

    override fun visitIntExprMulDivMod(ctx: IntExprMulDivModContext): IntExpr {
        return when (ctx.opt.text) {
            "*" -> ctx.lhs.visit() * ctx.rhs.visit()
            "/" -> ctx.lhs.visit() / ctx.rhs.visit()
            "%" -> ctx.lhs.visit() % ctx.rhs.visit()
            else -> ctx.exception.let {
                CompileError.error(ctx.location, it.message ?: "Invalid IntExpr").raise()
            }
        }
    }

    override fun visitIntExprShift(ctx: IntExprShiftContext): IntExpr {
        return when (ctx.opt.text) {
            "<<" -> ctx.lhs.visit() shl ctx.rhs.visit()
            ">>" -> ctx.lhs.visit() shr ctx.rhs.visit()
            ">>>" -> ctx.lhs.visit() lshr ctx.rhs.visit()
            else -> ctx.exception.let {
                CompileError.error(ctx.location, it.message ?: "Invalid IntExpr").raise()
            }
        }
    }

    override fun visitIntExprPow(ctx: IntExprPowContext): IntExpr {
        val base = ctx.base.visit()
        return try {
            base.pow(ctx.exponent.text.toUInt())
        } catch (exception: NumberFormatException) {
            IntExpr(IndeterminateLikePower(base, ctx.exponent.visit()), ctx.location)
        } catch (exception: ArithmeticException) {
            raiseCompileError(ctx.location) {
                "Exponent too large: ${ctx.exponent.text}"
            }
        }
    }

    override fun visitIntExprBitwise(ctx: IntExprBitwiseContext): IntExpr {
        return when (ctx.opt.text) {
            "&" -> ctx.lhs.visit() and ctx.rhs.visit()
            "|" -> ctx.lhs.visit() or ctx.rhs.visit()
            "^" -> ctx.lhs.visit() xor ctx.rhs.visit()
            else -> ctx.exception.let {
                CompileError.error(ctx.location, it.message ?: "Invalid IntExpr").raise()
            }
        }
    }

    override fun visitIntExprFuncCall(ctx: IntExprFuncCallContext): IntExpr {
        return this.callDeclaredFunction(
            ident = ctx.func.text,
            args = ctx.argList.args.map { it.visit() },
            location = ctx.location
        )
    }

    // FloatExpr

    private fun FloatExprContext.visit(): FloatExpr = visitFloatExpr(this)

    override fun visitClassicalExprFloat(ctx: ClassicalExprFloatContext) = ctx.floatExpr().visit()

    fun visitFloatExpr(ctx: FloatExprContext): FloatExpr = when (ctx) {
        is FloatExprIdentContext -> visitFloatExprIdent(ctx)
        is FloatExprLiteralContext -> visitFloatExprLiteral(ctx)
        is FloatExprQuotedContext -> visitFloatExprQuoted(ctx)
        is FloatExprPowContext -> visitFloatExprPow(ctx)
        is FloatExprMulDivContext -> visitFloatExprMulDiv(ctx)
        is FloatExprAddSubContext -> visitFloatExprAddSub(ctx)
        is FloatExprIntToFloatContext -> visitFloatExprIntToFloat(ctx)
        is FloatExprFuncCallContext -> visitFloatExprFuncCall(ctx)
        else -> ctx.exception.let {
            CompileError.error(ctx.location, it.message ?: "Invalid FloatExpr").raise()
        }
    }

    override fun visitFloatExprLiteral(ctx: FloatExprLiteralContext): FloatExpr {
        return FloatExprLiteral(ctx.value.text.toDouble(), ctx.location)
    }

    override fun visitFloatExprIdent(ctx: FloatExprIdentContext): FloatExpr {
        return FloatExprVariable(
            this.getVariable(ctx.ident.text, ctx.location),
            ctx.location
        )
    }

    override fun visitFloatExprQuoted(ctx: FloatExprQuotedContext): FloatExpr {
        return ctx.expr.visit()
    }

    override fun visitFloatExprAddSub(ctx: FloatExprAddSubContext): FloatExpr {
        return when (ctx.opt.text) {
            "+" -> ctx.lhs.visit() + (ctx.rhsFloat?.visit() ?: ctx.rhsInt!!.visit())
            "-" -> ctx.lhs.visit() - (ctx.rhsFloat?.visit() ?: ctx.rhsInt!!.visit())
            else -> ctx.exception.let {
                CompileError.error(ctx.location, it.message ?: "Invalid FloatExpr").raise()
            }
        }
    }

    override fun visitFloatExprMulDiv(ctx: FloatExprMulDivContext): FloatExpr {
        return when (ctx.opt.text) {
            "*" -> ctx.lhs.visit() * (ctx.rhsFloat?.visit() ?: ctx.rhsInt!!.visit())
            "/" -> ctx.lhs.visit() / (ctx.rhsFloat?.visit() ?: ctx.rhsInt!!.visit())
            else -> ctx.exception.let {
                CompileError.error(ctx.location, it.message ?: "Invalid FloatExpr").raise()
            }
        }
    }

    override fun visitFloatExprPow(ctx: FloatExprPowContext): FloatExpr {
        return ctx.base.visit() pow (ctx.expFloat?.visit() ?: ctx.expInt!!.visit())
    }

    override fun visitFloatExprIntToFloat(ctx: FloatExprIntToFloatContext): FloatExpr {
        return ctx.expr.visit().toFloatExpr()
    }

    override fun visitFloatExprFuncCall(ctx: FloatExprFuncCallContext): FloatExprFuncCall {
        return this.callDeclaredFunction(
            ident = ctx.func.text,
            args = ctx.argList.args.map { it.visit() },
            location = ctx.location
        )
    }

    // ComplexExpr

    private fun ComplexExprContext.visit(): ComplexExpr = visitComplexExpr(this)

    override fun visitClassicalExprComplex(ctx: ClassicalExprComplexContext) = ctx.complexExpr().visit()

    fun visitComplexExpr(ctx: ComplexExprContext): ComplexExpr = when (ctx){
        is ComplexExprImLiteralContext -> visitComplexExprImLiteral(ctx)
        is ComplexExprIntImContext -> visitComplexExprIntIm(ctx)
        is ComplexExprFloatImContext -> visitComplexExprFloatIm(ctx)
        is ComplexExprIntReContext -> visitComplexExprIntRe(ctx)
        is ComplexExprFloatReContext -> visitComplexExprFloatRe(ctx)
        is ComplexExprQuotedContext -> visitComplexExprQuoted(ctx)
        is ComplexExprPowerContext -> visitComplexExprPower(ctx)
        is ComplexExprMulDivContext -> visitComplexExprMulDiv(ctx)
        is ComplexExprAddSubContext -> visitComplexExprAddSub(ctx)
        is ComplexExprFuncCallContext -> visitComplexExprFuncCall(ctx)
        else -> ctx.exception.let {
            CompileError.error(ctx.location, it.message ?: "Invalid ComplexExpr").raise()
        }
    }

    override fun visitComplexExprQuoted(ctx: ComplexExprQuotedContext): ComplexExpr {
        return ctx.expr.visit()
    }

    override fun visitComplexExprImLiteral(ctx: ComplexExprImLiteralContext): ComplexExprLiteral {
        return ComplexExprLiteral(
            real = FloatExprLiteral(0.0),
            imaginary = FloatExprLiteral(ctx.im.text.trim('i').toDouble()),
            location = ctx.location,
        )
    }

    override fun visitComplexExprIntRe(ctx: ComplexExprIntReContext): ComplexExprLiteral {
        return ComplexExprLiteral(
            real = FloatExprLiteral(ctx.intRe.text.toDouble()),
            imaginary = FloatExprLiteral(0.0),
            location = ctx.location,
        )
    }

    override fun visitComplexExprIntIm(ctx: ComplexExprIntImContext): ComplexExprLiteral {
        return ComplexExprLiteral(
            real = FloatExprLiteral(0.0),
            imaginary = FloatExprLiteral(ctx.intIm.text.toDouble()),
            location = ctx.location,
        )
    }

    override fun visitComplexExprFloatRe(ctx: ComplexExprFloatReContext): ComplexExprLiteral {
        return ComplexExprLiteral(
            real = FloatExprLiteral(ctx.floatRe.text.toDouble()),
            imaginary = FloatExprLiteral(0.0),
            location = ctx.location,
        )
    }

    override fun visitComplexExprFloatIm(ctx: ComplexExprFloatImContext): ComplexExprLiteral {
        return ComplexExprLiteral(
            real = FloatExprLiteral(0.0),
            imaginary = FloatExprLiteral(ctx.floatIm.text.toDouble()),
            location = ctx.location,
        )
    }

    override fun visitComplexExprPower(ctx: ComplexExprPowerContext): ComplexExprBinary {
        return ctx.lhs.visit() pow ctx.rhs.visit()
    }

    override fun visitComplexExprAddSub(ctx: ComplexExprAddSubContext): ComplexExprBinary {
        return when (ctx.op.text) {
            "+" -> ctx.lhs.visit() + ctx.rhs.visit()
            "-" -> ctx.lhs.visit() - ctx.rhs.visit()
            else -> ctx.exception.let {
                CompileError.error(ctx.location, it.message ?: "Invalid ComplexExprBinary").raise()
            }
        }
    }

    override fun visitComplexExprMulDiv(ctx: ComplexExprMulDivContext): ComplexExprBinary {
        return when (ctx.op.text) {
            "*" -> ctx.lhs.visit() * ctx.rhs.visit()
            "/" -> ctx.lhs.visit() / ctx.rhs.visit()
            else -> ctx.exception.let {
                CompileError.error(ctx.location, it.message ?: "Invalid ComplexExprBinary").raise()
            }
        }
    }

    override fun visitComplexExprFuncCall(ctx: ComplexExprFuncCallContext): ComplexExprFuncCall {
        return this.callDeclaredFunction(
            ident = ctx.func.text,
            args = ctx.argList.args.map { it.visit() },
            location = ctx.location
        )
    }

    // BitsExpr

    private fun BitsExprContext.visit() = visitBitsExpr(this)

    fun visitBitsExpr(ctx: BitsExprContext): BitsExpr {
        return when (ctx) {
            is BitsExprIdentContext -> visitBitsExprIdent(ctx)
            is BitsExprConcateContext -> visitBitsExprConcate(ctx)
            is BitsExprBitwiseContext -> visitBitsExprBitwise(ctx)
            is BitsExprFuncCallContext -> visitBitsExprFuncCall(ctx)
            else -> raiseCompileError(ctx.location) {
                ctx.exception.message ?: "Invalid BitsExpr"
            }
        }
    }

    override fun visitBitsExprIdent(ctx: BitsExprIdentContext): BitsExprVariable {
        return BitsExprVariable(getVariable(ctx.ident.text, ctx.location), ctx.location)
    }

    override fun visitBitsExprConcate(ctx: BitsExprConcateContext): BitsExprBinary {
        return BitsExprBinary(
            operator = BitsExprBinary.Operator.Concat,
            lhs = ctx.lhs.visit(),
            rhs = ctx.rhs.visit(),
            location = ctx.location
        )
    }

    override fun visitBitsExprBitwise(ctx: BitsExprBitwiseContext): BitsExprBinary {
        return BitsExprBinary(
            operator = when (ctx.opt.text) {
                "&" -> BitsExprBinary.Operator.And
                "|" -> BitsExprBinary.Operator.Or
                "^" -> BitsExprBinary.Operator.Xor
                else -> unreachable()
            },
            lhs = ctx.lhs.visit(),
            rhs = ctx.rhs.visit(),
            location = ctx.location,
        )
    }

    override fun visitBitsExprFuncCall(ctx: BitsExprFuncCallContext): BitsExpr {
        TODO()
    }

    // ListExpr

    private fun ListExprContext.visit() = visitListExpr(this)

    override fun visitClassicalExprList(ctx: ClassicalExprListContext) = ctx.listExpr().visit()

    fun visitListExpr(ctx: ListExprContext): ListExpr<ClassicalTrait> {
        return when (ctx) {
            is ListExprIdentContext -> visitListExprIdent(ctx)
            is ListExprLiteralContext -> visitListExprLiteral(ctx)
            is ListExprIntGeneratorContext -> visitListExprIntGenerator(ctx)
            else -> raiseCompileError(ctx.location) {
                ctx.exception.message ?: "Invalid BitsExpr"
            }
        }
    }

    override fun visitListExprIdent(ctx: ListExprIdentContext): ListExpr<ClassicalTrait> {
        getVariable<ListVariable>(ctx.ident.text, ctx.location).run {
            return ListExprVariable(this, this.elementType, ctx.location)
        }
    }

    override fun visitListExprLiteral(ctx: ListExprLiteralContext): ListExprLiteral<ClassicalTrait> {
        val elements = ctx.elements.map { element ->
            try { element.visit() } catch (exception: Exception) {
                raiseCompileError(ctx.location) {
                    "Invalid list element" + (exception.message?.let { ": $it" } ?: "")
                }
            }
        }
        val elementType: ClassicalType = ctx.typeIdent?.let {
            ClassicalType.of(ctx.typeIdent!!.text) ?: run {
                raiseCompileError(ctx.location) {
                    "Invalid list element type ${ctx.typeIdent!!.text}"
                }
            }
        } ?: run {
            raiseCompileErrorIf(elements.isEmpty(), ctx.location) {
                "Unable to infer element type of an empty list, please identify the type explicitly"
            }
            elements[0].type.also { type ->
                elements.forEach {
                    raiseCompileErrorIf(it.type != type, ctx.location) {
                        "Elements type not match in the same list"
                    }
                }
            }
        }

        return when (elementType) {
            ClassicalType.Bool -> ListExprLiteral(elements.map { it as BoolExpr }, ctx.location)
            ClassicalType.Int -> ListExprLiteral(elements.map { it as IntExpr }, ctx.location)
            ClassicalType.Float -> ListExprLiteral(elements.map { it as FloatExpr }, ctx.location)
            ClassicalType.Complex -> ListExprLiteral(elements.map { it as ComplexExpr }, ctx.location)
            ClassicalType.Bits -> ListExprLiteral(elements.map { it as BitsExpr }, ctx.location)
            is ClassicalListType -> ListExprLiteral(elements.map { it as ListExpr<*> }, ctx.location)
            else -> TODO("not implemented yet")
        }
    }

    override fun visitListExprIntGenerator(ctx: ListExprIntGeneratorContext): IntListGenerator {
        return visitClassicalIntListGenerator(ctx.generator)
    }
}
