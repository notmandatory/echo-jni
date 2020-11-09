package org.notmandatory.echojni

class Lib {
    external fun echo(message: String): String

    companion object {
        @JvmStatic
        fun load() {
            System.loadLibrary("echo_jni")
        }
    }

    fun echoMessage(message: String): String {
        return echo(message)
    }
}
