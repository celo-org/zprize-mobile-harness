package com.example.zprize;

public class RustMSM {
    private static native String benchmarkMSM(final String pattern);

    public String runMSM(String to) {
        return benchmarkMSM(to);
    }

    private static native String greeting(final String pattern);

    public String sayHello(String to) {
        return greeting(to);
    }
}

