package com.example.rust_android

import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        findViewById<TextView>(R.id.sample_text).text = hello("山田さん")
    }

    external fun hello(name: String): String

    companion object {
        init {
            System.loadLibrary("helloworld")
        }
    }
}