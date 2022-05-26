package com.example.zprize;

import androidx.appcompat.app.AppCompatActivity;

import java.io.File;
import java.io.InputStream;
import java.io.FileOutputStream;
import android.os.Bundle;
import android.text.InputType;
import android.text.TextUtils;
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

        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        LinearLayout linearLayout = findViewById(R.id.rootContainer);

        Button btnShow = new Button(this);
        btnShow.setText("Press to run using random elements");
        btnShow.setLayoutParams(new LinearLayout.LayoutParams(ViewGroup.LayoutParams.WRAP_CONTENT, ViewGroup.LayoutParams.WRAP_CONTENT));

        Button btnShowFile = new Button(this);
        btnShowFile.setText("Press to run from test vector file");
        btnShowFile.setLayoutParams(new LinearLayout.LayoutParams(ViewGroup.LayoutParams.WRAP_CONTENT, ViewGroup.LayoutParams.WRAP_CONTENT));

        EditText iters = new EditText(this);
        iters.setHint("#iterations");
        iters.setInputType(InputType.TYPE_CLASS_NUMBER);

        EditText numElems = new EditText(this);
        numElems.setHint("#elems as power of 2");
        numElems.setInputType(InputType.TYPE_CLASS_NUMBER);

        TextView resultView = new TextView(this);

        btnShow.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                resultView.setText("Mean time to run with random elements is: ");
                RustMSM g = new RustMSM();
                File dir = getFilesDir();
                String dir_path = dir.getAbsolutePath();
                String iters_val = iters.getText().toString();
                String numElemsVal = numElems.getText().toString();
                if (TextUtils.isDigitsOnly(iters_val) && !TextUtils.isEmpty(iters_val)
                && TextUtils.isDigitsOnly(numElemsVal) && !TextUtils.isEmpty(numElemsVal)) {
                    String r = g.runMSMRandom(dir_path, iters_val, numElemsVal);
                    String result = "Mean time to run with random elements is: " + r;
                    resultView.setText(result);
                }
            }
        });

        btnShowFile.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                File filePoints = new File(getFilesDir()+"/points");
                try {

                    InputStream is = getAssets().open("points");
                    int size = is.available();
                    byte[] buffer = new byte[size];
                    is.read(buffer);
                    is.close();

                    FileOutputStream fos = new FileOutputStream(filePoints);
                    fos.write(buffer);
                    fos.close();
                } catch (Exception e) { throw new RuntimeException(e); }
                File fileScalars = new File(getFilesDir()+"/scalars");
                try {

                    InputStream is = getAssets().open("scalars");
                    int size = is.available();
                    byte[] buffer = new byte[size];
                    is.read(buffer);
                    is.close();

                    FileOutputStream fos = new FileOutputStream(fileScalars);
                    fos.write(buffer);
                    fos.close();
                } catch (Exception e) { throw new RuntimeException(e); }

                resultView.setText("Mean time to run with test vectors is: ");
                RustMSM g = new RustMSM();
                File dir = getFilesDir();
                String dir_path = dir.getAbsolutePath();
                String iters_val = iters.getText().toString();
                if (TextUtils.isDigitsOnly(iters_val) && !TextUtils.isEmpty(iters_val)) {
                    String r = g.runMSMFile(dir_path, iters_val);
                    String result = "Mean time to run with test vectors is: " + r;
                    resultView.setText(result);
                }
            }
        });

        // Add Button to LinearLayout
        if (linearLayout != null) {
            linearLayout.addView(btnShowFile);
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