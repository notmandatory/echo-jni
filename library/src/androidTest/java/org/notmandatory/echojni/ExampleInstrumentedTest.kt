package org.bitcoindevkit.bdkjni

import android.util.Log
import androidx.test.ext.junit.runners.AndroidJUnit4
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.runBlocking
import org.junit.Assert.*
import org.junit.Test
import org.junit.runner.RunWith
import org.notmandatory.echojni.Lib

/**
 * Instrumented test, which will execute on an Android device.
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 *
 */
@RunWith(AndroidJUnit4::class)
class ExampleInstrumentedTest {

    companion object {
        init {
            System.loadLibrary("echo_jni")
        }
    }

    @Test
    fun multiThread() {
        runBlocking {
            val flow1 = newFlow(1).flowOn(Dispatchers.IO)
            val flow2 = newFlow(2).flowOn(Dispatchers.IO)
            flow1.flatMapMerge(concurrency = 2) { flow2 }.collect()
            //flow1.collect()
        }
    }

    private fun newFlow(id: Int): Flow<Pair<Int, String>> {
        return (1..100).asFlow()
            .onStart { Log.d("MT-TEST", "start flow $id") }
            .onCompletion { Log.d("MT-TEST", "complete flow $id") }
            //.map { MT-TESTAndGetBalance(it) }
            .onEach { Log.d("MT-TEST", "flow $id, iteration $it") }
            .map {
                Pair(it, Lib().echo("flow $id, iteration $it"))
            }
            .catch { e ->
                Log.e("MT-TEST", "failed flow $id with exception: $e")
                fail()
            }
            .onEach {
                //assertFalse(it.second == 0L)
                Log.d("MT-TEST", "verifying flow $id, iteration ${it.first}")
                assertFalse(
                    "NullOrEmpty, flow $id, iteration ${it.first}",
                    it.second.isNullOrEmpty()
                )
                assertEquals("flow $id, iteration ${it.first}", it.second)
                Log.d("MT-TEST", "finished flow $id iteration ${it.first}")
            }
    }
}
