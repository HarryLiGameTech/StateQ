package org.stateq.util;

import java.util.function.Consumer;

public class TestUtils {
    public static void assertNoCompileErrors(String code, Consumer<String> action) {
        TestUtilsKt.assertNoCompileErrors(code, source -> {
            action.accept(source);
            return null;
        });
    }
}
