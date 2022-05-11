package com.example.zprize;

import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.widget.TextView;

public class MainActivity extends AppCompatActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        RustMSM g = new RustMSM();
        String r = g.runMSM("hi");
        int x = 5;
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        ((TextView)findViewById(R.id.greetingField)).setText(String.valueOf(x));
    }

    static {
        System.loadLibrary("msm");
    }
}