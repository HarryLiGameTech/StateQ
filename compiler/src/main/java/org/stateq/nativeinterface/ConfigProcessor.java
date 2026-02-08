package org.stateq.nativeinterface;

import com.google.common.collect.Multimap;
import com.google.common.collect.TreeMultimap;
import org.graalvm.nativeimage.c.type.CTypeConversion;
import org.stateq.config.Config;
import org.stateq.exception.InvalidCompileOptionException;

import java.util.List;
import java.util.Optional;

public class ConfigProcessor {

    private final Multimap<String, String> keyValueMap = TreeMultimap.create();

    public ConfigProcessor(List<Compiler.KeyValueEntry> nativeEntries) {
        for (Compiler.KeyValueEntry entry : nativeEntries) {
            String key = CTypeConversion.toJavaString(entry.key());
            String value = CTypeConversion.toJavaString(entry.value());
            keyValueMap.put(key, value);
        }
    }

    private String getFirst(String key) {
        var iter = keyValueMap.get(key).iterator();
        if (iter.hasNext()) {
            return iter.next();
        } else {
            throw new InvalidCompileOptionException(key, "null",
                "The compile option \"" + key + "\" is compulsory"
            );
        }
    }

    private Optional<String> tryGetFirst(String key) {
        var iter = keyValueMap.get(key).iterator();
        if (iter.hasNext()) {
            return Optional.of(iter.next());
        } else {
            return Optional.empty();
        }
    }

    public Config process() {
        // TODO
        return null;
    }
}
