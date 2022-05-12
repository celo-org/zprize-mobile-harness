package com.example.zprize;

import androidx.appcompat.app.AppCompatActivity;

import java.io.File;
import android.os.Bundle;
import android.widget.TextView;

public class MainActivity extends AppCompatActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        RustMSM g = new RustMSM();
        String r = g.runMSM("hi");
        File dir = getFilesDir();
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        ((TextView)findViewById(R.id.greetingField)).setText("hello beta");
    }

    static {
        System.loadLibrary("msm");
    }
}