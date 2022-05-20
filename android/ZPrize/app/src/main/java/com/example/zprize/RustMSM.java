package com.example.zprize;

public class RustMSM {
    private static native String benchmarkMSMRandom(final String dir, final String iters, final String numElems);

    public String runMSMRandom(String dir, String iters, String numElems) {
        return benchmarkMSMRandom(dir, iters, numElems);
    }
}

