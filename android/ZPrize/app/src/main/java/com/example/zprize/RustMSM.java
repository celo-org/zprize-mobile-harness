package com.example.zprize;

public class RustMSM {
    private static native String benchmarkMSM(final String dir, final String iters);

    public String runMSM(String dir, String iters) {
        return benchmarkMSM(dir, iters);
    }

    private static native String greeting(final String pattern);

    public String sayHello(String to) {
        return greeting(to);
    }
}

