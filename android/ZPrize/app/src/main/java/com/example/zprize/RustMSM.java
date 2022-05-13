package com.example.zprize;

public class RustMSM {
    private static native String benchmarkMSM(final String dir);

    public String runMSM(String dir) {
        return benchmarkMSM(dir);
    }

    private static native String greeting(final String pattern);

    public String sayHello(String to) {
        return greeting(to);
    }
}

