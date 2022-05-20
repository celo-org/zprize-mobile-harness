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

        EditText iters = new EditText(this);
        iters.setHint("#iterations");
        iters.setInputType(InputType.TYPE_CLASS_NUMBER);

        EditText numElems = new EditText(this);
        numElems.setHint("#elems as power of 2");
        numElems.setInputType(InputType.TYPE_CLASS_NUMBER);

        TextView resultView = new TextView(this);
       // resultView.setText("This is the result");

        btnShow.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                resultView.setText("Mean time to run is: ");
                RustMSM g = new RustMSM();
                File dir = getFilesDir();
                String dir_path = dir.getAbsolutePath();
                String iters_val = iters.getText().toString();
                String numElemsVal = numElems.getText().toString();
                String r = g.runMSM(dir_path, iters_val, numElemsVal);
                String result = "Mean time to run is: " + r;
                resultView.setText(result);
                //Toast.makeText(MainActivity.this, r, Toast.LENGTH_LONG).show();
            }
        });



        // Add Button to LinearLayout
        if (linearLayout != null) {
            linearLayout.addView(btnShow);
            linearLayout.addView(iters);
            linearLayout.addView(numElems);
            linearLayout.addView(resultView);
        }

    }

    static {
        System.loadLibrary("msm");
    }
}