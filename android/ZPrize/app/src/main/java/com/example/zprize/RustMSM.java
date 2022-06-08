package com.example.zprize;

public class RustMSM {
    private static native String benchmarkMSMRandom(final String dir, final String numElems);

    public String runMSMRandom(String dir, String numElems) {
        return benchmarkMSMRandom(dir, numElems);
    }

    private static native String benchmarkMSMRandomMultipleVecs(final String dir, final String numElems, final String numVecs);

    public String runMSMRandomMultipleVecs(String dir, String numElems, String numVecs) {
        return benchmarkMSMRandomMultipleVecs(dir, numElems, numVecs);
    }

    private static native String benchmarkMSMFile(final String dir);

    public String runMSMFile(String dir) {
        return benchmarkMSMFile(dir);
    }
}

