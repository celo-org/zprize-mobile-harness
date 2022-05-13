package com.example.zprize;

import androidx.appcompat.app.AppCompatActivity;

import java.io.File;
import android.os.Bundle;
import android.widget.TextView;

public class MainActivity extends AppCompatActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        RustMSM g = new RustMSM();
        File dir = getFilesDir();
        String dir_path = dir.getAbsolutePath();
        String r = g.runMSM(dir_path);

        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        ((TextView)findViewById(R.id.greetingField)).setText("hello charlie");
    }

    static {
        System.loadLibrary("msm");
    }
}