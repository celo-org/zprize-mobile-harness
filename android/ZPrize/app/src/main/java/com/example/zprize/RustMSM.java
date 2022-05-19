package com.example.zprize;

public class RustMSM {
    private static native String benchmarkMSM(final String dir, final String iters, final String numElems);

    public String runMSM(String dir, String iters, String numElems) {
        return benchmarkMSM(dir, iters, numElems);
    }

    private static native String greeting(final String pattern);

    public String sayHello(String to) {
        return greeting(to);
    }
}

