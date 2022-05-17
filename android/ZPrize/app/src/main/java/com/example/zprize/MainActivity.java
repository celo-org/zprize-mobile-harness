package com.example.zprize;

import androidx.appcompat.app.AppCompatActivity;

import java.io.File;
import android.os.Bundle;
import android.text.InputType;
import android.widget.EditText;
import android.widget.TextView;
import android.os.Bundle;
import android.view.View;
import android.view.ViewGroup;
import android.widget.Button;
import android.widget.LinearLayout;
import android.widget.Toast;

public class MainActivity extends AppCompatActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {


       /* super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        ((TextView)findViewById(R.id.greetingField)).setText("hello charlie");*/

        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        LinearLayout linearLayout = findViewById(R.id.rootContainer);

        // Create Button Dynamically
        Button btnShow = new Button(this);
        btnShow.setText("Press to run");
        btnShow.setLayoutParams(new LinearLayout.LayoutParams(ViewGroup.LayoutParams.WRAP_CONTENT, ViewGroup.LayoutParams.WRAP_CONTENT));
        btnShow.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                RustMSM g = new RustMSM();
                File dir = getFilesDir();
                String dir_path = dir.getAbsolutePath();
                String r = g.runMSM(dir_path);
                Toast.makeText(MainActivity.this, "welcome alpha!", Toast.LENGTH_LONG).show();
            }
        });

        EditText iters = new EditText(this);
        iters.setHint("#iterations");
        iters.setInputType(InputType.TYPE_CLASS_NUMBER);

        // Add Button to LinearLayout
        if (linearLayout != null) {
            linearLayout.addView(btnShow);
            linearLayout.addView(iters);
        }

    }

    static {
        System.loadLibrary("msm");
    }
}