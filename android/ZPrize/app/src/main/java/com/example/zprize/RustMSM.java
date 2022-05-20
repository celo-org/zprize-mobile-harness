package com.example.zprize;

public class RustMSM {
    private static native String benchmarkMSMRandom(final String dir, final String iters, final String numElems);

    public String runMSMRandom(String dir, String iters, String numElems) {
        return benchmarkMSMRandom(dir, iters, numElems);
    }

    private static native String benchmarkMSMFile(final String dir, final String iters);

    public String runMSMFile(String dir, String iters) {
        return benchmarkMSMFile(dir, iters);
    }
}

