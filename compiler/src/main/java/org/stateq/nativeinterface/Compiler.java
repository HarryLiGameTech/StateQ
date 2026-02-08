package org.stateq.nativeinterface;

import org.graalvm.nativeimage.IsolateThread;
import org.graalvm.nativeimage.c.CContext;
import org.graalvm.nativeimage.c.function.CEntryPoint;
import org.graalvm.nativeimage.c.function.CFunction;
import org.graalvm.nativeimage.c.struct.CField;
import org.graalvm.nativeimage.c.struct.CStruct;
import org.graalvm.nativeimage.c.type.CCharPointer;
import org.graalvm.nativeimage.c.type.CTypeConversion;
import org.graalvm.word.Pointer;
import org.graalvm.word.PointerBase;
import org.jetbrains.annotations.NotNull;
import org.stateq.StateqCompilerKt;
import org.stateq.util.CompileError;
import org.stateq.util.CompileErrorType;

import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import static org.graalvm.nativeimage.c.type.CTypeConversion.CCharPointerHolder;

@SuppressWarnings("unused")
@CContext(Compiler.Headers.class)
public class Compiler {

    static class Headers implements CContext.Directives {
        @Override
        public List<String> getHeaderFiles() {
            return Collections.singletonList("<stateq_compiler.h>");
        }
    }

    @SuppressWarnings("unused")
    @CStruct("TKeyValueEntry")
    interface KeyValueEntry extends PointerBase {
        @CField CCharPointer key();
        @CField CCharPointer value();
    }

    @SuppressWarnings("unused")
    @CStruct("TKeyValueEntryList")
    interface KeyValueEntryList extends PointerBase {
        @CField int size();
        @CField Pointer entries();
    }

    static class KeyValueEntryLists {
        @CFunction(value = "get_entry_from_list", transition = CFunction.Transition.NO_TRANSITION)
        static native KeyValueEntry get(KeyValueEntryList list, int index);

        static Map<String, String> toMap(KeyValueEntryList list) {
            Map<String, String> map = new HashMap<>();
            for (int i = 0; i < list.size(); i++) {
                KeyValueEntry entry = get(list, i);
                map.put(CTypeConversion.toJavaString(entry.key()), CTypeConversion.toJavaString(entry.value()));
            }
            return map;
        }
    }

    @SuppressWarnings("unused")
    @CStruct(value = "TCompileResult", isIncomplete = true)
    interface NativeCompileResult extends PointerBase { }

    static class CompileResults {

        @CFunction(value = "create_compile_result", transition = CFunction.Transition.NO_TRANSITION)
        static native NativeCompileResult create(int targetCount, int errorCount);

        @CFunction(value = "set_compile_error", transition = CFunction.Transition.NO_TRANSITION)
        static native void setCompileError(
            NativeCompileResult compileResult, int index, int type,
            CCharPointer source, int line, int column, CCharPointer message
        );

        @CFunction(value = "set_compile_target", transition = CFunction.Transition.NO_TRANSITION)
        static native void setCompileTarget(
            NativeCompileResult compileResult, int index, CCharPointer path
        );
    }

    static int errTypeId(CompileErrorType errType) {
        return switch (errType) {
            case ERROR -> 0;
            case WARNING -> 1;
            case NOTE -> 2;
        };
    }

    static String cStrToJava(@NotNull CCharPointer cStr) {
        return CTypeConversion.toJavaString(cStr);
    }

    @SuppressWarnings("unused")
    @CEntryPoint(name = "libstateq_compile")
    static NativeCompileResult compile(IsolateThread thread, CCharPointer filePath, KeyValueEntryList config) {
        var compileResult = StateqCompilerKt.compile(cStrToJava(filePath), KeyValueEntryLists.toMap(config));
        var targets = compileResult.getTargets();
        var errors = compileResult.getErrors();
        NativeCompileResult nativeCompileResult = CompileResults.create(targets.size(), errors.size());

        for (int i = 0; i < compileResult.getTargets().size(); i++) {
            String path = compileResult.getTargets().get(i);
            try (CCharPointerHolder targetPath = CTypeConversion.toCString(path)) {
                CompileResults.setCompileTarget(nativeCompileResult, i, targetPath.get());
            }
        }

        for (int i = 0; i < compileResult.getErrors().size(); i++) {
            CompileError err = compileResult.getErrors().get(i);
            String source = err.getPath() == null ? "" : err.getPath().toString();
            String message = err.getMessage();
            try (
                CCharPointerHolder src = CTypeConversion.toCString(source);
                CCharPointerHolder msg = CTypeConversion.toCString(message);
            ) {
                CompileResults.setCompileError(
                    nativeCompileResult, i, errTypeId(err.getType()), src.get(),
                    err.getLine(), err.getColumn(), msg.get()
                );
            }
        }

        return nativeCompileResult;
    }
}
