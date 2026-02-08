package org.stateq.builtin

import org.stateq.module.classical.ExternFloatFunction
import org.stateq.module.classical.ExternIntFunction
import org.stateq.parameter.FloatVariable
import org.stateq.type.ClassicalType
import org.stateq.util.Location

object ClassicalConstants {
    val pi = FloatVariable("pi", Location.builtin)
}

object ClassicalFunctions {
    val sin = ExternFloatFunction("sin", listOf(ClassicalType.Float), Location.builtin)
    val cos = ExternFloatFunction("cos", listOf(ClassicalType.Float), Location.builtin)
    val tan = ExternFloatFunction("tan", listOf(ClassicalType.Float), Location.builtin)
    val exp = ExternFloatFunction("exp", listOf(ClassicalType.Float), Location.builtin)
    val log2 = ExternFloatFunction("log2", listOf(ClassicalType.Float), Location.builtin)
    val log2i = ExternFloatFunction("log2i", listOf(ClassicalType.Int), Location.builtin)
    val ceil = ExternIntFunction("ceil", listOf(ClassicalType.Float), Location.builtin)
    val floor = ExternIntFunction("floor", listOf(ClassicalType.Float), Location.builtin)
    val log = ExternFloatFunction("log", listOf(ClassicalType.Float, ClassicalType.Float), Location.builtin)
    val mpowi = ExternFloatFunction("mpowi", listOf(ClassicalType.Int, ClassicalType.Int, ClassicalType.Int), Location.builtin)
}
